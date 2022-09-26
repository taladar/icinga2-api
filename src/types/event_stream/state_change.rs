//! Event Stream Type: StateChange
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-stream-type-statechange)
//!
//! [Definition in Icinga source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/apievents.cpp#L71=)

use serde::{Deserialize, Serialize};

use thiserror::Error;

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::common::check_result::IcingaCheckResult;
use crate::types::enums::host_or_service_state::IcingaHostOrServiceState;
use crate::types::enums::host_state::IcingaHostState;
use crate::types::enums::service_state::IcingaServiceState;
use crate::types::enums::state_type::IcingaStateType;
use crate::types::names::{IcingaHostName, IcingaServiceName};

/// the StateChange event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(
    try_from = "IcingaEventStateChangeNumericState",
    into = "IcingaEventStateChangeNumericState"
)]
pub struct IcingaEventStateChange {
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
    /// the new state of the host or service
    pub state: IcingaHostOrServiceState,
    /// the type of the new state (soft/hard)
    pub state_type: IcingaStateType,
    /// the related check result
    pub check_result: IcingaCheckResult,
    /// number of active downtimes on the host/service
    pub downtime_depth: u64,
    /// is this acknowledged
    pub acknowledgement: bool,
}

/// error for the conversion from a numeric value to a host or service state
#[derive(Debug, Clone, Error)]
pub enum NumericToHostOrServiceStateError {
    /// the numeric value has no known meaning as a host state
    #[error("unknown numeric value to interpret as host state: {0}")]
    UnknownNumericHostState(u64),
    /// the numeric value has no known meaning as a service state
    #[error("unknown numeric value to interpret as service state: {0}")]
    UnknownNumericServiceState(u64),
}

impl TryFrom<IcingaEventStateChangeNumericState> for IcingaEventStateChange {
    type Error = NumericToHostOrServiceStateError;

    fn try_from(value: IcingaEventStateChangeNumericState) -> Result<Self, Self::Error> {
        let state = match (value.state, &value.service) {
            (0, None) => IcingaHostOrServiceState::Host(IcingaHostState::Up),
            (1, None) => IcingaHostOrServiceState::Host(IcingaHostState::Down),
            (2, None) => IcingaHostOrServiceState::Host(IcingaHostState::Unreachable),
            (n, None) => {
                return Err(NumericToHostOrServiceStateError::UnknownNumericHostState(n));
            }
            (0, Some(_)) => IcingaHostOrServiceState::Service(IcingaServiceState::Ok),
            (1, Some(_)) => IcingaHostOrServiceState::Service(IcingaServiceState::Warning),
            (2, Some(_)) => IcingaHostOrServiceState::Service(IcingaServiceState::Critical),
            (3, Some(_)) => IcingaHostOrServiceState::Service(IcingaServiceState::Unknown),
            (4, Some(_)) => IcingaHostOrServiceState::Service(IcingaServiceState::Unreachable),
            (99, Some(_)) => IcingaHostOrServiceState::Service(IcingaServiceState::Pending),
            (n, Some(_)) => {
                return Err(NumericToHostOrServiceStateError::UnknownNumericServiceState(n));
            }
        };
        Ok(IcingaEventStateChange {
            timestamp: value.timestamp,
            host: value.host,
            service: value.service,
            state,
            state_type: value.state_type,
            check_result: value.check_result,
            downtime_depth: value.downtime_depth,
            acknowledgement: value.acknowledgement,
        })
    }
}

/// helper type to deserialize [IcingaEventStateChange] with a numeric state
/// and then convert it to the correct enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IcingaEventStateChangeNumericState {
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
    /// the new state of the host or service
    pub state: u64,
    /// the type of the new state (soft/hard)
    pub state_type: IcingaStateType,
    /// the related check result
    pub check_result: IcingaCheckResult,
    /// number of active downtimes on the host/service
    pub downtime_depth: u64,
    /// is this acknowledged
    pub acknowledgement: bool,
}

impl From<IcingaEventStateChange> for IcingaEventStateChangeNumericState {
    fn from(value: IcingaEventStateChange) -> Self {
        let state = match value.state {
            IcingaHostOrServiceState::Host(IcingaHostState::Up) => 0,
            IcingaHostOrServiceState::Host(IcingaHostState::Down) => 1,
            IcingaHostOrServiceState::Host(IcingaHostState::Unreachable) => 2,
            IcingaHostOrServiceState::Service(IcingaServiceState::Ok) => 0,
            IcingaHostOrServiceState::Service(IcingaServiceState::Warning) => 1,
            IcingaHostOrServiceState::Service(IcingaServiceState::Critical) => 2,
            IcingaHostOrServiceState::Service(IcingaServiceState::Unknown) => 3,
            IcingaHostOrServiceState::Service(IcingaServiceState::Unreachable) => 4,
            IcingaHostOrServiceState::Service(IcingaServiceState::Pending) => 99,
        };
        IcingaEventStateChangeNumericState {
            timestamp: value.timestamp,
            host: value.host,
            service: value.service,
            state,
            state_type: value.state_type,
            check_result: value.check_result,
            downtime_depth: value.downtime_depth,
            acknowledgement: value.acknowledgement,
        }
    }
}
