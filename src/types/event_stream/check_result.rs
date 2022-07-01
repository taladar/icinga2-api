//! Event Stream Type: CheckResult
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-stream-type-checkresult)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/apievents.cpp#L37=)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::common::check_result::IcingaCheckResult;
use crate::types::names::{IcingaHostName, IcingaServiceName};

/// the CheckResult stream event
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaEventCheckResult {
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
    /// the actual check result
    pub check_result: IcingaCheckResult,
    /// number of active downtimes on the host/service
    pub downtime_depth: u64,
    /// is this acknowledged
    pub acknowledgement: bool,
}
