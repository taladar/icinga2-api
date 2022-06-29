//! ApiUser
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#apiuser)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/remote/apiuser.ti)

use serde::{Deserialize, Serialize};

use crate::types::{
    common::{config_object::IcingaConfigObject, function::IcingaFunction},
    enums::object_type::IcingaObjectType,
};

/// an icinga permission entry
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]

pub enum IcingaPermissionEntry {
    /// a simple string permission
    String(String),
    /// a permission with a filter function
    Filtered {
        /// the permission
        permission: String,
        /// the function to filter if it is allowed
        filter: IcingaFunction,
    },
}

/// an API user
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaApiUser {
    /// type of icinga object, should always be ApiUser for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object fields
    #[serde(flatten)]
    pub config_object: IcingaConfigObject,
    /// client CN
    pub client_cn: Option<String>,
    /// permissions
    pub permissions: Option<Vec<IcingaPermissionEntry>>,
}
