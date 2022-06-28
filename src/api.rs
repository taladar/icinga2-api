//! The main API object and some small structs that appear in more than one query result

use std::{path::Path, str::from_utf8};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::enums::IcingaObjectType;

use self::{
    check_command::IcingaCheckCommand,
    dependency::{IcingaDependency, IcingaDependencyJoinTypes},
    event_command::IcingaEventCommand,
    filter::IcingaFilter,
    host::{IcingaHost, IcingaHostJoinTypes},
    host_group::IcingaHostGroup,
    joins::{IcingaJoinType, IcingaJoins},
    metadata::IcingaMetadataType,
    notification_command::IcingaNotificationCommand,
    service::{IcingaService, IcingaServiceJoinTypes},
    service_group::IcingaServiceGroup,
    user_group::IcingaUserGroup,
};

// monitoring objects
pub mod api_user;
pub mod check_command;
pub mod dependency;
pub mod endpoint;
pub mod event_command;
pub mod host;
pub mod host_group;
pub mod notification;
pub mod notification_command;
pub mod scheduled_downtime;
pub mod service;
pub mod service_group;
pub mod time_period;
pub mod user;
pub mod user_group;
pub mod zone;

// runtime objects
pub mod comment;
pub mod downtime;

// other types
pub mod check_result;
pub mod checkable;
pub mod command;
pub mod config_object;
pub mod custom_var_object;
pub mod joins;
pub mod metadata;
pub mod performance_data;

// filter language
pub mod filter;

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
    pub fn from_config_file(path: &Path) -> Result<Self, crate::error::Error> {
        let content =
            std::fs::read_to_string(path).map_err(crate::error::Error::CouldNotReadConfigFile)?;
        let config: crate::config::Icinga2Instance =
            toml::from_str(&content).map_err(crate::error::Error::CouldNotParseConfig)?;
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
            let ca_cert_content = std::fs::read(ca_certificate)
                .map_err(crate::error::Error::CouldNotReadCACertFile)?;
            let ca_cert = reqwest::Certificate::from_pem(&ca_cert_content)
                .map_err(crate::error::Error::CouldNotParsePEMCACertificate)?;
            let client_builder = client_builder.add_root_certificate(ca_cert);
            client_builder.tls_built_in_root_certs(false)
        } else {
            client_builder
        };
        let client = client_builder
            .build()
            .map_err(crate::error::Error::CouldNotBuildReqwestClientFromSuppliedInformation)?;
        let url =
            url::Url::parse(&config.url).map_err(crate::error::Error::CouldNotParseUrlInConfig)?;
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
    ) -> Result<Res, crate::error::Error>
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
            Err(crate::error::Error::EmptyResponseBody(status))
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

    /// retrieve Icinga hosts
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    ///
    /// # Panics
    ///
    /// this panics if the object_type field in the filter is not IcingaObjectType::Host
    pub fn hosts(
        &self,
        joins: IcingaJoins<IcingaHostJoinTypes>,
        meta: &[IcingaMetadataType],
        filter: Option<IcingaFilter>,
    ) -> Result<Vec<IcingaHost>, crate::error::Error> {
        if let Some(filter) = &filter {
            assert_eq!(filter.object_type, IcingaObjectType::Host);
        }
        let mut url = self
            .url
            .join("v1/objects/hosts")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
        add_joins_to_url(&mut url, &joins)?;
        add_meta_to_url(&mut url, meta)?;
        let ResultsWrapper { results } =
            self.rest::<IcingaFilter, ResultsWrapper<IcingaHost>>(http::Method::GET, url, filter)?;
        Ok(results)
    }

    /// retrieve Icinga services
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    ///
    /// # Panics
    ///
    /// this panics if the object_type field in the filter is not IcingaObjectType::Service
    pub fn services(
        &self,
        joins: IcingaJoins<IcingaServiceJoinTypes>,
        meta: &[IcingaMetadataType],
        filter: Option<IcingaFilter>,
    ) -> Result<Vec<IcingaService>, crate::error::Error> {
        if let Some(filter) = &filter {
            assert_eq!(filter.object_type, IcingaObjectType::Service);
        }
        let mut url = self
            .url
            .join("v1/objects/services")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
        add_joins_to_url(&mut url, &joins)?;
        add_meta_to_url(&mut url, meta)?;
        let ResultsWrapper { results } = self.rest::<IcingaFilter, ResultsWrapper<IcingaService>>(
            http::Method::GET,
            url,
            filter,
        )?;
        Ok(results)
    }

    /// retrieve Icinga check commands
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    ///
    /// # Panics
    ///
    /// this panics if the object_type field in the filter is not IcingaObjectType::CheckCommand
    pub fn check_commands(
        &self,
        meta: &[IcingaMetadataType],
        filter: Option<IcingaFilter>,
    ) -> Result<Vec<IcingaCheckCommand>, crate::error::Error> {
        if let Some(filter) = &filter {
            assert_eq!(filter.object_type, IcingaObjectType::CheckCommand);
        }
        let mut url = self
            .url
            .join("v1/objects/checkcommands")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
        add_meta_to_url(&mut url, meta)?;
        let ResultsWrapper { results } = self
            .rest::<IcingaFilter, ResultsWrapper<IcingaCheckCommand>>(
                http::Method::GET,
                url,
                filter,
            )?;
        Ok(results)
    }

    /// retrieve Icinga notification commands
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    ///
    /// # Panics
    ///
    /// this panics if the object_type field in the filter is not IcingaObjectType::NotificationCommand
    pub fn notification_commands(
        &self,
        meta: &[IcingaMetadataType],
        filter: Option<IcingaFilter>,
    ) -> Result<Vec<IcingaNotificationCommand>, crate::error::Error> {
        if let Some(filter) = &filter {
            assert_eq!(filter.object_type, IcingaObjectType::NotificationCommand);
        }
        let mut url = self
            .url
            .join("v1/objects/notificationcommands")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
        add_meta_to_url(&mut url, meta)?;
        let ResultsWrapper { results } = self
            .rest::<IcingaFilter, ResultsWrapper<IcingaNotificationCommand>>(
                http::Method::GET,
                url,
                filter,
            )?;
        Ok(results)
    }

    /// retrieve Icinga event commands
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    ///
    /// # Panics
    ///
    /// this panics if the object_type field in the filter is not IcingaObjectType::EventCommand
    pub fn event_commands(
        &self,
        meta: &[IcingaMetadataType],
        filter: Option<IcingaFilter>,
    ) -> Result<Vec<IcingaEventCommand>, crate::error::Error> {
        if let Some(filter) = &filter {
            assert_eq!(filter.object_type, IcingaObjectType::EventCommand);
        }
        let mut url = self
            .url
            .join("v1/objects/eventcommands")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
        add_meta_to_url(&mut url, meta)?;
        let ResultsWrapper { results } = self
            .rest::<IcingaFilter, ResultsWrapper<IcingaEventCommand>>(
                http::Method::GET,
                url,
                filter,
            )?;
        Ok(results)
    }

    /// retrieve Icinga host groups
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    ///
    /// # Panics
    ///
    /// this panics if the object_type field in the filter is not IcingaObjectType::HostGroup
    pub fn host_groups(
        &self,
        meta: &[IcingaMetadataType],
        filter: Option<IcingaFilter>,
    ) -> Result<Vec<IcingaHostGroup>, crate::error::Error> {
        if let Some(filter) = &filter {
            assert_eq!(filter.object_type, IcingaObjectType::HostGroup);
        }
        let mut url = self
            .url
            .join("v1/objects/hostgroups")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
        add_meta_to_url(&mut url, meta)?;
        let ResultsWrapper { results } = self
            .rest::<IcingaFilter, ResultsWrapper<IcingaHostGroup>>(
                http::Method::GET,
                url,
                filter,
            )?;
        Ok(results)
    }

    /// retrieve Icinga service groups
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    ///
    /// # Panics
    ///
    /// this panics if the object_type field in the filter is not IcingaObjectType::ServiceGroup
    pub fn service_groups(
        &self,
        meta: &[IcingaMetadataType],
        filter: Option<IcingaFilter>,
    ) -> Result<Vec<IcingaServiceGroup>, crate::error::Error> {
        if let Some(filter) = &filter {
            assert_eq!(filter.object_type, IcingaObjectType::ServiceGroup);
        }
        let mut url = self
            .url
            .join("v1/objects/servicegroups")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
        add_meta_to_url(&mut url, meta)?;
        let ResultsWrapper { results } = self
            .rest::<IcingaFilter, ResultsWrapper<IcingaServiceGroup>>(
                http::Method::GET,
                url,
                filter,
            )?;
        Ok(results)
    }

    /// retrieve Icinga user groups
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    ///
    /// # Panics
    ///
    /// this panics if the object_type field in the filter is not IcingaObjectType::UserGroup
    pub fn user_groups(
        &self,
        meta: &[IcingaMetadataType],
        filter: Option<IcingaFilter>,
    ) -> Result<Vec<IcingaUserGroup>, crate::error::Error> {
        if let Some(filter) = &filter {
            assert_eq!(filter.object_type, IcingaObjectType::UserGroup);
        }
        let mut url = self
            .url
            .join("v1/objects/usergroups")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
        add_meta_to_url(&mut url, meta)?;
        let ResultsWrapper { results } = self
            .rest::<IcingaFilter, ResultsWrapper<IcingaUserGroup>>(
                http::Method::GET,
                url,
                filter,
            )?;
        Ok(results)
    }

    /// retrieve Icinga dependencies
    ///
    /// # Errors
    ///
    /// fails if the icinga2 API could not be reached, won't accept our authentication information or if the response can not be decoded
    ///
    /// # Panics
    ///
    /// this panics if the object_type field in the filter is not IcingaObjectType::Dependency
    pub fn dependencies(
        &self,
        joins: IcingaJoins<IcingaDependencyJoinTypes>,
        meta: &[IcingaMetadataType],
        filter: Option<IcingaFilter>,
    ) -> Result<Vec<IcingaDependency>, crate::error::Error> {
        if let Some(filter) = &filter {
            assert_eq!(filter.object_type, IcingaObjectType::Dependency);
        }
        let mut url = self
            .url
            .join("v1/objects/dependencies")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
        add_joins_to_url(&mut url, &joins)?;
        add_meta_to_url(&mut url, meta)?;
        let ResultsWrapper { results } = self
            .rest::<IcingaFilter, ResultsWrapper<IcingaDependency>>(
                http::Method::GET,
                url,
                filter,
            )?;
        Ok(results)
    }
}

/// shared code for all the handlers that have a joins parameters
pub(crate) fn add_joins_to_url<JT: IcingaJoinType + Ord + std::fmt::Display>(
    url: &mut url::Url,
    joins: &IcingaJoins<JT>,
) -> Result<(), crate::error::Error> {
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
    Ok(())
}

/// shared code for all handlers that have a meta parameter
pub(crate) fn add_meta_to_url(
    url: &mut url::Url,
    meta: &[IcingaMetadataType],
) -> Result<(), crate::error::Error> {
    if !meta.is_empty() {
        for v in meta {
            url.query_pairs_mut().append_pair("meta", &v.to_string());
        }
    }
    Ok(())
}

/// wrapper for Json results
#[derive(Debug, Deserialize)]
pub struct ResultsWrapper<T> {
    /// the internal field in the Icinga2 object containing all an array of the actual results
    results: Vec<T>,
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
