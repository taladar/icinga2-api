//! Command Type

use serde::{Deserialize, Serialize};

/// the type of command, used in the execute-command action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IcingaCommandType {
    /// check command
    CheckCommand,
    /// event command
    EventCommand,
    /// notification command
    NotificationCommand,
}
