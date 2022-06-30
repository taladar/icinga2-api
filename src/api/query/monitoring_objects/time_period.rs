//! Icinga2 time periods

crate::types::query::query!(
    ListTimePeriods,
    ListTimePeriodsBuilder,
    monitoring_objects,
    time_period,
    IcingaTimePeriod,
    IcingaObjectType::TimePeriod,
    "v1/objects/timeperiods"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_time_periods() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListTimePeriods::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaTimePeriod>> = icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
