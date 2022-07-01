//! Event Stream Type: DowntimeAdded
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-stream-type-downtimeadded)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/apievents.cpp#L311=)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::runtime_objects::downtime::IcingaDowntime;

/// the DowntimeAdded event type
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaEventDowntimeAdded {
    /// when the event happened
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    pub timestamp: time::OffsetDateTime,
    /// the downtime this notification is about
    pub downtime: IcingaDowntime,
}
