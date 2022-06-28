//! Icinga2 host

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    enums::IcingaObjectType,
    serde::{deserialize_empty_string_or_parse, deserialize_optional_icinga_timestamp},
};

use super::{
    checkable::IcingaCheckable,
    host_group::IcingaHostGroupName,
    joins::{IcingaJoinResult, IcingaJoinType},
    metadata::IcingaMetadata,
};

/// attributes on an [IcingaHost]
#[derive(Debug, Deserialize)]
pub struct IcingaHostAttributes {
    /// type of icinga object, should always be Host for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// all the attributes from the icinga checkable object (shared fields between host and service)
    #[serde(flatten)]
    pub checkable: IcingaCheckable,
    /// host Ipv4 address
    pub address: std::net::Ipv4Addr,
    /// optional host Ipv6 address
    #[serde(deserialize_with = "deserialize_empty_string_or_parse")]
    pub address6: Option<std::net::Ipv6Addr>,
    /// a short description of the host
    pub display_name: String,
    /// a list of groups the host belongs to
    pub groups: Vec<IcingaHostGroupName>,
    /// the previous hard state
    pub last_hard_state: IcingaHostState,
    /// the previous state
    pub last_state: IcingaHostState,
    /// when the last DOWN state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_down: Option<time::OffsetDateTime>,
    /// when the last UP state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_up: Option<time::OffsetDateTime>,
    /// the current state
    pub state: IcingaHostState,
}

/// the result of an icinga hosts query
#[derive(Debug, Deserialize)]
pub struct IcingaHost {
    /// host attributes
    pub attrs: IcingaHostAttributes,
    /// joins
    pub joins: IcingaHostJoins,
    /// metadata, only contains data if the parameter meta contains one or more values
    pub meta: IcingaMetadata,
    /// object name
    pub name: String,
    /// type of icinga object, should always be Host for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}


#[cfg(test)]
mod test {
    use crate::{
        api::{
            filter::IcingaFilter, host::IcingaHostState, joins::IcingaJoins,
            metadata::IcingaMetadataType, Icinga2,
        },
        enums::IcingaObjectType,
    };
    use std::{collections::BTreeMap, error::Error};
    use tracing_test::traced_test;

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
