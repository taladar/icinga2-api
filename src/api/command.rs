//! Icinga2 structs representing various command related concepts
use std::collections::BTreeMap;

use serde::Deserialize;

use super::{custom_var_object::IcingaCustomVarObject, IcingaFunction};
use crate::serde::deserialize_optional_seconds_as_duration;

/// shared fields in the various command objects
#[derive(Debug, Deserialize)]
pub struct IcingaCommand {
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// the descriptions of the command arguments
    pub arguments: Option<BTreeMap<String, IcingaCommandArgumentDescription>>,
    /// the actual command
    pub command: Option<IcingaCommandLine>,
    /// environment variables
    pub env: Option<BTreeMap<String, String>>,
    /// function for execution
    pub execute: IcingaFunction,
    /// command timeout
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub timeout: Option<time::Duration>,
}

/// command parameters (scalar values basically)
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaCommandParameter {
    /// string value
    String(String),
    /// Boolean
    Boolean(bool),
    /// Integer
    Integer(i64),
}

/// command to execute with parameters
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaCommandLine {
    /// a single string for the whole command, will likely need a shell to do
    /// word splitting
    Shell(String),
    /// individual command and parameters
    Exec(Vec<IcingaCommandParameter>),
}

/// set_if condition in command argument description
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaArgumentCondition {
    /// a string condition, most likely a boolean variable
    String(String),
    /// a function condition
    Function(IcingaFunction),
}

/// the description of a single
#[derive(Debug, Deserialize)]
pub struct IcingaCommandArgumentDescription {
    /// the description of this argument
    pub description: Option<String>,
    /// the default value for this argument
    pub value: Option<String>,
    /// name of an argument to set
    pub key: Option<String>,
    /// should the key be repeated
    pub repeat_key: Option<bool>,
    /// condition when to set it
    pub set_if: Option<IcingaArgumentCondition>,
    /// is this argument required
    pub required: Option<bool>,
}
