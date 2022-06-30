//! Icinga2 check command

crate::types::query::query!(
    ListCheckCommands,
    ListCheckCommandsBuilder,
    monitoring_objects,
    check_command,
    IcingaCheckCommand,
    IcingaObjectType::CheckCommand,
    "v1/objects/checkcommands"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_check_commands() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListCheckCommands::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaCheckCommand>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
