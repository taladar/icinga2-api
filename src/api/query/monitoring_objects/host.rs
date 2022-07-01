//! Icinga2 host
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#host)

crate::types::query::query_with_joins!(
    ListHosts,
    ListHostsBuilder,
    monitoring_objects,
    host,
    IcingaHost,
    IcingaHostJoinTypes,
    IcingaHostJoins,
    IcingaObjectType::Host,
    "v1/objects/hosts"
);

#[cfg(test)]
mod test {
    use super::*;
    use std::{collections::BTreeMap, error::Error};
    use tracing_test::traced_test;

    use crate::{
        api::async_client::Icinga2Async,
        api::blocking::Icinga2,
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
        let api_endpoint = ListHosts::builder()
            .joins(IcingaJoins::AllJoins)
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObjectWithJoins<IcingaHost, IcingaHostJoins>> =
            icinga2.rest(api_endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_hosts_filtered() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListHosts::builder()
            .filter(IcingaFilter {
                object_type: IcingaObjectType::Host,
                filter: "host.state == filter_state".to_string(),
                filter_vars: BTreeMap::from([(
                    "filter_state".to_string(),
                    serde_json::to_value(IcingaHostState::Up)?,
                )]),
            })
            .build()?;
        let _: ResultsWrapper<QueryResultObject<IcingaHost>> = icinga2.rest(api_endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[tokio::test]
    async fn test_hosts_async() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let icinga2 = Icinga2Async::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let api_endpoint = ListHosts::builder()
            .joins(IcingaJoins::AllJoins)
            .meta([IcingaMetadataType::UsedBy, IcingaMetadataType::Location])
            .build()?;
        let _: ResultsWrapper<QueryResultObjectWithJoins<IcingaHost, IcingaHostJoins>> =
            icinga2.rest(api_endpoint).await?;
        Ok(())
    }
}
