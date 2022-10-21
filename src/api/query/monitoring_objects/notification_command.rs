//! Icinga2 notification commands

crate::types::query::query!(
    ListNotificationCommands,
    ListNotificationCommandsBuilder,
    monitoring_objects,
    notification_command,
    IcingaNotificationCommand,
    IcingaObjectType::NotificationCommand,
    "v1/objects/notificationcommands"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_notification_commands() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListNotificationCommands::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaNotificationCommand>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
