//! Command Type

use serde::{Deserialize, Serialize};

/// the type of command, used in the execute-command action
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "enumoid", derive(enumoid::Enumoid))]
pub enum IcingaCommandType {
    /// check command
    CheckCommand,
    /// event command
    EventCommand,
    /// notification command
    NotificationCommand,
}
