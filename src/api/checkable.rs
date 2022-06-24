//! shared fields in various icinga2 api objects

use serde::Deserialize;

use crate::{
    enums::{IcingaAcknowledgementType, IcingaHostOrServiceState, IcingaStateType},
    serde::{
        deserialize_empty_string_or_string, deserialize_icinga_timestamp,
        deserialize_optional_icinga_timestamp, deserialize_optional_seconds_as_duration,
    },
};

use super::check_result::IcingaCheckResult;

/// shared attributes on any checkable object (host and service)
#[derive(Debug, Deserialize)]
pub struct IcingaCheckable {
    /// the type of acknowledgement (includes None)
    pub acknowledgement: IcingaAcknowledgementType,
    /// when the acknowledgement expires
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub acknowledgement_expiry: Option<time::OffsetDateTime>,
    /// when the acknowledgement last changed
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub acknowledgement_last_change: Option<time::OffsetDateTime>,
    /// URL for actions for the checkable (host or service)
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub action_url: Option<String>,
    /// the current check attempt number
    pub check_attempt: u64,
    /// the name of the check command
    pub check_command: String,
    /// the interval used for checks when the host/service is in a HARD state
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub check_interval: Option<time::Duration>,
    /// name of a time period when this host/service is checked
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub check_period: Option<String>,
    /// check timeout
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub check_timeout: Option<time::Duration>,
    /// the endpoint the command is executed on
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub command_endpoint: Option<String>,
    /// number of active downtimes on the host/service
    pub downtime_depth: u64,
    /// whether active checks are enabled
    pub enable_active_checks: bool,
    /// enabled event handlers for this host/service
    pub enable_event_handler: bool,
    /// whether flap detection is enabled
    pub enable_flapping: bool,
    /// whether notifications are enabled
    pub enable_notifications: bool,
    /// whether passive checks are enabled
    pub enable_passive_checks: bool,
    /// whether performance data processing is enabled
    pub enable_perfdata: bool,
    /// the name of an event command that should be executed every time the host/service state changes or the host/service is in a SOFT state
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub event_command: Option<String>,
    /// contains the state of execute-command executions
    pub executions: Option<()>,
    /// whether the host/service is flapping between states
    pub flapping: bool,
    /// current flapping value in percent
    pub flapping_current: f64,
    /// a list of states that should be ignored during flapping calculations
    #[serde(default)]
    pub flapping_ignore_states: Option<Vec<IcingaHostOrServiceState>>,
    /// when the last flapping change occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub flapping_last_change: Option<time::OffsetDateTime>,
    /// deprecated and has no effect, replaced by flapping_threshold_low and flapping_threshold_high
    pub flapping_threshold: f64,
    /// the flapping lower bound in percent for a host/service to be considered flapping
    pub flapping_threshold_low: f64,
    /// the flapping upper bound in percent for a host/service to be considered flapping
    pub flapping_threshold_high: f64,
    /// force the next check (execute it now)
    pub force_next_check: bool,
    /// force next notification (send it now)
    pub force_next_notification: bool,
    /// whether the host/service problem is handled (downtime or acknowledgement)
    pub handled: bool,
    /// icon image for the host/service
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub icon_image: Option<String>,
    /// icon image alt text for the host/service
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub icon_image_alt: Option<String>,
    /// when the last check occurred
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub last_check: time::OffsetDateTime,
    /// the result of the last check
    pub last_check_result: IcingaCheckResult,
    /// when the last hard state change occurred
    #[serde(deserialize_with = "deserialize_icinga_timestamp")]
    pub last_hard_state_change: time::OffsetDateTime,
    /// whether the host/service was reachable when the last check occurred
    pub last_reachable: bool,
    /// when the last state change occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_change: Option<time::OffsetDateTime>,
    /// the previous state type (soft/hard)
    pub last_state_type: IcingaStateType,
    /// when the last UNREACHABLE state occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub last_state_unreachable: Option<time::OffsetDateTime>,
    /// the number of times the host/service is checked before changing into a new hard state
    pub max_check_attempts: u64,
    /// when the next check occurs
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub next_check: Option<time::OffsetDateTime>,
    /// when the next check update is to be expected
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub next_update: Option<time::OffsetDateTime>,
    /// notes for the host/service
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub notes: Option<String>,
    /// URL for notes for the host/service
    #[serde(deserialize_with = "deserialize_empty_string_or_string")]
    pub notes_url: Option<String>,
    /// when the previous state change occurred
    #[serde(deserialize_with = "deserialize_optional_icinga_timestamp")]
    pub previous_state_change: Option<time::OffsetDateTime>,
    /// whether the host/service is considered to be in a problem state type (not up)
    pub problem: bool,
    /// the interval used for checks when the host/service is in a SOFT state
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_seconds_as_duration")]
    pub retry_interval: Option<time::Duration>,
    /// pre-calculated value, higher means more severe
    pub severity: u64,
    /// the current state type (soft/hard)
    pub state_type: IcingaStateType,
    /// treat all state changes as HARD changes
    pub volatile: bool,
}
