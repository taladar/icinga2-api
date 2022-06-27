//! Icinga2 services

use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::{enums::IcingaObjectType, serde::deserialize_optional_icinga_timestamp};

use super::{
    check_command::IcingaCheckCommandAttributes,
    checkable::IcingaCheckable,
    host::{IcingaHostAttributes, IcingaHostName},
    joins::{IcingaJoinResult, IcingaJoinType},
    metadata::IcingaMetadata,
    service_group::IcingaServiceGroupName,
};

/// a service name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaServiceName(pub String);

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
    /// type of icinga object, should always be Service for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// all the attributes from the icinga checkable object (shared fields between host and service)
    #[serde(flatten)]
    pub checkable: IcingaCheckable,
    /// a short description of the service
    pub display_name: String,
    /// a list of groups the service belongs to
    pub groups: Vec<IcingaServiceGroupName>,
    /// the hostname for this service
    pub host_name: IcingaHostName,
    /// the previous hard state
    pub last_hard_state: IcingaServiceState,
    /// the previous state
    pub last_state: IcingaServiceState,
    /// when the last CRITICAL state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_critical: Option<time::OffsetDateTime>,
    /// when the last OK state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_ok: Option<time::OffsetDateTime>,
    /// when the last UNKNOWN state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_unknown: Option<time::OffsetDateTime>,
    /// when the last WARNINGE state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_warning: Option<time::OffsetDateTime>,
    /// the current state
    pub state: IcingaServiceState,
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
    use crate::api::{
        filter::IcingaFilter, joins::IcingaJoins, metadata::IcingaMetadataType, Icinga2,
    };
    use std::{collections::BTreeMap, error::Error};
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
            None,
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
            None,
        )?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_services_filtered() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.services(
            IcingaJoins::NoJoins,
            &[],
            Some(IcingaFilter {
                object_type: IcingaObjectType::Service,
                filter: "service.state == ServiceUnknown && service.vars.serviceSeverity == filter_severity".to_string(),
                filter_vars: BTreeMap::from([("filter_severity".to_string(), serde_json::json!("imminent"))]),
            }),
        )?;
        Ok(())
    }
}
