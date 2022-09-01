//! ServiceGroup
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#servicegroup)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/servicegroup.ti)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_empty_string_or_string, serialize_none_as_empty_string};
use crate::types::common::custom_var_object::CustomVarHolder;
use crate::types::{
    common::custom_var_object::IcingaCustomVarObject, enums::object_type::IcingaObjectType,
    names::IcingaServiceGroupName,
};

/// a service group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IcingaServiceGroup {
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// a short description of the service group
    pub display_name: String,
    /// a list of groups the service group belongs to
    pub groups: Option<Vec<IcingaServiceGroupName>>,
    /// URL for actions for the checkable (host or service)
    #[serde(
        serialize_with = "serialize_none_as_empty_string",
        deserialize_with = "deserialize_empty_string_or_string"
    )]
    pub action_url: Option<String>,
    /// notes for the host/service
    #[serde(
        serialize_with = "serialize_none_as_empty_string",
        deserialize_with = "deserialize_empty_string_or_string"
    )]
    pub notes: Option<String>,
    /// URL for notes for the host/service
    #[serde(
        serialize_with = "serialize_none_as_empty_string",
        deserialize_with = "deserialize_empty_string_or_string"
    )]
    pub notes_url: Option<String>,
}

impl CustomVarHolder for IcingaServiceGroup {
    fn custom_var_value(&self, name: &str) -> Option<&serde_json::Value> {
        self.custom_var.custom_var_value(name)
    }
}
