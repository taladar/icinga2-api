#![deny(unknown_lints)]
#![deny(renamed_and_removed_lints)]
#![forbid(unsafe_code)]
#![deny(deprecated)]
#![forbid(private_in_public)]
#![forbid(non_fmt_panics)]
#![deny(unreachable_code)]
#![deny(unreachable_patterns)]
#![forbid(unused_doc_comments)]
#![forbid(unused_must_use)]
#![deny(while_true)]
#![deny(unused_parens)]
#![deny(redundant_semicolons)]
#![deny(non_ascii_idents)]
#![deny(confusable_idents)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::cargo_common_metadata)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_debug_implementations)]
#![deny(clippy::mod_module_files)]
//#![warn(clippy::pedantic)]
#![warn(clippy::redundant_else)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::panic)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![doc = include_str!("../README.md")]

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::from_utf8,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use thiserror::Error;

/// Error type for icinga2_api
#[derive(Debug, Error)]
pub enum Error {
    /// could not read config file
    #[error("could not read config file: {0}")]
    CouldNotReadConfigFile(std::io::Error),
    /// could not parse config
    #[error("could not parse config: {0}")]
    CouldNotParseConfig(toml::de::Error),
    /// could not parse URL in config
    #[error("could not parse URL in config: {0}")]
    CouldNotParseUrlInConfig(url::ParseError),
    /// could not build reqwest client from supplied information
    #[error("could not build reqwest client from supplied information: {0}")]
    CouldNotBuildReqwestClientFromSuppliedInformation(reqwest::Error),
    /// could not read CA certificate file
    #[error("could not read CA certificate file: {0}")]
    CouldNotReadCACertFile(std::io::Error),
    /// could not parse PEM CA certificate
    #[error("could not parse PEM CA certificate: {0}")]
    CouldNotParsePEMCACertificate(reqwest::Error),
    /// An error occurred when serializing/deserializing JSON
    #[error("error in json serialization/deserialization: {0}")]
    SerdeJsonError(#[from] serde_path_to_error::Error<serde_json::Error>),
    /// Response body was empty so we can not deserialize it as JSON
    #[error("empty response body with status: {0}")]
    EmptyResponseBody(reqwest::StatusCode),
    /// An error occurred in the reqwest library (HTTP)
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    /// could not parse URL fragment
    #[error("could not parse URL fragment: {0}")]
    CouldNotParseUrlFragment(url::ParseError),
}

/// this represents the configuration for an Icinga instance we connect to
#[derive(Debug, Deserialize)]
pub struct Icinga2Instance {
    /// the URL to connect to, without the v1 component or anything after that
    pub url: String,
    /// the CA certificate to use to validate the server certificate
    pub ca_certificate: Option<PathBuf>,
    /// username
    pub username: String,
    /// password
    pub password: String,
}

/// the runtime object for an Icinga2 instance
#[derive(Debug)]
pub struct Icinga2 {
    /// the HTTP client to use
    client: reqwest::blocking::Client,
    /// the base URL for the Icinga API
    url: url::Url,
    /// username
    pub username: String,
    /// password
    pub password: String,
}

impl Icinga2 {
    /// create a new Icinga2 instance from a TOML config file
    ///
    /// # Errors
    /// this fails if the configuration file can not be found or parsed
    /// or the CA certificate file mentioned in the configuration file
    /// can not be found or parsed
    pub fn from_config_file(path: &Path) -> Result<Self, crate::Error> {
        let content =
            std::fs::read_to_string(path).map_err(crate::Error::CouldNotReadConfigFile)?;
        let config: Icinga2Instance =
            toml::from_str(&content).map_err(crate::Error::CouldNotParseConfig)?;
        let client_builder = reqwest::blocking::ClientBuilder::new();
        let client_builder = client_builder.user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ));
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let client_builder = client_builder.default_headers(headers);
        let client_builder = if let Some(ca_certificate) = &config.ca_certificate {
            let ca_cert_content =
                std::fs::read(ca_certificate).map_err(crate::Error::CouldNotReadCACertFile)?;
            let ca_cert = reqwest::Certificate::from_pem(&ca_cert_content)
                .map_err(crate::Error::CouldNotParsePEMCACertificate)?;
            let client_builder = client_builder.add_root_certificate(ca_cert);
            client_builder.tls_built_in_root_certs(false)
        } else {
            client_builder
        };
        let client = client_builder
            .build()
            .map_err(crate::Error::CouldNotBuildReqwestClientFromSuppliedInformation)?;
        let url = url::Url::parse(&config.url).map_err(crate::Error::CouldNotParseUrlInConfig)?;
        let username = config.username;
        let password = config.password;
        Ok(Icinga2 {
            client,
            url,
            username,
            password,
        })
    }

    /// common code for the API calls
    fn rest<Req, Res>(
        &self,
        method: http::Method,
        url: url::Url,
        request_body: Option<Req>,
    ) -> Result<Res, crate::Error>
    where
        Req: Serialize + std::fmt::Debug,
        Res: DeserializeOwned + std::fmt::Debug,
    {
        let actual_method = if method == http::Method::GET && request_body.is_some() {
            http::Method::POST
        } else {
            method.to_owned()
        };
        let mut req = self.client.request(actual_method, url.to_owned());
        if method == http::Method::GET && request_body.is_some() {
            tracing::trace!("Sending GET request with body as POST via X-HTTP-Method-Override");
            req = req.header(
                "X-HTTP-Method-Override",
                http::header::HeaderValue::from_static("GET"),
            );
        }
        req = req.basic_auth(&self.username, Some(&self.password));
        if let Some(request_body) = request_body {
            tracing::trace!("Request body:\n{:#?}", request_body);
            req = req.json(&request_body);
        }
        let result = req.send();
        if let Err(ref e) = result {
            tracing::error!(%url, %method, "Icinga2 send error: {:?}", e);
        }
        let result = result?;
        let status = result.status();
        let response_body = result.bytes()?;
        match from_utf8(&response_body) {
            Ok(response_body) => {
                tracing::trace!("Response body:\n{}", &response_body);
            }
            Err(e) => {
                tracing::trace!(
                    "Response body that could not be parsed as utf8 because of {}:\n{:?}",
                    &e,
                    &response_body
                );
            }
        }
        if status.is_client_error() {
            tracing::error!(%url, %method, "Icinga2 status error (client error): {:?}", status);
        } else if status.is_server_error() {
            tracing::error!(%url, %method, "Icinga2 status error (server error): {:?}", status);
        }
        if response_body.is_empty() {
            Err(crate::Error::EmptyResponseBody(status))
        } else {
            let jd = &mut serde_json::Deserializer::from_slice(&response_body);
            match serde_path_to_error::deserialize(jd) {
                Ok(response_body) => {
                    tracing::trace!("Parsed response body:\n{:#?}", response_body);
                    Ok(response_body)
                }
                Err(e) => {
                    let path = e.path();
                    tracing::error!("Parsing failed at path {}: {}", path.to_string(), e.inner());
                    if let Ok(response_body) = serde_json::from_slice(&response_body) {
                        let mut response_body: serde_json::Value = response_body;
                        for segment in path {
                            match (response_body, segment) {
                                (
                                    serde_json::Value::Array(vs),
                                    serde_path_to_error::Segment::Seq { index },
                                ) => {
                                    if let Some(v) = vs.get(*index) {
                                        response_body = v.to_owned();
                                    } else {
                                        // if we can not find the element serde_path_to_error references fall back to just returning the error
                                        return Err(e.into());
                                    }
                                }
                                (
                                    serde_json::Value::Object(m),
                                    serde_path_to_error::Segment::Map { key },
                                ) => {
                                    if let Some(v) = m.get(key) {
                                        response_body = v.to_owned();
                                    } else {
                                        // if we can not find the element serde_path_to_error references fall back to just returning the error
                                        return Err(e.into());
                                    }
                                }
                                _ => {
                                    // if we can not find the element serde_path_to_error references fall back to just returning the error
                                    return Err(e.into());
                                }
                            }
                        }
                        tracing::error!("Value in location path references is: {}", response_body);
                    }
                    Err(e.into())
                }
            }
        }
    }

    /// shared code for all the handlers that have meta and joins parameters
    /// to add those to the URL
    fn handle_joins_and_meta<JT: IcingaJoinType + Ord + std::fmt::Display>(
        &self,
        url: &mut url::Url,
        joins: &IcingaJoins<JT>,
        meta: &[IcingaMetadataType],
    ) -> Result<(), crate::Error> {
        match joins {
            IcingaJoins::NoJoins => (),
            IcingaJoins::AllJoins => {
                url.query_pairs_mut().append_pair("all_joins", "1");
            }
            IcingaJoins::SpecificJoins { full, partial } => {
                for j in full {
                    url.query_pairs_mut().append_pair("joins", &j.to_string());
                }
                for (j, fields) in partial {
                    for f in fields {
                        url.query_pairs_mut()
                            .append_pair("joins", &format!("{}.{}", &j.to_string(), &f));
                    }
                }
            }
        }
        if !meta.is_empty() {
            for v in meta {
                url.query_pairs_mut().append_pair("meta", &v.to_string());
            }
        }
        Ok(())
    }

    /// retrieve Icinga hosts
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    pub fn hosts(
        &self,
        joins: IcingaJoins<IcingaHostJoinTypes>,
        meta: &[IcingaMetadataType],
    ) -> Result<Vec<IcingaHost>, crate::Error> {
        let mut url = self
            .url
            .join("v1/objects/hosts")
            .map_err(crate::Error::CouldNotParseUrlFragment)?;
        self.handle_joins_and_meta(&mut url, &joins, meta)?;
        let ResultsWrapper { results } =
            self.rest::<(), ResultsWrapper<IcingaHost>>(http::Method::GET, url, None)?;
        Ok(results)
    }

    /// retrieve Icinga services
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    pub fn services(
        &self,
        joins: IcingaJoins<IcingaServiceJoinTypes>,
        meta: &[IcingaMetadataType],
    ) -> Result<Vec<IcingaService>, crate::Error> {
        let mut url = self
            .url
            .join("v1/objects/services")
            .map_err(crate::Error::CouldNotParseUrlFragment)?;
        self.handle_joins_and_meta(&mut url, &joins, meta)?;
        let ResultsWrapper { results } =
            self.rest::<(), ResultsWrapper<IcingaService>>(http::Method::GET, url, None)?;
        Ok(results)
    }

    /// retrieve Icinga check commands
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    pub fn check_commands(
        &self,
        meta: &[IcingaMetadataType],
    ) -> Result<Vec<IcingaCheckCommand>, crate::Error> {
        let mut url = self
            .url
            .join("v1/objects/checkcommands")
            .map_err(crate::Error::CouldNotParseUrlFragment)?;
        if !meta.is_empty() {
            for v in meta {
                url.query_pairs_mut().append_pair("meta", &v.to_string());
            }
        }
        let ResultsWrapper { results } =
            self.rest::<(), ResultsWrapper<IcingaCheckCommand>>(http::Method::GET, url, None)?;
        Ok(results)
    }
}

/// wrapper for Json results
#[derive(Debug, Deserialize)]
pub struct ResultsWrapper<T> {
    /// the internal field in the Icinga2 object containing all an array of the actual results
    results: Vec<T>,
}

/// the type of icinga object we are dealing with
#[derive(Debug, Deserialize)]
pub enum IcingaObjectType {
    /// an icinga monitored host
    Host,
    /// an icinga service
    Service,
    /// an icinga check result
    CheckResult,
    /// a performance data value
    PerfdataValue,
    /// an icinga comment
    Comment,
    /// an icinga dependency between hosts or services
    Dependency,
    /// an icinga notification
    Notification,
    /// a function
    Function,
    /// a check command
    CheckCommand,
}

/// deserializes a unix timestamp with sub second accuracy
/// (usually 6 digits after the decimal point for icinga)
///
/// # Errors
///
/// returns an error if the value can not be parsed as an f64
/// or if it can not be converted from a unix timestamp to a
/// [time::OffsetDateTime]
pub fn deserialize_icinga_timestamp<'de, D>(
    deserializer: D,
) -> Result<time::OffsetDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f: f64 = f64::deserialize(deserializer)?;

    let i = (f * 1_000_000_000f64) as i128;

    time::OffsetDateTime::from_unix_timestamp_nanos(i).map_err(serde::de::Error::custom)
}

/// deserializes an optional unix timestamp with sub second accuracy
/// (usually 6 digits after the decimal point for icinga)
/// if the value is 0 return None
///
/// # Errors
///
/// returns an error if the value can not be parsed as an f64
/// or if it can not be converted from a unix timestamp to a
/// [time::OffsetDateTime]
pub fn deserialize_optional_icinga_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<time::OffsetDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f: f64 = f64::deserialize(deserializer)?;

    if f == 0.0f64 {
        Ok(None)
    } else {
        let i = (f * 1_000_000_000f64) as i128;

        Ok(Some(
            time::OffsetDateTime::from_unix_timestamp_nanos(i).map_err(serde::de::Error::custom)?,
        ))
    }
}

/// deserialize an optional Ipv6Addr where None is represented as
/// an empty string
///
/// # Errors
///
/// returns an error if the value can not be interpreted as a null or String
/// or if parsing it to an [std::net::Ipv6Addr] fails
pub fn deserialize_empty_string_or_ipv6_address<'de, D>(
    deserializer: D,
) -> Result<Option<std::net::Ipv6Addr>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    if let Some(s) = s {
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s.parse().map_err(serde::de::Error::custom)?))
        }
    } else {
        Ok(None)
    }
}

/// deserialize an optional String where None is represented as
/// an empty string
///
/// # Errors
///
/// returns an error if the value can not be interpreted as null or a String
pub fn deserialize_empty_string_or_string<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    if let Some(s) = s {
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    } else {
        Ok(None)
    }
}

/// deserialize an integer as a time::Duration where the integer represents seconds
///
/// # Errors
///
/// returns an error if the value can not be parsed as an integer
pub fn deserialize_seconds_as_duration<'de, D>(deserializer: D) -> Result<time::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let i: i64 = i64::deserialize(deserializer)?;
    Ok(time::Duration::seconds(i))
}

/// deserialize an integer as a time::Duration where the integer represents seconds
///
/// # Errors
///
/// returns an error if the value can not be interpreted as null or an integer
pub fn deserialize_optional_seconds_as_duration<'de, D>(
    deserializer: D,
) -> Result<Option<time::Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let i: Option<i64> = Option::deserialize(deserializer)?;
    if let Some(i) = i {
        Ok(Some(time::Duration::seconds(i)))
    } else {
        Ok(None)
    }
}

/// which state type we are dealing with
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum IcingaStateType {
    /// soft state (recently changed)
    Soft = 0,
    /// hard state (no recent changes)
    Hard = 1,
}

/// host state
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum IcingaHostState {
    /// host is up
    Up = 0,
    /// host is down
    Down = 1,
    /// host is unreachable
    Unreachable = 2,
}

/// variables in check result (seem to be very static)
#[derive(Debug, Deserialize)]
pub struct IcingaHostCheckResultVars {
    /// used for internal calculations
    pub attempt: u64,
    /// used for internal calculations
    pub reachable: bool,
    /// used for internal calculations
    pub state: IcingaHostState,
    /// used for internal calculations
    pub state_type: IcingaStateType,
}

/// represents performance data
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaPerformanceData {
    /// performance data in string format
    String(String),
    /// structured performance data value
    PerfDataValue {
        /// object type, should always be PerfdataValue here
        #[serde(rename = "type")]
        object_type: IcingaObjectType,
        /// is this a counter
        counter: bool,
        /// the current value
        value: f64,
        /// the critical value
        crit: Option<f64>,
        /// the warning value
        warn: Option<f64>,
        /// the label for the type of values
        #[serde(deserialize_with = "deserialize_empty_string_or_string")]
        label: Option<String>,
        /// the minimum value
        min: Option<f64>,
        /// the maximum value
        max: Option<f64>,
        /// the unit for the type of values
        #[serde(deserialize_with = "deserialize_empty_string_or_string")]
        unit: Option<String>,
    },
}

/// a host check result
#[derive(Debug, Deserialize)]
pub struct IcingaHostCheckResult {
    /// was this an active check
    pub active: bool,
    /// name of host which provided this check result
    pub check_source: String,
    /// the command called for the check
    pub command: Option<IcingaCommand>,
    /// start of command execution
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub execution_start: time::OffsetDateTime,
    /// end of command execution
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub execution_end: time::OffsetDateTime,
    /// exit status of the check command
    pub exit_status: u64,
    /// output of the check command
    pub output: String,
    /// performance data provided by the check command
    pub performance_data: Option<Vec<IcingaPerformanceData>>,
    /// hard state before this check
    pub previous_hard_state: IcingaHostState,
    /// scheduled check execution start time
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub schedule_start: time::OffsetDateTime,
    /// scheduled check execution end time
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub schedule_end: time::OffsetDateTime,
    /// name of host which did the scheduling
    pub scheduling_source: String,
    /// state returned by this check
    pub state: IcingaHostState,
    /// the TTL of this check result
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub ttl: Option<time::Duration>,
    /// the type of icinga object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// variables for internal calculations before this check
    pub vars_before: Option<IcingaHostCheckResultVars>,
    /// variables for internal calculations after this check
    pub vars_after: Option<IcingaHostCheckResultVars>,
}

/// an icinga source location inside the icinga config files
#[derive(Debug, Deserialize)]
pub struct IcingaSourceLocation {
    /// path of the config file
    pub path: String,
    /// start line
    pub first_line: u64,
    /// start column
    pub first_column: u64,
    /// end line
    pub last_line: u64,
    /// end column
    pub last_column: u64,
}

/// an icinga variable value
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaVariableValue {
    /// string value
    String(String),
    /// list of strings value
    List(Vec<String>),
    /// key/value object
    Object(BTreeMap<String, IcingaVariableValue>),
    /// Boolean
    Boolean(bool),
    /// Integer
    Integer(i64),
}

/// acknowledgement type
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum IcingaAcknowledgementType {
    /// no acknowledgement
    None = 0,
    /// normal acknowledgement
    Normal = 1,
    /// sticky acknowledgement
    Sticky = 2,
}

/// HA mode
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum HAMode {
    /// run a check once
    Once,
    /// run a check everywhere
    Everywhere,
}

/// command parameters (scalar values basically)
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaCommandParameter {
    /// string value
    String(String),
    /// Boolean
    Boolean(bool),
    /// Integer
    Integer(i64),
}

/// command to execute with parameters
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaCommand {
    /// a single string for the whole command, will likely need a shell to do
    /// word splitting
    Shell(String),
    /// individual command and parameters
    Exec(Vec<IcingaCommandParameter>),
}

/// attributes on an [IcingaHost]
#[derive(Debug, Deserialize)]
pub struct IcingaHostAttributes {
    /// full object name
    #[serde(rename = "__name")]
    pub full_name: String,
    /// host name
    pub name: String,
    /// type of icinga object, should always be Host for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// the type of acknowledgement (includes None)
    pub acknowledgement: IcingaAcknowledgementType,
    /// when the acknowledgement expires
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub acknowledgement_expiry: Option<time::OffsetDateTime>,
    /// when the acknowledgement last changed
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub acknowledgement_last_change: Option<time::OffsetDateTime>,
    /// URL for actions for the host
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub action_url: Option<String>,
    /// object is active (being checked)
    pub active: bool,
    /// host Ipv4 address
    pub address: std::net::Ipv4Addr,
    /// optional host Ipv6 address
    #[serde(deserialize_with = "deserialize_empty_string_or_ipv6_address")]
    pub address6: Option<std::net::Ipv6Addr>,
    /// the current check attempt number
    pub check_attempt: u64,
    /// the name of the check command
    pub check_command: String,
    /// the interval used for checks when the host is in a HARD state
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub check_interval: Option<time::Duration>,
    /// name of a time period when this host is checked
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub check_period: Option<String>,
    /// check timeout
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub check_timeout: Option<time::Duration>,
    /// the endpoint the command is executed on
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub command_endpoint: Option<String>,
    /// a short description of the host
    pub display_name: String,
    /// number of active downtimes on the host
    pub downtime_depth: u64,
    /// whether active checks are enabled
    pub enable_active_checks: bool,
    /// enabled event handlers for this host
    pub enable_event_handler: bool,
    /// whether flap detection is enabled
    pub enable_flapping: bool,
    /// whether notifications are enabled
    pub enable_notifications: bool,
    /// whether passive checks are enabled
    pub enable_passive_checks: bool,
    /// whether performance data processing is enabled
    pub enable_perfdata: bool,
    /// the name of an event command that should be executed every time the host state changes or the host is in a SOFT state
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub event_command: Option<String>,
    /// contains the state of execute-command executions
    pub executions: Option<()>,
    /// whether the host is flapping between states
    pub flapping: bool,
    /// current flapping value in percent
    pub flapping_current: f64,
    /// a list of states that should be ignored during flapping calculations
    #[serde(default)]
    pub flapping_ignore_states: Option<Vec<IcingaHostState>>,
    /// when the last flapping change occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub flapping_last_change: Option<time::OffsetDateTime>,
    /// deprecated and has no effect, replaced by flapping_threshold_low and flapping_threshold_high
    pub flapping_threshold: f64,
    /// the flapping lower bound in percent for a host to be considered flapping
    pub flapping_threshold_low: f64,
    /// the flapping upper bound in percent for a host to be considered flapping
    pub flapping_threshold_high: f64,
    /// force the next check (execute it now)
    pub force_next_check: bool,
    /// force next notification (send it now)
    pub force_next_notification: bool,
    /// a list of groups the host belongs to
    pub groups: Vec<String>,
    /// whether to run a check once or everywhere
    pub ha_mode: HAMode,
    /// whether the host problem is handled (downtime or acknowledgement)
    pub handled: bool,
    /// icon image for the host
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub icon_image: Option<String>,
    /// icon image alt text for the host
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub icon_image_alt: Option<String>,
    /// when the last check occurred
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub last_check: time::OffsetDateTime,
    /// the result of the last check
    pub last_check_result: IcingaHostCheckResult,
    /// the previous hard state
    pub last_hard_state: IcingaHostState,
    /// when the last hard state change occurred
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub last_hard_state_change: time::OffsetDateTime,
    /// whether the host was reachable when the last check occurred
    pub last_reachable: bool,
    /// the previous state
    pub last_state: IcingaHostState,
    /// when the last state change occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_change: Option<time::OffsetDateTime>,
    /// when the last DOWN state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_down: Option<time::OffsetDateTime>,
    /// the previous state type (soft/hard)
    pub last_state_type: IcingaStateType,
    /// when the last UNREACHABLE state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_unreachable: Option<time::OffsetDateTime>,
    /// when the last UP state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_up: Option<time::OffsetDateTime>,
    /// the number of times the host is checked before changing into a new hard state
    pub max_check_attempts: u64,
    /// when the next check occurs
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub next_check: Option<time::OffsetDateTime>,
    /// when the next check update is to be expected
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub next_update: Option<time::OffsetDateTime>,
    /// notes for the host
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub notes: Option<String>,
    /// URL for notes for the host
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub notes_url: Option<String>,
    /// original values of object attributes modified at runtime
    pub original_attributes: Option<()>,
    /// configuration package name this object belongs to, _etc for local configuration
    /// _api for runtime created objects
    pub package: String,
    /// object has been paused at runtime
    pub paused: bool,
    /// when the previous state change occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub previous_state_change: Option<time::OffsetDateTime>,
    /// whether the host is considered to be in a problem state type (not up)
    pub problem: bool,
    /// the interval used for checks when the host is in a SOFT state
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub retry_interval: Option<time::Duration>,
    /// pre-calculated value, higher means more severe
    pub severity: u64,
    /// location information whether the configuration files are stored
    pub source_location: IcingaSourceLocation,
    /// the current state
    pub state: IcingaHostState,
    /// the current state type (soft/hard)
    pub state_type: IcingaStateType,
    /// templates imported on object compilation
    pub templates: Vec<String>,
    /// custom variables specific to this host
    pub vars: BTreeMap<String, IcingaVariableValue>,
    /// timestamp when the object was created or modified. syncred throughout cluster nodes
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub version: Option<time::OffsetDateTime>,
    /// treat all state changes as HARD changes
    pub volatile: bool,
    /// the zone this object is a member of
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub zone: Option<String>,
}

/// the result of an icinga hosts query
#[derive(Debug, Deserialize)]
pub struct IcingaHost {
    /// host attributes
    pub attrs: IcingaHostAttributes,
    /// joins
    pub joins: IcingaHostJoins,
    /// metadata, only contains data if the parameter meta contains one or more values
    pub meta: IcingaMetadata,
    /// object name
    pub name: String,
    /// type of icinga object, should always be Host for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}

/// service state
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum IcingaServiceState {
    /// service is OK
    Ok = 0,
    /// service is WARNING
    Warning = 1,
    /// service is CRITICAL
    Critical = 2,
    /// service is UNKNOWN
    Unknown = 3,
    /// service is UNREACHABLE
    Unreachable = 4,
    /// service is PENDING
    Pending = 99,
}

/// variables in check result (seem to be very static)
#[derive(Debug, Deserialize)]
pub struct IcingaServiceCheckResultVars {
    /// used for internal calculations
    pub attempt: u64,
    /// used for internal calculations
    pub reachable: bool,
    /// used for internal calculations
    pub state: IcingaServiceState,
    /// used for internal calculations
    pub state_type: IcingaStateType,
}

/// a service check result
#[derive(Debug, Deserialize)]
pub struct IcingaServiceCheckResult {
    /// was this an active check
    pub active: bool,
    /// name of host which provided this check result
    pub check_source: String,
    /// the command called for the check
    pub command: Option<IcingaCommand>,
    /// start of command execution
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub execution_start: time::OffsetDateTime,
    /// end of command execution
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub execution_end: time::OffsetDateTime,
    /// exit status of the check command
    pub exit_status: u64,
    /// output of the check command
    pub output: String,
    /// performance data provided by the check command
    pub performance_data: Option<Vec<IcingaPerformanceData>>,
    /// hard state before this check
    pub previous_hard_state: IcingaServiceState,
    /// scheduled check execution start time
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub schedule_start: time::OffsetDateTime,
    /// scheduled check execution end time
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub schedule_end: time::OffsetDateTime,
    /// name of host which did the scheduling
    pub scheduling_source: String,
    /// state returned by this check
    pub state: IcingaServiceState,
    /// the TTL of this check result
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub ttl: Option<time::Duration>,
    /// the type of icinga object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// variables for internal calculations before this check
    pub vars_before: Option<IcingaServiceCheckResultVars>,
    /// variables for internal calculations after this check
    pub vars_after: Option<IcingaServiceCheckResultVars>,
}

/// attributes on an [IcingaService]
#[derive(Debug, Deserialize)]
pub struct IcingaServiceAttributes {
    /// full object name
    #[serde(rename = "__name")]
    pub full_name: String,
    /// service name (without host)
    pub name: String,
    /// type of icinga object, should always be Service for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// the type of acknowledgement (includes None)
    pub acknowledgement: IcingaAcknowledgementType,
    /// when the acknowledgement expires
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub acknowledgement_expiry: Option<time::OffsetDateTime>,
    /// when the acknowledgement last changed
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub acknowledgement_last_change: Option<time::OffsetDateTime>,
    /// URL for actions for the host
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub action_url: Option<String>,
    /// object is active (being checked)
    pub active: bool,
    /// the current check attempt number
    pub check_attempt: u64,
    /// the name of the check command
    pub check_command: String,
    /// the interval used for checks when the service is in a HARD state
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub check_interval: Option<time::Duration>,
    /// name of a time period when this service is checked
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub check_period: Option<String>,
    /// check timeout
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub check_timeout: Option<time::Duration>,
    /// the endpoint the command is executed on
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub command_endpoint: Option<String>,
    /// a short description of the host
    pub display_name: String,
    /// number of active downtimes on the host
    pub downtime_depth: u64,
    /// whether active checks are enabled
    pub enable_active_checks: bool,
    /// enabled event handlers for this host
    pub enable_event_handler: bool,
    /// whether flap detection is enabled
    pub enable_flapping: bool,
    /// whether notifications are enabled
    pub enable_notifications: bool,
    /// whether passive checks are enabled
    pub enable_passive_checks: bool,
    /// whether performance data processing is enabled
    pub enable_perfdata: bool,
    /// the name of an event command that should be executed every time the service state changes or the service is in a SOFT state
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub event_command: Option<String>,
    /// contains the state of execute-command executions
    pub executions: Option<()>,
    /// whether the host is flapping between states
    pub flapping: bool,
    /// current flapping value in percent
    pub flapping_current: f64,
    /// a list of states that should be ignored during flapping calculations
    #[serde(default)]
    pub flapping_ignore_states: Option<Vec<IcingaHostState>>,
    /// when the last flapping change occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub flapping_last_change: Option<time::OffsetDateTime>,
    /// deprecated and has no effect, replaced by flapping_threshold_low and flapping_threshold_high
    pub flapping_threshold: f64,
    /// the flapping lower bound in percent for a host to be considered flapping
    pub flapping_threshold_low: f64,
    /// the flapping upper bound in percent for a host to be considered flapping
    pub flapping_threshold_high: f64,
    /// force the next check (execute it now)
    pub force_next_check: bool,
    /// force next notification (send it now)
    pub force_next_notification: bool,
    /// a list of groups the host belongs to
    pub groups: Vec<String>,
    /// whether to run a check once or everywhere
    pub ha_mode: HAMode,
    /// whether the host problem is handled (downtime or acknowledgement)
    pub handled: bool,
    /// the hostname for this service
    pub host_name: String,
    /// icon image for the service
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub icon_image: Option<String>,
    /// icon image alt text for the service
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub icon_image_alt: Option<String>,
    /// when the last check occurred
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub last_check: time::OffsetDateTime,
    /// the result of the last check
    pub last_check_result: IcingaServiceCheckResult,
    /// the previous hard state
    pub last_hard_state: IcingaServiceState,
    /// when the last hard state change occurred
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub last_hard_state_change: time::OffsetDateTime,
    /// whether the host was reachable when the last check occurred
    pub last_reachable: bool,
    /// the previous state
    pub last_state: IcingaServiceState,
    /// when the last state change occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_change: Option<time::OffsetDateTime>,
    /// when the last CRITICAL state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_critical: Option<time::OffsetDateTime>,
    /// when the last OK state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_ok: Option<time::OffsetDateTime>,
    /// the previous state type (soft/hard)
    pub last_state_type: IcingaStateType,
    /// when the last UNKNOWN state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_unknown: Option<time::OffsetDateTime>,
    /// when the last UNREACHABLE state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_unreachable: Option<time::OffsetDateTime>,
    /// when the last WARNINGE state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_warning: Option<time::OffsetDateTime>,
    /// the number of times the host is checked before changing into a new hard state
    pub max_check_attempts: u64,
    /// when the next check occurs
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub next_check: Option<time::OffsetDateTime>,
    /// when the next check update is to be expected
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub next_update: Option<time::OffsetDateTime>,
    /// notes for the service
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub notes: Option<String>,
    /// URL for notes for the service
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub notes_url: Option<String>,
    /// original values of object attributes modified at runtime
    pub original_attributes: Option<()>,
    /// configuration package name this object belongs to, _etc for local configuration
    /// _api for runtime created objects
    pub package: String,
    /// object has been paused at runtime
    pub paused: bool,
    /// when the previous state change occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub previous_state_change: Option<time::OffsetDateTime>,
    /// whether the service is considered to be in a problem state type (not OK)
    pub problem: bool,
    /// the interval used for checks when the service is in a SOFT state
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub retry_interval: Option<time::Duration>,
    /// pre-calculated value, higher means more severe
    pub severity: u64,
    /// location information whether the configuration files are stored
    pub source_location: IcingaSourceLocation,
    /// the current state
    pub state: IcingaServiceState,
    /// the current state type (soft/hard)
    pub state_type: IcingaStateType,
    /// templates imported on object compilation
    pub templates: Vec<String>,
    /// custom variables specific to this host
    pub vars: BTreeMap<String, IcingaVariableValue>,
    /// timestamp when the object was created or modified. syncred throughout cluster nodes
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub version: Option<time::OffsetDateTime>,
    /// treat all state changes as HARD changes
    pub volatile: bool,
    /// the zone this object is a member of
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub zone: Option<String>,
}

/// the result of an icinga services query
#[derive(Debug, Deserialize)]
pub struct IcingaService {
    /// service attributes
    pub attrs: IcingaServiceAttributes,
    /// joins
    pub joins: IcingaServiceJoins,
    /// metadata, only contains data if the parameter meta contains one or more values
    pub meta: IcingaMetadata,
    /// object name
    pub name: String,
    /// type of icinga object, should always be Host for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}

/// the most minimal description of an icinga object
#[derive(Debug, Deserialize)]
pub struct IcingaObject {
    /// the name of the object
    pub name: String,
    /// the type of the object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}

/// a marker trait for all the various join types for the different objects
pub trait IcingaJoinType {}

/// possible joins parameter values for services
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaServiceJoinTypes {
    /// the host the service is on
    Host,
    /// the check command object for the service
    CheckCommand,
    /// the check period object for the service
    CheckPeriod,
    /// the event command object for the service
    EventCommand,
    /// the command endpoint object for the service
    CommandEndpoint,
}

impl IcingaJoinType for IcingaServiceJoinTypes {}

impl std::fmt::Display for IcingaServiceJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaServiceJoinTypes::Host => write!(f, "host"),
            IcingaServiceJoinTypes::CheckCommand => write!(f, "check_command"),
            IcingaServiceJoinTypes::CheckPeriod => write!(f, "check_period"),
            IcingaServiceJoinTypes::EventCommand => write!(f, "event_command"),
            IcingaServiceJoinTypes::CommandEndpoint => write!(f, "command_endpoint"),
        }
    }
}

/// return type for joins, either full or partial
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaJoinResult<T> {
    /// a full result we get if we just specified e.g. joins=host
    Full(T),
    /// a partial result we get if we specified individual fields, e.g. joins=host.name
    Partial(BTreeMap<String, serde_json::Value>),
}

/// return type joins for services
#[derive(Debug, Deserialize)]
pub struct IcingaServiceJoins {
    /// the host this service is on
    pub host: Option<IcingaJoinResult<IcingaHostAttributes>>,
    /// the check command object for the service
    pub check_command: Option<IcingaJoinResult<IcingaCheckCommandAttributes>>,
    //pub check_period: Option<IcingaJoinResult<IcingaCheckPeriod>>,
    //pub event_command: Option<IcingaJoinResult<IcingaEventCommand>>,
    //pub command_endpoint: Option<IcingaJoinResult<IcingaCommandEndpoint>>,
}

/// possible joins parameter values for hosts
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaHostJoinTypes {
    /// the check command object for the host
    CheckCommand,
    /// the check period object for the host
    CheckPeriod,
    /// the event command object for the host
    EventCommand,
    /// the command endpoint object for the host
    CommandEndpoint,
}

impl IcingaJoinType for IcingaHostJoinTypes {}

impl std::fmt::Display for IcingaHostJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaHostJoinTypes::CheckCommand => write!(f, "check_command"),
            IcingaHostJoinTypes::CheckPeriod => write!(f, "check_period"),
            IcingaHostJoinTypes::EventCommand => write!(f, "event_command"),
            IcingaHostJoinTypes::CommandEndpoint => write!(f, "command_endpoint"),
        }
    }
}

/// return type joins for hosts
#[derive(Debug, Deserialize)]
pub struct IcingaHostJoins {
    /// the check command object for the host
    pub check_command: Option<IcingaJoinResult<IcingaCheckCommand>>,
    //pub check_period: Option<IcingaJoinResult<IcingaCheckPeriod>>,
    //pub event_command: Option<IcingaJoinResult<IcingaEventCommand>>,
    //pub command_endpoint: Option<IcingaJoinResult<IcingaCommandEndpoint>>,
}

/// possible joins parameter values for notifications
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaNotificationJoinTypes {
    /// the host the notification is about
    Host,
    /// the service the notification is about
    Service,
    /// the notification command object for the notification
    Command,
    /// the notification period object for the notification
    Period,
}

impl IcingaJoinType for IcingaNotificationJoinTypes {}

impl std::fmt::Display for IcingaNotificationJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaNotificationJoinTypes::Host => write!(f, "host"),
            IcingaNotificationJoinTypes::Service => write!(f, "service"),
            IcingaNotificationJoinTypes::Command => write!(f, "command"),
            IcingaNotificationJoinTypes::Period => write!(f, "period"),
        }
    }
}

/// possible joins parameter values for dependencies
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaDependencyJoinTypes {
    /// the child host of the dependency
    ChildHost,
    /// the child service of the dependency
    ChildService,
    /// the parent host of the dependency
    ParentHost,
    /// the parent service of the dependency
    ParentService,
    /// the period object for which the dependency is valid
    Period,
}

impl IcingaJoinType for IcingaDependencyJoinTypes {}

impl std::fmt::Display for IcingaDependencyJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaDependencyJoinTypes::ChildHost => write!(f, "child_host"),
            IcingaDependencyJoinTypes::ChildService => write!(f, "child_service"),
            IcingaDependencyJoinTypes::ParentHost => write!(f, "parent_host"),
            IcingaDependencyJoinTypes::ParentService => write!(f, "parent_service"),
            IcingaDependencyJoinTypes::Period => write!(f, "period"),
        }
    }
}

/// possible joins parameter values for users
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaUserJoinTypes {
    /// the period object for which the user is valid (most likely something like shift or work hours)
    Period,
}

impl IcingaJoinType for IcingaUserJoinTypes {}

impl std::fmt::Display for IcingaUserJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaUserJoinTypes::Period => write!(f, "period"),
        }
    }
}

/// possible joins parameter values for zones
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaZoneJoinTypes {
    /// the parent zone object
    Parent,
}

impl IcingaJoinType for IcingaZoneJoinTypes {}

impl std::fmt::Display for IcingaZoneJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaZoneJoinTypes::Parent => write!(f, "parent"),
        }
    }
}

/// joins
#[derive(Debug)]
pub enum IcingaJoins<'a, JT>
where
    JT: IcingaJoinType + Ord + std::fmt::Display,
{
    /// do not include any joins
    NoJoins,
    /// include specific joins
    SpecificJoins {
        /// include the full objects for these joins
        full: Vec<JT>,
        /// include just the specified fields for these joins
        partial: BTreeMap<JT, Vec<&'a str>>,
    },
    /// include full objects for all possible joins
    AllJoins,
}

/// possible meta parameter values
#[derive(Debug)]
pub enum IcingaMetadataType {
    /// includes information about the other icinga objects using each returned object
    UsedBy,
    /// includes information about the config file location of each returned object
    Location,
}

impl std::fmt::Display for IcingaMetadataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaMetadataType::UsedBy => write!(f, "used_by"),
            IcingaMetadataType::Location => write!(f, "location"),
        }
    }
}

/// metadata
#[derive(Debug, Deserialize)]
pub struct IcingaMetadata {
    /// which other icinga objects use this object
    pub used_by: Option<Vec<IcingaObject>>,
    /// where in the config file this object is defined
    pub location: Option<IcingaSourceLocation>,
}

/// set_if condition in command argument description
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaArgumentCondition {
    /// a string condition, most likely a boolean variable
    String(String),
    /// a function condition
    Function(IcingaFunction),
}

/// the description of a single
#[derive(Debug, Deserialize)]
pub struct IcingaCommandArgumentDescription {
    /// the description of this argument
    pub description: Option<String>,
    /// the default value for this argument
    pub value: Option<String>,
    /// name of an argument to set
    pub key: Option<String>,
    /// should the key be repeated
    pub repeat_key: Option<bool>,
    /// condition when to set it
    pub set_if: Option<IcingaArgumentCondition>,
    /// is this argument required
    pub required: Option<bool>,
}

/// the description of an icinga function
#[derive(Debug, Deserialize)]
pub struct IcingaFunction {
    /// the arguments
    pub arguments: Vec<String>,
    /// is this deprecated
    pub deprecated: bool,
    /// the name
    pub name: String,
    /// is this command side-effect free
    pub side_effect_free: bool,
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}

/// a check command (e.g. in a join)
#[derive(Debug, Deserialize)]
pub struct IcingaCheckCommandAttributes {
    /// the name of the check command as deserialized from __name
    #[serde(rename = "__name")]
    pub full_name: String,
    /// the name of the check command as deserialized from name
    pub name: String,
    /// is this check command active
    pub active: bool,
    /// the descriptions of the command arguments
    pub arguments: Option<BTreeMap<String, IcingaCommandArgumentDescription>>,
    /// the actual command
    pub command: Option<IcingaCommand>,
    /// environment variables
    pub env: Option<BTreeMap<String, String>>,
    /// function for execution
    pub execute: IcingaFunction,
    /// whether to run a check once or everywhere
    pub ha_mode: HAMode,
    /// original values of object attributes modified at runtime
    pub original_attributes: Option<()>,
    /// configuration package name this object belongs to, _etc for local configuration
    /// _api for runtime created objects
    pub package: String,
    /// object has been paused at runtime
    pub paused: bool,
    /// location information whether the configuration files are stored
    pub source_location: IcingaSourceLocation,
    /// templates imported on object compilation
    pub templates: Vec<String>,
    /// command timeout
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub timeout: Option<time::Duration>,
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// custom variables specific to this host
    pub vars: Option<BTreeMap<String, IcingaVariableValue>>,
    /// timestamp when the object was created or modified. syncred throughout cluster nodes
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub version: Option<time::OffsetDateTime>,
    /// the zone this object is a member of
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub zone: Option<String>,
}

/// the result of an icinga check commands query
#[derive(Debug, Deserialize)]
pub struct IcingaCheckCommand {
    /// host attributes
    pub attrs: IcingaCheckCommandAttributes,
    /// metadata, only contains data if the parameter meta contains one or more values
    pub meta: IcingaMetadata,
    /// object name
    pub name: String,
    /// type of icinga object, should always be CheckCommand for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}

// TODO: filters https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#advanced-filters (operations, functions,.. below are just a selection of the most immediately interesting ones)
// * what are the semantics of a variable that does not exist (e.g. typo, field access to custom variables)
// * what are the semantics of a type mismatch (e.g. you apply string functions to a custom variable or field that is an array)
// * boolean literals
// * numeric literals (floating point numbers and integers are one type in icinga)
// * string literals (do filters support multi-line string literals?)
// * enum literals (service and host state and state type in particular)
// * duration literals
// * null literal
// * dictionary literals
// * array literals
// * operators ( https://icinga.com/docs/icinga-2/latest/doc/17-language-reference/#operators )
// ** () grouping
// ** function call
// ** element access (can we somehow get validation of field names here? Would require us to know the type of a variable but there is only a handful of those, could not be for all fields though since some are runtime, e.g. custom variables)
// ** logical not
// ** unary minus
// ** multiplication
// ** division
// ** remainder
// ** add numbers/durations
// ** concatenate string
// ** subtract numbers/durations
// ** equality
// ** inequality
// ** logical and
// ** logical or
// ** element in array
// ** element not in array
// ** less than, greater than, less than or equal, greater than or equal for numbers, durations (and strings?)
// * variables provided by the filter (varies by object type we query, some types of variables appear under different names for different queries)
// * functions ( https://icinga.com/docs/icinga-2/latest/doc/18-library-reference/ )
// ** match
// ** regex
// ** intersection
// ** union
// ** range
// ** get_time
// ** Math.min
// ** Math.max
// ** Array.all
// ** Array.any
// ** Array.contains
// ** Dictionary.contains
// ** Dictionary.keys
// ** Dictionary.values
// ** String.contains
// ** String.split
// ** String.trim
// ** String.lower
// ** String.upper

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_hosts() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.hosts(
            IcingaJoins::AllJoins,
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
        )?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_services() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.services(
            IcingaJoins::AllJoins,
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
        )?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_services_partial_host_join() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let mut partial = BTreeMap::new();
        partial.insert(IcingaServiceJoinTypes::Host, vec!["name"]);
        icinga2.services(
            IcingaJoins::SpecificJoins {
                full: vec![],
                partial,
            },
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
        )?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_check_commands() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.check_commands(&[IcingaMetadataType::UsedBy, IcingaMetadataType::Location])?;
        Ok(())
    }
}
