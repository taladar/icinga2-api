//! Custom var object with shared fields
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/customvarobject.ti)

use std::collections::BTreeMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::config_object::IcingaConfigObject;

/// allows easier retrieval of custom variables from all objects which store them somewhere
/// (possibly deep in some nested field)
pub trait CustomVarHolder {
    /// retrieve a reference to the value of the named custom variable if it
    /// exists
    fn custom_var_value(&self, name: &str) -> Option<&serde_json::Value>;

    /// deserialize the Value into a user provided data type
    fn custom_var_deserialized<T>(&self, name: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        self.custom_var_value(name)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

/// shared fields in the various objects supporting custom variables
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IcingaCustomVarObject {
    /// custom variables specific to this object
    pub vars: Option<BTreeMap<String, serde_json::Value>>,
    /// shared config object fields
    #[serde(flatten)]
    pub config_object: IcingaConfigObject,
}

impl CustomVarHolder for IcingaCustomVarObject {
    fn custom_var_value(&self, name: &str) -> Option<&serde_json::Value> {
        self.vars.as_ref().and_then(|vars| vars.get(name))
    }
}
