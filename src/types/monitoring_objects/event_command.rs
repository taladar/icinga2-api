//! EventCommand
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#eventcommand)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/eventcommand.ti)

use serde::{Deserialize, Serialize};

use crate::types::{common::command::IcingaCommand, enums::object_type::IcingaObjectType};

/// an event command (e.g. in a join)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcingaEventCommand {
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared fields in all command types
    #[serde(flatten)]
    pub command: IcingaCommand,
}
