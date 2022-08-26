//! DowntimeChildOptions
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/downtime.hpp)

use serde::{Deserialize, Serialize};

/// what to do with children when a downtime starts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaDowntimeChildOptions {
    /// downtime does not affect children
    DowntimeNoChildren,
    /// schedules child downtimes triggered by this downtime
    DowntimeTriggeredChildren,
    /// schedules non-triggered downtimes
    DOwntimeNonTriggeredChildren,
}
