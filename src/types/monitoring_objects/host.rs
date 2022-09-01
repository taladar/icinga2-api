//! Host
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#host)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/host.ti)

use serde::{Deserialize, Serialize};

use crate::serde::{
    deserialize_empty_string_or_parse, deserialize_optional_icinga_timestamp,
    serialize_none_as_empty_string_or_to_string, serialize_optional_icinga_timestamp,
};
use crate::types::common::custom_var_object::CustomVarHolder;
use crate::types::{
    common::checkable::IcingaCheckable,
    enums::{host_state::IcingaHostState, object_type::IcingaObjectType},
    names::IcingaHostGroupName,
};

/// a host monitored by Icinga
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IcingaHost {
    /// type of icinga object, should always be Host for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// all the attributes from the icinga checkable object (shared fields between host and service)
    #[serde(flatten)]
    pub checkable: IcingaCheckable,
    /// host Ipv4 address
    pub address: std::net::Ipv4Addr,
    /// optional host Ipv6 address
    #[serde(
        serialize_with = "serialize_none_as_empty_string_or_to_string",
        deserialize_with = "deserialize_empty_string_or_parse"
    )]
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
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_state_down: Option<time::OffsetDateTime>,
    /// when the last UP state occurred
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_state_up: Option<time::OffsetDateTime>,
    /// the current state
    pub state: IcingaHostState,
}

impl CustomVarHolder for IcingaHost {
    fn custom_var_value(&self, name: &str) -> Option<&serde_json::Value> {
        self.checkable.custom_var_value(name)
    }
}
