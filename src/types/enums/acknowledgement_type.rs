//! AcknowledgementType
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/checkable.ti)

use serde_repr::{Deserialize_repr, Serialize_repr};

/// type of acknowlegement in a checkable
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "enumoid", derive(enumoid::Enumoid))]
#[repr(u8)]
pub enum IcingaAcknowledgementType {
    /// no acknowledgement
    None = 0,
    /// normal acknowledgement
    Normal = 1,
    /// sticky acknowledgement
    Sticky = 2,
}
