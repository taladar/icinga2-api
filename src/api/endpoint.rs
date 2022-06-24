//! Icinga2 endpoint

use serde::Deserialize;

/// an endpoint name
#[derive(Debug, Deserialize, derive_more::FromStr)]
pub struct IcingaEndpointName(pub String);
