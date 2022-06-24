//! Icinga2 user groups

use serde::Deserialize;

/// a user group name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaUserGroupName(pub String);
