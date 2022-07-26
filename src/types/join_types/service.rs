//! Service

use serde::{Deserialize, Serialize};

use crate::types::monitoring_objects::{
    check_command::IcingaCheckCommand, endpoint::IcingaEndpoint, event_command::IcingaEventCommand,
    host::IcingaHost, time_period::IcingaTimePeriod,
};

use super::{IcingaJoinResult, IcingaJoinType};

/// possible joins parameter values for services
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaServiceJoinTypes {
    /// the host the service is on
    Host,
    /// the check command object for the service
    CheckCommand,
    /// the check period object for the service
    CheckPeriod,
    /// the event command object for the service
    EventCommand,
    /// the command endpoint object for the service
    CommandEndpoint,
}

impl IcingaJoinType for IcingaServiceJoinTypes {}

impl std::fmt::Display for IcingaServiceJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaServiceJoinTypes::Host => write!(f, "host"),
            IcingaServiceJoinTypes::CheckCommand => write!(f, "check_command"),
            IcingaServiceJoinTypes::CheckPeriod => write!(f, "check_period"),
            IcingaServiceJoinTypes::EventCommand => write!(f, "event_command"),
            IcingaServiceJoinTypes::CommandEndpoint => write!(f, "command_endpoint"),
        }
    }
}

/// return type joins for services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcingaServiceJoins {
    /// the host this service is on
    pub host: Option<IcingaJoinResult<IcingaHost>>,
    /// the check command object for the service
    pub check_command: Option<IcingaJoinResult<IcingaCheckCommand>>,
    /// the time period when the service is checked
    pub check_period: Option<IcingaJoinResult<IcingaTimePeriod>>,
    /// the event command run on state change
    pub event_command: Option<IcingaJoinResult<IcingaEventCommand>>,
    /// the command endpoint for the commands
    pub command_endpoint: Option<IcingaJoinResult<IcingaEndpoint>>,
}
