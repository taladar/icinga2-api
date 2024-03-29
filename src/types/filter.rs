//! data types relating to filtering

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::enums::object_type::IcingaObjectType;

/// represents an icinga filter as passed to icinga to limit queries and operations
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct IcingaFilter {
    /// the object type we want to filter
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// the filter expression, do not interpolate variable values into this dynamically, use filter_vars instead
    pub filter: String,
    /// the variable values for variables used in the filter expression
    pub filter_vars: BTreeMap<String, serde_json::Value>,
}
