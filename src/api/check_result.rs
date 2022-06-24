//! Icinga2 check result as it appears in various query results
use serde::Deserialize;

use crate::enums::{IcingaObjectType, IcingaStateType};

use super::{
    command::IcingaCommand, performance_data::IcingaPerformanceData, service::IcingaServiceState,
};
use crate::serde::{deserialize_icinga_timestamp, deserialize_optional_seconds_as_duration};

/// variables in check result (seem to be very static)
#[derive(Debug, Deserialize)]
pub struct IcingaCheckResultVars {
    /// used for internal calculations
    pub attempt: u64,
    /// used for internal calculations
    pub reachable: bool,
    /// used for internal calculations
    pub state: IcingaServiceState,
    /// used for internal calculations
    pub state_type: IcingaStateType,
}

/// a check result
#[derive(Debug, Deserialize)]
pub struct IcingaCheckResult {
    /// was this an active check
    pub active: bool,
    /// name of host which provided this check result
    pub check_source: String,
    /// the command called for the check
    pub command: Option<IcingaCommand>,
    /// start of command execution
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub execution_start: time::OffsetDateTime,
    /// end of command execution
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
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
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub schedule_start: time::OffsetDateTime,
    /// scheduled check execution end time
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub schedule_end: time::OffsetDateTime,
    /// name of host which did the scheduling
    pub scheduling_source: String,
    /// state returned by this check
    pub state: IcingaServiceState,
    /// the TTL of this check result
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub ttl: Option<time::Duration>,
    /// the type of icinga object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// variables for internal calculations before this check
    pub vars_before: Option<IcingaCheckResultVars>,
    /// variables for internal calculations after this check
    pub vars_after: Option<IcingaCheckResultVars>,
}
