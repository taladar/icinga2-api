//! Icinga Notification types
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/notification.hpp)

use serde::{Deserialize, Serialize};

/// a notification type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IcingaNotificationType {
    /// start of a downtime
    DowntimeStart,
    /// end of a downtime
    DowntimeEnd,
    /// removal of a downtime
    DowntimeRemoved,
    /// custom notification
    Custom,
    /// acknowledgement
    Acknowledgement,
    /// problem
    Problem,
    /// recovery
    Recovery,
    /// start of flapping
    FlappingStart,
    /// end of flapping
    FlappingEnd,
}
