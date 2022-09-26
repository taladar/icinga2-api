//! Icinga Notification types
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/notification.hpp)

use serde::{Deserialize, Serialize};

/// a notification type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "enumoid", derive(enumoid::Enumoid))]
pub enum IcingaNotificationType {
    /// start of a downtime
    #[serde(alias = "DOWNTIMESTART")]
    DowntimeStart,
    /// end of a downtime
    #[serde(alias = "DOWNTIMEEND")]
    DowntimeEnd,
    /// removal of a downtime
    #[serde(alias = "DOWNTIMECANCELLED")]
    DowntimeRemoved,
    /// custom notification
    #[serde(alias = "CUSTOM")]
    Custom,
    /// acknowledgement
    #[serde(alias = "ACKNOWLEDGEMENT")]
    Acknowledgement,
    /// problem
    #[serde(alias = "PROBLEM")]
    Problem,
    /// recovery
    #[serde(alias = "RECOVERY")]
    Recovery,
    /// start of flapping
    #[serde(alias = "FLAPPINGSTART")]
    FlappingStart,
    /// end of flapping
    #[serde(alias = "FLAPPINGEND")]
    FlappingEnd,
}
