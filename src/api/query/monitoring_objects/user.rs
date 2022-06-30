//! Icinga2 users

crate::types::query::query_with_joins!(
    ListUsers,
    ListUsersBuilder,
    monitoring_objects,
    user,
    IcingaUser,
    IcingaUserJoinTypes,
    IcingaUserJoins,
    IcingaObjectType::User,
    "v1/objects/users"
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
    fn test_users() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListUsers::builder()
            .joins(IcingaJoins::AllJoins)
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObjectWithJoins<IcingaUser, IcingaUserJoins>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
