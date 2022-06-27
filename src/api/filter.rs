//! data types relating to filtering

use std::collections::BTreeMap;

use serde::Serialize;

use crate::enums::IcingaObjectType;

/// represents an icinga filter as passed to icinga to limit queries and operations
#[derive(Debug, Serialize)]
pub struct IcingaFilter {
    /// the object type we want to filter
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// the filter expression, do not interpolate variable values into this dynamically, use filter_vars instead
    pub filter: String,
    /// the variable values for variables used in the filter expression
    pub filter_vars: BTreeMap<String, serde_json::Value>,
}
