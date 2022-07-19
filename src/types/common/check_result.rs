//! Icinga2 check result as it appears in various query results
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::serde::{
    deserialize_icinga_timestamp, deserialize_optional_seconds_as_duration,
    serialize_icinga_timestamp, serialize_optional_duration_as_seconds,
};
use crate::types::enums::{object_type::IcingaObjectType, service_state::IcingaServiceState};

use super::{command::IcingaCommandLine, performance_data::IcingaPerformanceData};

/// a check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcingaCheckResult {
    /// was this an active check
    pub active: bool,
    /// name of host which provided this check result
    pub check_source: String,
    /// the command called for the check
    pub command: Option<IcingaCommandLine>,
    /// start of command execution
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    pub execution_start: time::OffsetDateTime,
    /// end of command execution
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    pub execution_end: time::OffsetDateTime,
    /// exit status of the check command
    pub exit_status: u64,
    /// output of the check command
    pub output: String,
    /// performance data provided by the check command
    pub performance_data: Option<Vec<IcingaPerformanceData>>,
    /// hard state before this check
    pub previous_hard_state: IcingaServiceState,
    /// scheduled check execution start time
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    pub schedule_start: time::OffsetDateTime,
    /// scheduled check execution end time
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    pub schedule_end: time::OffsetDateTime,
    /// name of host which did the scheduling
    pub scheduling_source: String,
    /// state returned by this check
    pub state: IcingaServiceState,
    /// the TTL of this check result
    #[serde(default)]
    #[serde(
        serialize_with = "serialize_optional_duration_as_seconds",
        deserialize_with = "deserialize_optional_seconds_as_duration"
    )]
    pub ttl: Option<time::Duration>,
    /// the type of icinga object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// variables for internal calculations before this check
    pub vars_before: Option<BTreeMap<String, serde_json::Value>>,
    /// variables for internal calculations after this check
    pub vars_after: Option<BTreeMap<String, serde_json::Value>>,
}
