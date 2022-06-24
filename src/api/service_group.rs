//! Icinga2 service groups

use serde::Deserialize;

/// a service group name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaServiceGroupName(pub String);
