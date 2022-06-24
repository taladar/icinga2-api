//! Icinga2 Custom var object with shared fields

use std::collections::BTreeMap;

use serde::Deserialize;

use super::config_object::IcingaConfigObject;

/// an icinga variable value
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaVariableValue {
    /// string value
    String(String),
    /// list of strings value
    List(Vec<String>),
    /// key/value object
    Object(BTreeMap<String, IcingaVariableValue>),
    /// Boolean
    Boolean(bool),
    /// Integer
    Integer(i64),
}

/// shared fields in the various objects supporting custom variables
#[derive(Debug, Deserialize)]
pub struct IcingaCustomVarObject {
    /// custom variables specific to this object
    pub vars: Option<BTreeMap<String, IcingaVariableValue>>,
    /// shared config object fields
    #[serde(flatten)]
    pub config_object: IcingaConfigObject,
}
