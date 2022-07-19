//! ScheduledDowntime
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#scheduleddowntime)
//!
//! [Definition in the Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/scheduleddowntime.ti)

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::serde::{
    deserialize_optional_seconds_as_duration, serialize_optional_duration_as_seconds,
};
use crate::types::{
    common::custom_var_object::IcingaCustomVarObject,
    enums::{downtime_child_options::IcingaDowntimeChildOptions, object_type::IcingaObjectType},
    names::{IcingaHostName, IcingaServiceName},
};

/// a schedule repeating downtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcingaScheduledDowntime {
    /// type of icinga object, should always be ScheduledDowntime for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object and custom variable fields
    #[serde(flatten)]
    pub custom_var: IcingaCustomVarObject,
    /// the host for which the downtime will be scheduled
    pub host_name: IcingaHostName,
    /// the service for which the downtime will be scheduled, if not specified this is a host downtime
    pub service_name: Option<IcingaServiceName>,
    /// the author of the downtime
    pub author: String,
    /// the comment displayed for the downtime
    pub comment: String,
    /// the duration of the downtime
    #[serde(
        serialize_with = "serialize_optional_duration_as_seconds",
        deserialize_with = "deserialize_optional_seconds_as_duration"
    )]
    pub duration: Option<time::Duration>,
    /// is this a fixed downtime
    pub fixed: Option<bool>,
    /// how to apply this downtime to children
    pub child_options: Option<IcingaDowntimeChildOptions>,
    /// which days and durations apply to this timeperiod.
    pub ranges: BTreeMap<String, String>,
}
