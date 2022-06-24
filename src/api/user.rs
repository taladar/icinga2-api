//! Icinga2 users
use serde::Deserialize;

use super::IcingaJoinType;

/// a user name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaUserName(pub String);

/// possible joins parameter values for users
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaUserJoinTypes {
    /// the period object for which the user is valid (most likely something like shift or work hours)
    Period,
}

impl IcingaJoinType for IcingaUserJoinTypes {}

impl std::fmt::Display for IcingaUserJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaUserJoinTypes::Period => write!(f, "period"),
        }
    }
}

/// return type joins for users
#[derive(Debug, Deserialize)]
pub struct IcingaUserJoins {
    //pub period: Option<IcingaJoinResult<IcingaPeriodAttributes>>,
}
