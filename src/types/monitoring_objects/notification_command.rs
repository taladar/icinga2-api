//! NotificationCommand
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#notificationcommand)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/notificationcommand.ti)

use serde::{Deserialize, Serialize};

use crate::types::{
    common::{command::IcingaCommand, custom_var_object::CustomVarHolder},
    enums::object_type::IcingaObjectType,
};

/// a notification command
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IcingaNotificationCommand {
    /// type of object
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// shared fields in all command types
    #[serde(flatten)]
    pub command: IcingaCommand,
}

impl CustomVarHolder for IcingaNotificationCommand {
    fn custom_var_value(&self, name: &str) -> Option<&serde_json::Value> {
        self.command.custom_var_value(name)
    }
}
