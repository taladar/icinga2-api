//! Dependency
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#dependency)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/dependency.ti)

use serde::{Deserialize, Serialize};

use crate::serde::{
    deserialize_empty_string_or_parse, serialize_none_as_empty_string_or_to_string,
};
use crate::types::names::IcingaTimePeriodName;
use crate::types::{
    common::custom_var_object::IcingaCustomVarObject,
    enums::{host_or_service_state::IcingaHostOrServiceState, object_type::IcingaObjectType},
    names::{IcingaHostName, IcingaServiceName},
};

/// attributes on an [IcingaDependency]
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaDependency {
    /// type of icinga object, should always be Dependency for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// the child host name
    pub child_host_name: IcingaHostName,
    /// the child service name
    #[serde(
        serialize_with = "serialize_none_as_empty_string_or_to_string",
        deserialize_with = "deserialize_empty_string_or_parse"
    )]
    pub child_service_name: Option<IcingaServiceName>,
    /// the parent host name
    pub parent_host_name: IcingaHostName,
    /// the parent service name
    #[serde(
        serialize_with = "serialize_none_as_empty_string_or_to_string",
        deserialize_with = "deserialize_empty_string_or_parse"
    )]
    pub parent_service_name: Option<IcingaServiceName>,
    /// whether checks are disabled by this dependency
    pub disable_checks: bool,
    /// whether notifications are disabled by this dependency
    pub disable_notifications: bool,
    /// whether this dependency ignores soft states
    pub ignore_soft_states: bool,
    /// the name of the time period when this dependency is active
    pub period: Option<IcingaTimePeriodName>,
    /// states when this dependency is enabled
    pub states: Vec<IcingaHostOrServiceState>,
}
