//! TimePeriod
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#timeperiod)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/timeperiod.ti)

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types::{
    common::{
        custom_var_object::{CustomVarHolder, IcingaCustomVarObject},
        function::IcingaFunction,
    },
    enums::object_type::IcingaObjectType,
    names::IcingaTimePeriodName,
};

/// an Icinga time period
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IcingaTimePeriod {
    /// type of icinga object, should always be TimePeriod for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// a short description of the time period
    pub display_name: String,
    /// the time ranges in this time period
    pub ranges: BTreeMap<String, String>,
    /// function to update this time period's calculated attributes
    pub update: IcingaFunction,
    /// whether includes or excludes are processed first
    pub prefer_includes: Option<bool>,
    /// excludes
    pub excludes: Vec<IcingaTimePeriodName>,
    /// includes
    pub includes: Vec<IcingaTimePeriodName>,
    /// TODO: not sure what the meaning of this attribute is
    pub valid_begin: Option<serde_json::Value>,
    /// TODO: not sure what the meaning of this attribute is
    pub valid_end: Option<serde_json::Value>,
    /// TODO: not sure what the meaning of this attribute is
    pub segments: Option<Vec<serde_json::Value>>,
    /// is the current time inside this time period
    pub is_inside: Option<bool>,
}

impl CustomVarHolder for IcingaTimePeriod {
    fn custom_var_value(&self, name: &str) -> Option<&serde_json::Value> {
        self.custom_var.custom_var_value(name)
    }
}
