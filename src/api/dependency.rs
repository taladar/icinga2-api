//! Icinga2 dependency

use serde::Deserialize;

use crate::enums::IcingaHostOrServiceState;
use crate::serde::deserialize_empty_string_or_parse;

use super::custom_var_object::IcingaCustomVarObject;
use super::host::IcingaHostName;
use super::joins::IcingaJoinResult;
use super::metadata::IcingaMetadata;
use super::service::IcingaServiceName;
use super::{
    host::IcingaHostAttributes, service::IcingaServiceAttributes, IcingaJoinType, IcingaObjectType,
};

/// attributes on an [IcingaDependency]
#[derive(Debug, Deserialize)]
pub struct IcingaDependencyAttributes {
    /// type of icinga object, should always be Dependency for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// the child host name
    pub child_host_name: IcingaHostName,
    /// the child service name
    #[serde(deserialize_with = "deserialize_empty_string_or_parse")]
    pub child_service_name: Option<IcingaServiceName>,
    /// the parent host name
    pub parent_host_name: IcingaHostName,
    /// the parent service name
    #[serde(deserialize_with = "deserialize_empty_string_or_parse")]
    pub parent_service_name: Option<IcingaServiceName>,
    /// whether checks are disabled by this dependency
    pub disable_checks: bool,
    /// whether notifications are disabled by this dependency
    pub disable_notifications: bool,
    /// whether this dependency ignores soft states
    pub ignore_soft_states: bool,
    /// the name of the period when this dependency is active
    pub period: String,
    /// states when this dependency is enabled
    pub states: Vec<IcingaHostOrServiceState>,
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
            None,
        )?;
        Ok(())
    }
}
