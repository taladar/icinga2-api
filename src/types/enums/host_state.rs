//! HostState
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/checkresult.ti)

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// host state
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "enumoid", derive(enumoid::Enumoid))]
#[repr(u8)]
pub enum IcingaHostState {
    /// host is up
    Up = 0,
    /// host is down
    Down = 1,
    /// host is unreachable
    Unreachable = 2,
}

/// host state deserialization helper by name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(u8)]
#[serde(remote = "IcingaHostState")]
pub enum IcingaHostStateByName {
    /// host is up
    Up = 0,
    /// host is down
    Down = 1,
    /// host is unreachable
    Unreachable = 2,
}
