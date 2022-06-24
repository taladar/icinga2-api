//! Icinga2 check command
use std::collections::BTreeMap;

use serde::Deserialize;

use crate::enums::HAMode;
use crate::serde::{
    deserialize_empty_string_or_string, deserialize_optional_icinga_timestamp,
    deserialize_optional_seconds_as_duration,
};

use super::command::{IcingaCommand, IcingaCommandArgumentDescription};
use super::metadata::IcingaMetadata;
use super::{IcingaFunction, IcingaObjectType, IcingaSourceLocation, IcingaVariableValue};

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

#[cfg(test)]
mod test {
    use crate::api::{metadata::IcingaMetadataType, Icinga2};
    use std::error::Error;
    use tracing_test::traced_test;

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
