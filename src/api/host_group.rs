//! Icinga2 host groups

use serde::Deserialize;

/// a host group name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaHostGroupName(pub String);
