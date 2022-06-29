//! Service
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#service)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/service.ti)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_optional_icinga_timestamp, serialize_optional_icinga_timestamp};
use crate::types::{
    common::checkable::IcingaCheckable,
    enums::{object_type::IcingaObjectType, service_state::IcingaServiceState},
    names::{IcingaHostName, IcingaServiceGroupName},
};

/// an Icinga monitored service
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaService {
    /// type of icinga object, should always be Service for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// all the attributes from the icinga checkable object (shared fields between host and service)
    #[serde(flatten)]
    pub checkable: IcingaCheckable,
    /// a short description of the service
    pub display_name: String,
    /// a list of groups the service belongs to
    pub groups: Vec<IcingaServiceGroupName>,
    /// the hostname for this service
    pub host_name: IcingaHostName,
    /// the previous hard state
    pub last_hard_state: IcingaServiceState,
    /// the previous state
    pub last_state: IcingaServiceState,
    /// when the last CRITICAL state occurred
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_state_critical: Option<time::OffsetDateTime>,
    /// when the last OK state occurred
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_state_ok: Option<time::OffsetDateTime>,
    /// when the last UNKNOWN state occurred
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_state_unknown: Option<time::OffsetDateTime>,
    /// when the last WARNINGE state occurred
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_state_warning: Option<time::OffsetDateTime>,
    /// the current state
    pub state: IcingaServiceState,
}
