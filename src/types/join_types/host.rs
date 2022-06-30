//! Host

use serde::{Deserialize, Serialize};

use crate::types::monitoring_objects::{
    check_command::IcingaCheckCommand, endpoint::IcingaEndpoint, event_command::IcingaEventCommand,
    time_period::IcingaTimePeriod,
};

use super::{IcingaJoinResult, IcingaJoinType};

/// possible joins parameter values for hosts
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    /// the check period when the host is expected to be up
    pub check_period: Option<IcingaJoinResult<IcingaTimePeriod>>,
    /// the event command that is called when the host state changes
    pub event_command: Option<IcingaJoinResult<IcingaEventCommand>>,
    /// the endpoint for the commands
    pub command_endpoint: Option<IcingaJoinResult<IcingaEndpoint>>,
}
