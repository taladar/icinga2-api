//! Event Stream Type: CommentAdded
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-stream-type-commentadded)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/apievents.cpp#L265=)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::runtime_objects::comment::IcingaComment;

/// the CommentAdded event type
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaEventCommentAdded {
    /// when the event happened
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    pub timestamp: time::OffsetDateTime,
    /// the comment this notification is about
    pub comment: IcingaComment,
}
