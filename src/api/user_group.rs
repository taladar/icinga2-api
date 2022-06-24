//! Icinga2 user groups

use serde::Deserialize;

use crate::enums::IcingaObjectType;

use super::{custom_var_object::IcingaCustomVarObject, metadata::IcingaMetadata};

/// a user group name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaUserGroupName(pub String);

/// a user group
#[derive(Debug, Deserialize)]
pub struct IcingaUserGroupAttributes {
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// a short description of the user group
    pub display_name: String,
    /// a list of groups the user group belongs to
    pub groups: Option<Vec<IcingaUserGroupName>>,
}

/// the result of an icinga user group query
#[derive(Debug, Deserialize)]
pub struct IcingaUserGroup {
    /// attributes
    pub attrs: IcingaUserGroupAttributes,
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
    fn test_user_groups() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.user_groups(&[IcingaMetadataType::UsedBy, IcingaMetadataType::Location])?;
        Ok(())
    }
}
