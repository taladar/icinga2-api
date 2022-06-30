//! Icinga2 host groups

crate::types::query::query!(
    ListHostGroups,
    ListHostGroupsBuilder,
    monitoring_objects,
    host_group,
    IcingaHostGroup,
    IcingaObjectType::HostGroup,
    "v1/objects/hostgroups"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_host_groups() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListHostGroups::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaHostGroup>> = icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
