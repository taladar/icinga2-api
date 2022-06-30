//! Icinga2 service groups

crate::types::query::query!(
    ListServiceGroups,
    ListServiceGroupsBuilder,
    monitoring_objects,
    service_group,
    IcingaServiceGroup,
    IcingaObjectType::ServiceGroup,
    "v1/objects/servicegroups"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_service_groups() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListServiceGroups::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaServiceGroup>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
