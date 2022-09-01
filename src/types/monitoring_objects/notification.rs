//! Notification
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#notification)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/notification.ti)

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::serde::{
    deserialize_optional_icinga_timestamp, deserialize_optional_seconds_as_duration,
    serialize_optional_duration_as_seconds, serialize_optional_icinga_timestamp,
};
use crate::types::common::custom_var_object::CustomVarHolder;
use crate::types::enums::notification_filter::IcingaNotificationFilter;
use crate::types::enums::notification_type::IcingaNotificationType;
use crate::types::{
    common::custom_var_object::IcingaCustomVarObject,
    enums::object_type::IcingaObjectType,
    names::{
        IcingaEndpointName, IcingaHostName, IcingaNotificationCommandName, IcingaServiceName,
        IcingaTimePeriodName, IcingaUserGroupName, IcingaUserName,
    },
};

/// an icinga notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IcingaNotification {
    /// type of icinga object, should always be Notification for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// the notification command to call
    pub command: Option<IcingaNotificationCommandName>,
    /// the renotification interval
    #[serde(
        serialize_with = "serialize_optional_duration_as_seconds",
        deserialize_with = "deserialize_optional_seconds_as_duration"
    )]
    pub interval: Option<time::Duration>,
    /// the name of the time period when this notification is active
    pub period: Option<IcingaTimePeriodName>,
    /// the users to notify
    pub users: Option<Vec<IcingaUserName>>,
    /// the user groups to notify
    pub user_groups: Option<Vec<IcingaUserGroupName>>,
    /// A dictionary containing begin and end attributes for the notification.
    pub times: Option<BTreeMap<String, serde_json::Value>>,
    /// A list of type filters when this notification should be triggered. By default everything is matched.
    pub types: Option<Vec<IcingaNotificationType>>,
    /// A list of state filters when this notification should be triggered. By default everything is matched. Note that the states filter is ignored for notifications of type Acknowledgement!
    pub states: Option<Vec<IcingaNotificationFilter>>,
    /// The name of the host this notification belongs to.
    pub host_name: IcingaHostName,
    /// The short name of the service this notification belongs to. If omitted, this notification object is treated as host notification.
    pub service_name: Option<IcingaServiceName>,
    /// the users notified by this notification
    pub notified_problem_users: Option<Vec<IcingaUserName>>,
    /// do not send any more notifications for this issue
    pub no_more_notifications: Option<bool>,
    /// when was this notification last sent
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_notification: Option<time::OffsetDateTime>,
    /// when will this notification be sent next
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub next_notification: Option<time::OffsetDateTime>,
    /// the number of notifications sent out
    pub notification_number: u64,
    /// the last notification that was about a problem (as opposed to acknowlegement or end of problem)
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_problem_notification: Option<time::OffsetDateTime>,
    /// the command endpoint for the notification command
    pub command_endpoint: Option<IcingaEndpointName>,
}

impl CustomVarHolder for IcingaNotification {
    fn custom_var_value(&self, name: &str) -> Option<&serde_json::Value> {
        self.custom_var.custom_var_value(name)
    }
}
