//! ConfigObject
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/base/configobject.ti)

use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

use crate::types::enums::ha_mode::HAMode;

use super::source_location::IcingaSourceLocation;

use crate::serde::{deserialize_empty_string_or_parse, deserialize_optional_icinga_timestamp};

/// shared fields in the various objects defined in the configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaConfigObject {
    /// full object name
    #[serde(rename = "__name")]
    pub full_name: String,
    /// short object name
    pub name: String,
    /// object is active (being checked)
    pub active: bool,
    /// whether to run a check once or everywhere
    pub ha_mode: HAMode,
    /// original values of object attributes modified at runtime
    pub original_attributes: Option<BTreeMap<String, serde_json::Value>>,
    /// configuration package name this object belongs to, _etc for local configuration
    /// _api for runtime created objects
    pub package: IcingaPackageName,
    /// object has been paused at runtime
    pub paused: bool,
    /// location information whether the configuration files are stored
    pub source_location: IcingaSourceLocation,
    /// templates imported on object compilation
    pub templates: Vec<IcingaTemplateName>,
    /// timestamp when the object was created or modified. syncred throughout cluster nodes
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub version: Option<time::OffsetDateTime>,
    /// the zone this object is a member of
    #[serde(deserialize_with = "deserialize_empty_string_or_parse")]
    pub zone: Option<IcingaZoneName>,
}
