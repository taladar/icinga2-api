//! DowntimeChildOptions
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/downtime.hpp)

use serde::{Deserialize, Serialize};

/// what to do with children when a downtime starts
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "enumoid", derive(enumoid::Enumoid))]
pub enum IcingaDowntimeChildOptions {
    /// downtime does not affect children
    DowntimeNoChildren,
    /// schedules child downtimes triggered by this downtime
    DowntimeTriggeredChildren,
    /// schedules non-triggered downtimes
    #[serde(rename = "DowntimeNonTriggeredChildren")]
    DOwntimeNonTriggeredChildren,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_serialize_as_icinga_names() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            serde_json::to_string(&IcingaDowntimeChildOptions::DOwntimeNonTriggeredChildren)?,
            "\"DowntimeNonTriggeredChildren\""
        );
        assert_eq!(
            serde_json::from_str::<IcingaDowntimeChildOptions>("\"DowntimeNonTriggeredChildren\"")?,
            IcingaDowntimeChildOptions::DOwntimeNonTriggeredChildren
        );
        assert_eq!(
            serde_json::to_string(&IcingaDowntimeChildOptions::DowntimeTriggeredChildren)?,
            "\"DowntimeTriggeredChildren\""
        );
        Ok(())
    }
}
