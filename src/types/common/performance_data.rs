//! Icinga2 performance data as it appears in check results

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_empty_string_or_string, serialize_none_as_empty_string};
use crate::types::enums::object_type::IcingaObjectType;

/// represents performance data
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IcingaPerformanceData {
    /// performance data in string format
    String(String),
    /// structured performance data value
    PerfDataValue {
        /// object type, should always be PerfdataValue here
        #[serde(rename = "type")]
        object_type: IcingaObjectType,
        /// is this a counter
        counter: bool,
        /// the current value
        value: f64,
        /// the critical value
        crit: Option<f64>,
        /// the warning value
        warn: Option<f64>,
        /// the label for the type of values
        #[serde(
            serialize_with = "serialize_none_as_empty_string",
            deserialize_with = "deserialize_empty_string_or_string"
        )]
        label: Option<String>,
        /// the minimum value
        min: Option<f64>,
        /// the maximum value
        max: Option<f64>,
        /// the unit for the type of values
        #[serde(
            serialize_with = "serialize_none_as_empty_string",
            deserialize_with = "deserialize_empty_string_or_string"
        )]
        unit: Option<String>,
    },
}
