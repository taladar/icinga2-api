//! Icinga2 user groups

crate::types::query::query!(
    ListUserGroups,
    ListUserGroupsBuilder,
    monitoring_objects,
    user_group,
    IcingaUserGroup,
    IcingaObjectType::UserGroup,
    "v1/objects/usergroups"
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::{api::blocking::Icinga2, types::metadata::IcingaMetadataType};
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_user_groups() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListUserGroups::builder()
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaUserGroup>> = icinga2.rest(api_endpoint)?;
        Ok(())
    }
}
