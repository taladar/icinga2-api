//! Icinga2 notifications

crate::types::query::query_with_joins!(
    ListNotifications,
    ListNotificationsBuilder,
    monitoring_objects,
    notification,
    IcingaNotification,
    IcingaNotificationJoinTypes,
    IcingaNotificationJoins,
    IcingaObjectType::Notification,
    "v1/objects/notifications"
);

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;
    use tracing_test::traced_test;

    use crate::{
        api::blocking::Icinga2,
        types::{join_types::IcingaJoins, metadata::IcingaMetadataType},
    };

    #[traced_test]
    #[test]
    fn test_notifications() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListNotifications::builder()
            .joins(IcingaJoins::AllJoins)
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<
            QueryResultObjectWithJoins<IcingaNotification, IcingaNotificationJoins>,
        > = icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
