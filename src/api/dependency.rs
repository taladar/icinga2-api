//! Icinga2 dependency
use std::collections::BTreeMap;

use serde::Deserialize;

use crate::enums::{HAMode, IcingaHostOrServiceState};
use crate::serde::{deserialize_empty_string_or_string, deserialize_optional_icinga_timestamp};

use super::joins::IcingaJoinResult;
use super::metadata::IcingaMetadata;
use super::{
    host::IcingaHostAttributes, service::IcingaServiceAttributes, IcingaJoinType, IcingaObjectType,
    IcingaSourceLocation, IcingaVariableValue,
};

/// attributes on an [IcingaDependency]
#[derive(Debug, Deserialize)]
pub struct IcingaDependencyAttributes {
    /// full object name
    #[serde(rename = "__name")]
    pub full_name: String,
    /// service name (without host)
    pub name: String,
    /// type of icinga object, should always be Service for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// whether this dependency is active
    pub active: bool,
    /// the child host name
    pub child_host_name: String,
    /// the child service name
    pub child_service_name: String,
    /// the parent host name
    pub parent_host_name: String,
    /// the parent service name
    pub parent_service_name: String,
    /// whether checks are disabled by this dependency
    pub disable_checks: bool,
    /// whether notifications are disabled by this dependency
    pub disable_notifications: bool,
    /// whether to run a check once or everywhere
    pub ha_mode: HAMode,
    /// whether this dependency ignores soft states
    pub ignore_soft_states: bool,
    /// original values of object attributes modified at runtime
    pub original_attributes: Option<()>,
    /// configuration package name this object belongs to, _etc for local configuration
    /// _api for runtime created objects
    pub package: String,
    /// object has been paused at runtime
    pub paused: bool,
    /// the name of the period when this dependency is active
    pub period: String,
    /// states when this dependency is enabled
    pub states: Vec<IcingaHostOrServiceState>,
    /// location information whether the configuration files are stored
    pub source_location: IcingaSourceLocation,
    /// templates imported on object compilation
    pub templates: Vec<String>,
    /// custom variables specific to this host
    pub vars: Option<BTreeMap<String, IcingaVariableValue>>,
    /// timestamp when the object was created or modified. syncred throughout cluster nodes
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub version: Option<time::OffsetDateTime>,
    /// the zone this object is a member of
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub zone: Option<String>,
}

/// the result of an icinga dependencies query
#[derive(Debug, Deserialize)]
pub struct IcingaDependency {
    /// dependency attributes
    pub attrs: IcingaDependencyAttributes,
    /// joins
    pub joins: IcingaDependencyJoins,
    /// metadata, only contains data if the parameter meta contains one or more values
    pub meta: IcingaMetadata,
    /// object name
    pub name: String,
    /// type of icinga object, should always be Dependency for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
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

/// return type joins for dependencies
#[derive(Debug, Deserialize)]
pub struct IcingaDependencyJoins {
    /// the child host of the dependency
    pub child_host: Option<IcingaJoinResult<IcingaHostAttributes>>,
    /// the child service of the dependency
    pub child_service: Option<IcingaJoinResult<IcingaServiceAttributes>>,
    /// the parent host of the dependency
    pub parent_host: Option<IcingaJoinResult<IcingaHostAttributes>>,
    /// the parent service of the dependency
    pub parent_service: Option<IcingaJoinResult<IcingaServiceAttributes>>,
    //pub period: Option<IcingaJoinResult<IcingaPeriodAttributes>>,
}

#[cfg(test)]
mod test {
    use crate::api::{joins::IcingaJoins, metadata::IcingaMetadataType, Icinga2};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_dependencies() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.dependencies(
            IcingaJoins::AllJoins,
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
        )?;
        Ok(())
    }
}
