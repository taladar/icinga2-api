//! User

use serde::{Serialize, Deserialize};

use super::{IcingaJoinType, IcingaJoinResult};

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
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaUserJoins {
    /// the time period when the user is active
    pub period: Option<IcingaJoinResult<IcingaTimePeriod>>,
}
