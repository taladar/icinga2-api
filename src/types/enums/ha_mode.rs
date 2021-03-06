//! HAMode
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/base/configobject.ti)

use serde_repr::{Deserialize_repr, Serialize_repr};

/// HA mode
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum HAMode {
    /// run a check once
    HARunOnce,
    /// run a check everywhere
    HARunEverywhere,
}
