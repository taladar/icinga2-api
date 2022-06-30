//! Icinga2 dependency

crate::types::query::query_with_joins!(
    ListDependencies,
    ListDependenciesBuilder,
    monitoring_objects,
    dependency,
    IcingaDependency,
    IcingaDependencyJoinTypes,
    IcingaDependencyJoins,
    IcingaObjectType::Dependency,
    "v1/objects/dependencies"
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
    fn test_dependencies() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListDependencies::builder()
            .joins(IcingaJoins::AllJoins)
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObjectWithJoins<IcingaDependency, IcingaDependencyJoins>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
