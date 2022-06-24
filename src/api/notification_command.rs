//! Icinga2 notification commands

use serde::Deserialize;

/// a notification command name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaNotificationCommandName(pub String);
