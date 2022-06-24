//! Icinga2 event command

use serde::Deserialize;

/// an event command name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaEventCommandName(pub String);
