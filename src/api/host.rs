//! Icinga2 host

#[cfg(test)]
mod test {
    use std::{collections::BTreeMap, error::Error};
    use tracing_test::traced_test;

    use crate::{
        api::Icinga2,
        types::{
            enums::{host_state::IcingaHostState, object_type::IcingaObjectType},
            filter::IcingaFilter,
            join_types::IcingaJoins,
            metadata::IcingaMetadataType,
        },
    };

    #[traced_test]
    #[test]
    fn test_hosts() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.hosts(
            IcingaJoins::AllJoins,
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
            None,
        )?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_hosts_filtered() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.hosts(
            IcingaJoins::NoJoins,
            &[],
            Some(IcingaFilter {
                object_type: IcingaObjectType::Host,
                filter: "host.state == filter_state".to_string(),
                filter_vars: BTreeMap::from([(
                    "filter_state".to_string(),
                    serde_json::to_value(IcingaHostState::Up)?,
                )]),
            }),
        )?;
        Ok(())
    }
}
