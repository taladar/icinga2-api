//! Icinga2 downtime

crate::types::query::query!(
    ListDowntimes,
    ListDowntimesBuilder,
    runtime_objects,
    downtime,
    IcingaDowntime,
    IcingaObjectType::Downtime,
    "v1/objects/downtimes"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_downtimes() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListDowntimes::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaDowntime>> = icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
