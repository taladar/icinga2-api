//! Icinga2 services

#[cfg(test)]
mod test {
    use std::{collections::BTreeMap, error::Error};
    use tracing_test::traced_test;

    use crate::{
        api::Icinga2,
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
        icinga2.services(
            IcingaJoins::AllJoins,
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
            None,
        )?;
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
        icinga2.services(
            IcingaJoins::SpecificJoins {
                full: vec![],
                partial,
            },
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
            None,
        )?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_services_filtered() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.services(
            IcingaJoins::NoJoins,
            &[],
            Some(IcingaFilter {
                object_type: IcingaObjectType::Service,
                filter: "service.state == ServiceUnknown && service.vars.serviceSeverity == filter_severity".to_string(),
                filter_vars: BTreeMap::from([("filter_severity".to_string(), serde_json::json!("imminent"))]),
            }),
        )?;
        Ok(())
    }
}
