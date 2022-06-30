//! Icinga2 scheduled downtime

crate::types::query::query!(
    ListScheduledDowntimes,
    ListScheduledDowntimesBuilder,
    monitoring_objects,
    scheduled_downtime,
    IcingaScheduledDowntime,
    IcingaObjectType::ScheduledDowntime,
    "v1/objects/scheduleddowntimes"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_scheduled_downtimes() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListScheduledDowntimes::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaScheduledDowntime>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
