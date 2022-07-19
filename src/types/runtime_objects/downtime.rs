//! Downtime
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#downtime)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/downtime.ti)

use serde::{Deserialize, Serialize};

use crate::serde::{
    deserialize_optional_icinga_timestamp, deserialize_optional_seconds_as_duration,
    serialize_optional_duration_as_seconds, serialize_optional_icinga_timestamp,
};
use crate::types::names::{IcingaDowntimeName, IcingaScheduledDowntimeName, IcingaZoneName};
use crate::types::{
    common::config_object::IcingaConfigObject,
    enums::object_type::IcingaObjectType,
    names::{IcingaHostName, IcingaServiceName},
};

/// an icinga downtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcingaDowntime {
    /// type of icinga object, should always be Downtime for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object fields
    #[serde(flatten)]
    pub config_object: IcingaConfigObject,
    /// the host for which the downtime will be scheduled
    pub host_name: IcingaHostName,
    /// the service for which the downtime will be scheduled, if not specified this is a host downtime
    pub service_name: Option<IcingaServiceName>,
    /// the time when the downtime was created (but not necessarily started)
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub entry_time: Option<time::OffsetDateTime>,
    /// the author of the downtime
    pub author: String,
    /// the comment displayed for the downtime
    pub comment: String,
    /// the time when the downtime starts
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub start_time: Option<time::OffsetDateTime>,
    /// the time when the downtime ends
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub end_time: Option<time::OffsetDateTime>,
    /// the time when this downtime was triggered
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub trigger_time: Option<time::OffsetDateTime>,
    /// is this a fixed downtime
    pub fixed: Option<bool>,
    /// the duration of the downtime
    #[serde(
        serialize_with = "serialize_optional_duration_as_seconds",
        deserialize_with = "deserialize_optional_seconds_as_duration"
    )]
    pub duration: Option<time::Duration>,
    /// which other downtime triggered this one
    pub triggered_by: Option<IcingaDowntimeName>,
    /// which scheduled downtime scheduled this downtime
    pub scheduled_by: Option<IcingaScheduledDowntimeName>,
    /// the parent downtime
    pub parent: Option<IcingaDowntimeName>,
    /// downtimes which should be triggered by this one
    pub triggers: Option<Vec<IcingaDowntimeName>>,
    /// TODO: what does this mean
    pub legacy_id: u64,
    /// when was this downtime removed
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub remove_time: Option<time::OffsetDateTime>,
    /// was this downtime canceled
    pub was_canceled: Option<bool>,
    /// TODO: what does this mean
    pub config_owner: Option<String>,
    /// TODO: what does this mean
    pub config_owner_hash: Option<String>,
    /// TODO: what does this mean
    pub authoritative_zone: Option<IcingaZoneName>,
}
