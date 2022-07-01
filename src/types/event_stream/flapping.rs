//! Event Stream Type: Flapping
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-stream-type-flapping)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/apievents.cpp#L155=)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::enums::host_or_service_state::IcingaHostOrServiceState;
use crate::types::enums::state_type::IcingaStateType;
use crate::types::names::{IcingaHostName, IcingaServiceName};

/// the Flapping event type
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaEventFlapping {
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
    /// the state of the host or service
    pub state: IcingaHostOrServiceState,
    /// the type of the state (soft/hard)
    pub state_type: IcingaStateType,
    /// is it flapping
    pub is_flapping: bool,
    /// current flapping value in percent
    pub flapping_current: f64,
    /// the flapping lower bound in percent for a host/service to be considered flapping
    pub flapping_threshold_low: f64,
    /// the flapping upper bound in percent for a host/service to be considered flapping
    pub flapping_threshold_high: f64,
}
