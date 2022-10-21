//! Icinga2 endpoint

crate::types::query::query!(
    ListEndpoints,
    ListEndpointsBuilder,
    monitoring_objects,
    endpoint,
    IcingaEndpoint,
    IcingaObjectType::Endpoint,
    "v1/objects/endpoints"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_endpoints() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListEndpoints::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaEndpoint>> = icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
