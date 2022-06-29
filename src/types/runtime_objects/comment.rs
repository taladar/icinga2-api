//! Comment
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#comment)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/comment.ti)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_optional_icinga_timestamp, serialize_optional_icinga_timestamp};
use crate::types::enums::comment_type::IcingaCommentType;
use crate::types::{
    common::config_object::IcingaConfigObject,
    enums::object_type::IcingaObjectType,
    names::{IcingaHostName, IcingaServiceName},
};

/// an icinga comment
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaComment {
    /// type of icinga object, should always be Comment for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object fields
    #[serde(flatten)]
    pub config_object: IcingaConfigObject,
    /// the host this comment is attached to
    pub host_name: IcingaHostName,
    /// the service this comment is attached to, if not specified it is a host comment
    pub service_name: Option<IcingaServiceName>,
    /// the time the comment was entered
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub entry_time: Option<time::OffsetDateTime>,
    /// the type of comment (user or acknowledgement)
    pub entry_type: Option<IcingaCommentType>,
    /// the comment author
    pub author: String,
    /// the comment body
    pub text: String,
    /// Only evaluated for entry_type Acknowledgement. true does not remove the comment when the acknowledgement is removed.
    pub persistent: Option<bool>,
    /// the comment will expire at this time
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub expire_time: Option<time::OffsetDateTime>,
    /// TODO: what does this mean?
    pub legacy_id: u64,
}
