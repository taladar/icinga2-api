//! Event Stream Type: ObjectDeleted
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/apievents.cpp#L403=)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::enums::object_type::IcingaObjectType;

/// the ObjectDeleted event type
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaEventObjectDeleted {
    /// when the event happened
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    pub timestamp: time::OffsetDateTime,
    /// the object type of the object that changed
    pub object_type: IcingaObjectType,
    /// the object name of the object that changed
    pub object_name: String,
}
