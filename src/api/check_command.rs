//! Icinga2 check command

#[cfg(test)]
mod test {
    use crate::{api::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_check_commands() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.check_commands(
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
            None,
        )?;
        Ok(())
    }
}
