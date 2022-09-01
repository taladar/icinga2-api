//! StateType
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/checkresult.ti)

use serde_repr::{Deserialize_repr, Serialize_repr};

/// which state type we are dealing with
#[derive(
    Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "enumoid", derive(enumoid::Enumoid))]
#[repr(u8)]
pub enum IcingaStateType {
    /// soft state (recently changed)
    Soft = 0,
    /// hard state (no recent changes)
    Hard = 1,
}
