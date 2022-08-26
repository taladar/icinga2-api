//! Endpoint
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#endpoint)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/remote/endpoint.ti)

use serde::{Deserialize, Serialize};

use crate::serde::{
    deserialize_empty_string_or_string, deserialize_optional_icinga_timestamp,
    deserialize_optional_seconds_as_duration, serialize_none_as_empty_string,
    serialize_optional_duration_as_seconds, serialize_optional_icinga_timestamp,
};
use crate::types::{
    common::config_object::IcingaConfigObject, enums::object_type::IcingaObjectType,
};

/// an endpoint to which icinga can connect (host, port,...)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IcingaEndpoint {
    /// type of icinga object, should always be Endpoint for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared config object fields
    #[serde(flatten)]
    pub config_object: IcingaConfigObject,
    /// the host to connect to
    #[serde(
        serialize_with = "serialize_none_as_empty_string",
        deserialize_with = "deserialize_empty_string_or_string"
    )]
    pub host: Option<String>,
    /// the port to connect to
    pub port: String,
    /// how long to keep the replay logs on connection loss, disabled if set to 0
    #[serde(
        serialize_with = "serialize_optional_duration_as_seconds",
        deserialize_with = "deserialize_optional_seconds_as_duration"
    )]
    pub log_duration: Option<time::Duration>,
    /// log position on the local side
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub local_log_position: Option<time::OffsetDateTime>,
    /// log position on the remote side
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub remote_log_position: Option<time::OffsetDateTime>,
    /// remote icinga version
    pub icinga_version: Option<u64>,
    /// remote icinga capabilities
    pub capabilities: Option<u64>,
    /// is this endpoint currently connecting
    pub connecting: Option<bool>,
    /// is this endpoint currently syncing
    pub syncing: Option<bool>,
    /// is this endpoint currently connected
    pub connected: Option<bool>,
    /// time when the last message was sent to this endpoint
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_message_sent: Option<time::OffsetDateTime>,
    /// time when the last message was received from this endpoint
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_message_received: Option<time::OffsetDateTime>,
    /// frequency of message sending on this endpoint
    pub messages_send_per_second: Option<f64>,
    /// frequency of message reception on this endpoint
    pub messages_received_per_second: Option<f64>,
    /// bandwidth used sending on this endpoint
    pub bytes_sent_per_second: Option<f64>,
    /// bandwidth used receiving on this endpoint
    pub bytes_received_per_second: Option<f64>,
}
