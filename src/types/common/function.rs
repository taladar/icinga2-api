//! Function
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/base/function.ti)

use serde::{Deserialize, Serialize};

use crate::types::enums::object_type::IcingaObjectType;

/// the description of an icinga function
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaFunction {
    /// the arguments
    pub arguments: Vec<String>,
    /// is this deprecated
    pub deprecated: bool,
    /// the name
    pub name: String,
    /// is this command side-effect free
    pub side_effect_free: bool,
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}
