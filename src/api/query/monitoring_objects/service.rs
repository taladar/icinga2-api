//! Icinga2 services
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#service)

crate::types::query::query_with_joins!(
    ListServices,
    ListServicesBuilder,
    monitoring_objects,
    service,
    IcingaService,
    IcingaServiceJoinTypes,
    IcingaServiceJoins,
    IcingaObjectType::Service,
    "v1/objects/services"
);

#[cfg(test)]
mod test {
    use super::*;
    use std::{collections::BTreeMap, error::Error};
    use tracing_test::traced_test;

    use crate::{
        api::blocking::Icinga2,
        types::{
            enums::object_type::IcingaObjectType,
            filter::IcingaFilter,
            join_types::{service::IcingaServiceJoinTypes, IcingaJoins},
            metadata::IcingaMetadataType,
        },
    };

    #[traced_test]
    #[test]
    fn test_services() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListServices::builder()
            .joins(IcingaJoins::AllJoins)
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObjectWithJoins<IcingaService, IcingaServiceJoins>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_services_partial_host_join() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let mut partial = BTreeMap::new();
        partial.insert(IcingaServiceJoinTypes::Host, vec!["name"]);
        let api_endpoint = ListServices::builder()
            .joins(IcingaJoins::SpecificJoins {
                full: vec![],
                partial,
            })
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObjectWithJoins<IcingaService, IcingaServiceJoins>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_services_filtered() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListServices::builder()
            .filter(IcingaFilter {
                object_type: IcingaObjectType::Service,
                filter: "service.state == ServiceUnknown && service.vars.serviceSeverity == filter_severity".to_string(),
                filter_vars: BTreeMap::from([("filter_severity".to_string(), serde_json::json!("imminent"))]),
            })
            .build()?;
        let _: ResultsWrapper<QueryResultObjectWithJoins<IcingaService, IcingaServiceJoins>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
