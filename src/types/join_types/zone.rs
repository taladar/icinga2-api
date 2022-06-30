//! Zone

use serde::{Deserialize, Serialize};

use crate::types::monitoring_objects::zone::IcingaZone;

use super::{IcingaJoinResult, IcingaJoinType};

/// possible joins parameter values for zones
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaZoneJoinTypes {
    /// the parent zone object
    Parent,
}

impl IcingaJoinType for IcingaZoneJoinTypes {}

impl std::fmt::Display for IcingaZoneJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaZoneJoinTypes::Parent => write!(f, "parent"),
        }
    }
}

/// return type joins for zones
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaZoneJoins {
    /// the parent zone
    pub parent: Option<IcingaJoinResult<IcingaZone>>,
}
