//! Event Stream Type: AcknowledgementSet
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-stream-type-acknowledgementset)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/apievents.cpp#L191=)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::enums::acknowledgement_type::IcingaAcknowledgementType;
use crate::types::enums::host_or_service_state::IcingaHostOrServiceState;
use crate::types::enums::state_type::IcingaStateType;
use crate::types::names::{IcingaHostName, IcingaServiceName};

/// the AcknowledgementSet event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IcingaEventAcknowledgementSet {
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
    /// the author of the acknowledgement
    pub author: String,
    /// the text of the acknowledgement comment
    pub comment: String,
    /// the type of acknowledgement
    pub acknowledgement_type: IcingaAcknowledgementType,
    /// Whether a notification of the Acknowledgement type will be sent.
    pub notify: bool,
    /// whether the comment persists after expiry of this acknowledgement
    pub persistent: bool,
    /// when the notification will expire
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    pub expiry: time::OffsetDateTime,
}
