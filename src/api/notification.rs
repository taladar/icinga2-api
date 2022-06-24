//! Icinga2 notifications
use serde::Deserialize;

use super::{
    host::IcingaHostAttributes, joins::IcingaJoinResult, service::IcingaServiceAttributes,
    IcingaJoinType,
};

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
#[derive(Debug, Deserialize)]
pub struct IcingaNotificationJoins {
    /// the host this Notification is about
    pub host: Option<IcingaJoinResult<IcingaHostAttributes>>,
    /// the service this Notification is about
    pub service: Option<IcingaJoinResult<IcingaServiceAttributes>>,
    ///// the notification command object for the notification
    //pub command: Option<IcingaJoinResult<IcingaNotificationCommandAttributes>>,
    //pub period: Option<IcingaJoinResult<IcingaPeriodAttributes>>,
}
