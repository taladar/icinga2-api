//! Icinga2 time periods

use serde::Deserialize;

/// a time period name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaTimePeriodName(pub String);
