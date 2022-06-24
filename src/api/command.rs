//! Icinga2 structs representing various command related concepts
use serde::Deserialize;

use super::IcingaFunction;

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
pub enum IcingaCommand {
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
