//! ObjectType

use serde::{Deserialize, Serialize};

/// the type of icinga object we are dealing with
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IcingaObjectType {
    /// an icinga monitored host
    Host,
    /// an icinga service
    Service,
    /// an icinga check result
    CheckResult,
    /// a performance data value
    PerfdataValue,
    /// an icinga comment
    Comment,
    /// an icinga dependency between hosts or services
    Dependency,
    /// an icinga notification
    Notification,
    /// a function
    Function,
    /// a check command
    CheckCommand,
    /// a notification command
    NotificationCommand,
    /// an event command
    EventCommand,
    /// a host group
    HostGroup,
    /// a service group
    ServiceGroup,
    /// a user group
    UserGroup,
    /// an endpoint
    Endpoint,
    /// a time period
    TimePeriod,
    /// a zone
    Zone,
    /// a user
    User,
    /// a scheduled downtime
    ScheduledDowntime,
    /// a downtime
    Downtime,
    /// an api user
    ApiUser,
}
