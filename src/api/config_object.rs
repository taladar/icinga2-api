//! Icinga2 Config object with shared fields

use serde::Deserialize;

use crate::enums::HAMode;
use crate::serde::{deserialize_empty_string_or_string, deserialize_optional_icinga_timestamp};

/// an icinga source location inside the icinga config files
#[derive(Debug, Deserialize)]
pub struct IcingaSourceLocation {
    /// path of the config file
    pub path: String,
    /// start line
    pub first_line: u64,
    /// start column
    pub first_column: u64,
    /// end line
    pub last_line: u64,
    /// end column
    pub last_column: u64,
}

/// shared fields in the various objects defined in the configuration
#[derive(Debug, Deserialize)]
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
    pub original_attributes: Option<()>,
    /// configuration package name this object belongs to, _etc for local configuration
    /// _api for runtime created objects
    pub package: String,
    /// object has been paused at runtime
    pub paused: bool,
    /// location information whether the configuration files are stored
    pub source_location: IcingaSourceLocation,
    /// templates imported on object compilation
    pub templates: Vec<String>,
    /// timestamp when the object was created or modified. syncred throughout cluster nodes
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub version: Option<time::OffsetDateTime>,
    /// the zone this object is a member of
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub zone: Option<String>,
}
