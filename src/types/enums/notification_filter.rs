//! Icinga notification filter
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/notification.hpp)

use serde::{Deserialize, Serialize};

/// icinga notification filter
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "enumoid", derive(enumoid::Enumoid))]
pub enum IcingaNotificationFilter {
    /// service state filter OK
    #[serde(rename = "OK")]
    StateFilterOk,
    /// service state filter Warning
    #[serde(rename = "Warning")]
    StateFilterWarning,
    /// service state filter Critical
    #[serde(rename = "Critical")]
    StateFilterCritical,
    /// service state filter Unknown
    #[serde(rename = "Unknown")]
    StateFilterUnknown,

    /// host state filter Up
    #[serde(rename = "Up")]
    StateFilterUp,
    /// host state filter Down
    #[serde(rename = "Down")]
    StateFilterDown,
}
