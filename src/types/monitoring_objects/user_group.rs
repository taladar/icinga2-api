//! UserGroup
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#usergroup)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/usergroup.ti)

use serde::{Deserialize, Serialize};

use crate::types::{
    common::custom_var_object::{CustomVarHolder, IcingaCustomVarObject},
    enums::object_type::IcingaObjectType,
    names::IcingaUserGroupName,
};

/// a user group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IcingaUserGroup {
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// a short description of the user group
    pub display_name: String,
    /// a list of groups the user group belongs to
    pub groups: Option<Vec<IcingaUserGroupName>>,
}

impl CustomVarHolder for IcingaUserGroup {
    fn custom_var_value(&self, name: &str) -> Option<&serde_json::Value> {
        self.custom_var.custom_var_value(name)
    }
}
