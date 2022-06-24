//! Icinga2 zones
use serde::Deserialize;

use super::IcingaJoinType;

/// possible joins parameter values for zones
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Debug, Deserialize)]
pub struct IcingaZoneJoins {
    //pub parent: Option<IcingaJoinResult<IcingaZoneAttributes>>,
}
