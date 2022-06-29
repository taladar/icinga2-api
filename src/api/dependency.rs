//! Icinga2 dependency

#[cfg(test)]
mod test {
    use std::error::Error;
    use tracing_test::traced_test;

    use crate::{
        api::Icinga2,
        types::{join_types::IcingaJoins, metadata::IcingaMetadataType},
    };

    #[traced_test]
    #[test]
    fn test_dependencies() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.dependencies(
            IcingaJoins::AllJoins,
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
            None,
        )?;
        Ok(())
    }
}
