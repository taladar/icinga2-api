//! CheckCommand
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#checkcommand)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/checkcommand.ti)

use serde::{Deserialize, Serialize};

use crate::types::{common::command::IcingaCommand, enums::object_type::IcingaObjectType};

/// a check command (e.g. in a join)
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaCheckCommand {
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared fields in all command types
    #[serde(flatten)]
    pub command: IcingaCommand,
}
