//! Icinga2 services
use std::collections::BTreeMap;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::{
    enums::{HAMode, IcingaAcknowledgementType, IcingaObjectType, IcingaStateType},
    serde::{
        deserialize_empty_string_or_string, deserialize_icinga_timestamp,
        deserialize_optional_icinga_timestamp, deserialize_optional_seconds_as_duration,
    },
};

use super::{
    check_command::IcingaCheckCommandAttributes,
    check_result::IcingaCheckResult,
    host::IcingaHostAttributes,
    joins::{IcingaJoinResult, IcingaJoinType},
    metadata::IcingaMetadata,
    IcingaSourceLocation, IcingaVariableValue,
};

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

/// service state helper to deserialize by name
#[derive(Debug, Deserialize)]
#[repr(u8)]
#[serde(remote = "IcingaServiceState")]
pub enum IcingaServiceStateByName {
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
    pub flapping_ignore_states: Option<Vec<IcingaServiceState>>,
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
    pub last_check_result: IcingaCheckResult,
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

/// return type joins for services
#[derive(Debug, Deserialize)]
pub struct IcingaServiceJoins {
    /// the host this service is on
    pub host: Option<IcingaJoinResult<IcingaHostAttributes>>,
    /// the check command object for the service
    pub check_command: Option<IcingaJoinResult<IcingaCheckCommandAttributes>>,
    //pub check_period: Option<IcingaJoinResult<IcingaPeriodAttributes>>,
    //pub event_command: Option<IcingaJoinResult<IcingaEventCommand>>,
    //pub command_endpoint: Option<IcingaJoinResult<IcingaCommandEndpoint>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::{joins::IcingaJoins, metadata::IcingaMetadataType, Icinga2};
    use std::error::Error;
    use tracing_test::traced_test;

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
}
