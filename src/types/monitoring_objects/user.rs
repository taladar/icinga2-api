//! User
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#user)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/user.ti)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_optional_icinga_timestamp, serialize_optional_icinga_timestamp};
use crate::types::{
    common::custom_var_object::IcingaCustomVarObject,
    enums::{host_or_service_state::IcingaHostOrServiceState, object_type::IcingaObjectType},
    names::{IcingaTimePeriodName, IcingaUserGroupName},
};

/// an icinga user (e.g. for notification purposes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcingaUser {
    /// type of icinga object, should always be User for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// a short description of the user
    pub display_name: String,
    /// the groups in which the user is a member
    pub groups: Option<Vec<IcingaUserGroupName>>,
    /// the name of the time period when this user is active
    pub period: Option<IcingaTimePeriodName>,
    /// A set of type filters when a notification for this user should be triggered. By default everything is matched.
    pub types: Option<Vec<IcingaObjectType>>,
    /// A set of state filters when a notification for this should be triggered. By default everything is matched.
    pub states: Option<Vec<IcingaHostOrServiceState>>,
    /// user email address
    pub email: Option<String>,
    /// user pager number
    pub pager: Option<String>,
    /// enable notifications for this user
    pub enable_notifications: Option<bool>,
    /// when did we send the last notification to this user
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_notification: Option<time::OffsetDateTime>,
}
