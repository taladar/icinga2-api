//! Event Stream Type: StateChange
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-stream-type-statechange)
//!
//! [Definition in Icinga source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/apievents.cpp#L71=)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::common::check_result::IcingaCheckResult;
use crate::types::enums::host_or_service_state::IcingaHostOrServiceState;
use crate::types::enums::state_type::IcingaStateType;
use crate::types::names::{IcingaHostName, IcingaServiceName};

/// the StateChange event type
#[derive(Debug, Clone, Serialize, Deserialize)]
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
