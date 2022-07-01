//! Event Stream Type: Notification
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-stream-type-notification)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/apievents.cpp#L107=)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::common::check_result::IcingaCheckResult;
use crate::types::enums::notification_type::IcingaNotificationType;
use crate::types::names::{
    IcingaHostName, IcingaNotificationCommandName, IcingaServiceName, IcingaUserName,
};

/// the Notification event type
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaEventNotification {
    /// when the event happened
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    pub timestamp: time::OffsetDateTime,
    /// host on which  the event happened
    pub host: IcingaHostName,
    /// service for which the event happened, if not specified this is a host event
    pub service: Option<IcingaServiceName>,
    /// the notification command
    pub command: IcingaNotificationCommandName,
    /// the users that were notified
    pub users: Vec<IcingaUserName>,
    /// the type of notification that was sent out
    pub notification_type: IcingaNotificationType,
    /// the author of the notification
    pub author: String,
    /// the text of the notification
    pub text: String,
    /// the related check result
    pub check_result: IcingaCheckResult,
}
