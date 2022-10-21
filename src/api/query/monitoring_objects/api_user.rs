//! Icinga2 api user

crate::types::query::query!(
    ListApiUsers,
    ListApiUsersBuilder,
    monitoring_objects,
    api_user,
    IcingaApiUser,
    IcingaObjectType::ApiUser,
    "v1/objects/apiusers"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_api_users() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListApiUsers::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaApiUser>> = icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
