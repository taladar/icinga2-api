//! Various small enums

use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::api::host::{IcingaHostState, IcingaHostStateByName};
use crate::api::service::{IcingaServiceState, IcingaServiceStateByName};

/// which state type we are dealing with
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum IcingaStateType {
    /// soft state (recently changed)
    Soft = 0,
    /// hard state (no recent changes)
    Hard = 1,
}

/// HA mode
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum HAMode {
    /// run a check once
    HARunOnce,
    /// run a check everywhere
    HARunEverywhere,
}

/// HostState and ServiceState
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaHostOrServiceState {
    /// a host state
    Host(#[serde(with = "IcingaHostStateByName")] IcingaHostState),
    /// a service state
    Service(#[serde(with = "IcingaServiceStateByName")] IcingaServiceState),
}

/// the type of icinga object we are dealing with
#[derive(Debug, Deserialize)]
pub enum IcingaObjectType {
    /// an icinga monitored host
    Host,
    /// an icinga service
    Service,
    /// an icinga check result
    CheckResult,
    /// a performance data value
    PerfdataValue,
    /// an icinga comment
    Comment,
    /// an icinga dependency between hosts or services
    Dependency,
    /// an icinga notification
    Notification,
    /// a function
    Function,
    /// a check command
    CheckCommand,
    /// a notification command
    NotificationCommand,
    /// an event command
    EventCommand,
    /// a host group
    HostGroup,
    /// a service group
    ServiceGroup,
    /// a user group
    UserGroup,
}

/// acknowledgement type
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum IcingaAcknowledgementType {
    /// no acknowledgement
    None = 0,
    /// normal acknowledgement
    Normal = 1,
    /// sticky acknowledgement
    Sticky = 2,
}
