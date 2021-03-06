//! CommentType
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/comment.ti)

use serde_repr::{Deserialize_repr, Serialize_repr};

/// the type of comment in Icinga
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IcingaCommentType {
    /// a comment by a user directly
    CommentUser = 1,
    /// a comment created as part of an acknowledgement
    CommentAcknowledgement = 4,
}
