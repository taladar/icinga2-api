//! Notification

use serde::{Serialize, Deserialize};

use super::{IcingaJoinType, IcingaJoinResult};

/// possible joins parameter values for notifications
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaNotificationJoinTypes {
    /// the host the notification is about
    Host,
    /// the service the notification is about
    Service,
    /// the notification command object for the notification
    Command,
    /// the notification period object for the notification
    Period,
}

impl IcingaJoinType for IcingaNotificationJoinTypes {}

impl std::fmt::Display for IcingaNotificationJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaNotificationJoinTypes::Host => write!(f, "host"),
            IcingaNotificationJoinTypes::Service => write!(f, "service"),
            IcingaNotificationJoinTypes::Command => write!(f, "command"),
            IcingaNotificationJoinTypes::Period => write!(f, "period"),
        }
    }
}

/// return type joins for notifications
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaNotificationJoins {
    /// the host this Notification is about
    pub host: Option<IcingaJoinResult<IcingaHost>>,
    /// the service this Notification is about
    pub service: Option<IcingaJoinResult<IcingaService>>,
    ///// the notification command object for the notification
    pub command: Option<IcingaJoinResult<IcingaNotificationCommand>>,
    /// the time period when the notification is active
    pub period: Option<IcingaJoinResult<IcingaTimePeriod>>,
}
