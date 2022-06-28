//! ServiceState
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/checkresult.ti)

use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// service state
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IcingaServiceState {
    /// service is OK
    Ok = 0,
    /// service is WARNING
    Warning = 1,
    /// service is CRITICAL
    Critical = 2,
    /// service is UNKNOWN
    Unknown = 3,
    /// service is UNREACHABLE
    Unreachable = 4,
    /// service is PENDING
    Pending = 99,
}

/// service state helper to deserialize by name
#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
#[serde(remote = "IcingaServiceState")]
pub enum IcingaServiceStateByName {
    /// service is OK
    Ok = 0,
    /// service is WARNING
    Warning = 1,
    /// service is CRITICAL
    Critical = 2,
    /// service is UNKNOWN
    Unknown = 3,
    /// service is UNREACHABLE
    Unreachable = 4,
    /// service is PENDING
    Pending = 99,
}
