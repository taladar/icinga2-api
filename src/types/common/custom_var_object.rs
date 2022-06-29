//! Custom var object with shared fields
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/customvarobject.ti)

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::config_object::IcingaConfigObject;

/// shared fields in the various objects supporting custom variables
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaCustomVarObject {
    /// custom variables specific to this object
    pub vars: Option<BTreeMap<String, serde_json::Value>>,
    /// shared config object fields
    #[serde(flatten)]
    pub config_object: IcingaConfigObject,
}
