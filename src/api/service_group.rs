//! Icinga2 service groups

use serde::Deserialize;

use crate::enums::IcingaObjectType;
use crate::serde::deserialize_empty_string_or_string;

use super::custom_var_object::IcingaCustomVarObject;
use super::metadata::IcingaMetadata;

/// a service group
#[derive(Debug, Deserialize)]
pub struct IcingaServiceGroupAttributes {
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// a short description of the service group
    pub display_name: String,
    /// a list of groups the service group belongs to
    pub groups: Option<Vec<IcingaServiceGroupName>>,
    /// URL for actions for the checkable (host or service)
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub action_url: Option<String>,
    /// notes for the host/service
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub notes: Option<String>,
    /// URL for notes for the host/service
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub notes_url: Option<String>,
}

/// the result of an icinga service group query
#[derive(Debug, Deserialize)]
pub struct IcingaServiceGroup {
    /// attributes
    pub attrs: IcingaServiceGroupAttributes,
    /// metadata, only contains data if the parameter meta contains one or more values
    pub meta: IcingaMetadata,
    /// object name
    pub name: String,
    /// type of icinga object, should always be ServiceGroup for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}

#[cfg(test)]
mod test {
    use crate::api::{metadata::IcingaMetadataType, Icinga2};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_service_groups() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.service_groups(
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
            None,
        )?;
        Ok(())
    }
}
