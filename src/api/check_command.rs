//! Icinga2 check command

use serde::Deserialize;

use super::command::IcingaCommand;
use super::metadata::IcingaMetadata;
use super::IcingaObjectType;

/// a check command name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaCheckCommandName(pub String);

/// a check command (e.g. in a join)
#[derive(Debug, Deserialize)]
pub struct IcingaCheckCommandAttributes {
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared fields in all command types
    #[serde(flatten)]
    pub command: IcingaCommand,
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
