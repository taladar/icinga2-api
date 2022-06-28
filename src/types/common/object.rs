//! Minimal Icinga Object (type + name)

use serde::{Serialize, Deserialize};

use crate::types::enums::object_type::IcingaObjectType;

/// the most minimal description of an icinga object
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaObject {
    /// the name of the object
    pub name: String,
    /// the type of the object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}
