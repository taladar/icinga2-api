//! Zone
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#zone)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/remote/zone.ti)

use serde::{Deserialize, Serialize};

use crate::types::{
    common::config_object::IcingaConfigObject,
    enums::object_type::IcingaObjectType,
    names::{IcingaEndpointName, IcingaZoneName},
};

/// an Icinga cluster zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcingaZone {
    /// type of icinga object, should always be Zone for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object fields
    #[serde(flatten)]
    pub config_object: IcingaConfigObject,
    /// parent zone
    pub parent: Option<IcingaZoneName>,
    /// endpoints in the zone
    pub endpoints: Option<Vec<IcingaEndpointName>>,
    /// is this a global zone
    pub global: Option<bool>,
    /// all parents
    pub all_parents: Option<serde_json::Value>,
}
