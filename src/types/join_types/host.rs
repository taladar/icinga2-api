//! Host

use serde::{Serialize, Deserialize};

use super::{IcingaJoinType, IcingaJoinResult};

/// possible joins parameter values for hosts
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaHostJoinTypes {
    /// the check command object for the host
    CheckCommand,
    /// the check period object for the host
    CheckPeriod,
    /// the event command object for the host
    EventCommand,
    /// the command endpoint object for the host
    CommandEndpoint,
}

impl IcingaJoinType for IcingaHostJoinTypes {}

impl std::fmt::Display for IcingaHostJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaHostJoinTypes::CheckCommand => write!(f, "check_command"),
            IcingaHostJoinTypes::CheckPeriod => write!(f, "check_period"),
            IcingaHostJoinTypes::EventCommand => write!(f, "event_command"),
            IcingaHostJoinTypes::CommandEndpoint => write!(f, "command_endpoint"),
        }
    }
}

/// return type joins for hosts
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaHostJoins {
    /// the check command object for the host
    pub check_command: Option<IcingaJoinResult<IcingaCheckCommand>>,
    pub check_period: Option<IcingaJoinResult<IcingaTimePeriod>>,
    pub event_command: Option<IcingaJoinResult<IcingaEventCommand>>,
    pub command_endpoint: Option<IcingaJoinResult<IcingaCommandEndpoint>>,
}
