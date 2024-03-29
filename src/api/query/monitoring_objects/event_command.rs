//! Icinga2 event command

crate::types::query::query!(
    ListEventCommands,
    ListEventCommandsBuilder,
    monitoring_objects,
    event_command,
    IcingaEventCommand,
    IcingaObjectType::EventCommand,
    "v1/objects/eventcommands"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_event_commands() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListEventCommands::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaEventCommand>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
