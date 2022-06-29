//! Icinga2 user groups

#[cfg(test)]
mod test {
    use std::error::Error;
    use tracing_test::traced_test;

    use crate::{api::Icinga2, types::metadata::IcingaMetadataType};

    #[traced_test]
    #[test]
    fn test_user_groups() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        icinga2.user_groups(
            &[IcingaMetadataType::UsedBy, IcingaMetadataType::Location],
            None,
        )?;
        Ok(())
    }
}
