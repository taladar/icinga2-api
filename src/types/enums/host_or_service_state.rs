//! HostState or ServiceState by name

use serde::{Serialize, Deserialize};

use super::host_state::IcingaHostState;
use super::host_state::IcingaHostStateByName;
use super::service_state::IcingaServiceState;
use super::service_state::IcingaServiceStateByName;

/// HostState and ServiceState, serialized/deserialized by name
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IcingaHostOrServiceState {
    /// a host state
    Host(#[serde(with = "IcingaHostStateByName")] IcingaHostState),
    /// a service state
    Service(#[serde(with = "IcingaServiceStateByName")] IcingaServiceState),
}
