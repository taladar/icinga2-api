//! Event Stream Type: DowntimeAdded
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-stream-type-downtimeadded)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/apievents.cpp#L311=)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::runtime_objects::downtime::IcingaDowntime;

/// the DowntimeAdded event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IcingaEventDowntimeAdded {
    /// when the event happened
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    pub timestamp: time::OffsetDateTime,
    /// the downtime this notification is about
    pub downtime: IcingaDowntime,
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use pretty_assertions::assert_eq;

    use crate::types::event_stream::IcingaEvent;
    use crate::types::names::IcingaServiceName;

    /// A DowntimeAdded event as Icinga sends it: the downtime is serialized with
    /// only its config and state attributes, so the config object's `active`,
    /// `paused` and `ha_mode` are missing.
    const EVENT: &str = r#"{"downtime":{"__name":"host1!service1!3b5e2f6a-1c1e-4d4f-9a59-2b6c1d7e8f90","author":"admin","authoritative_zone":"","comment":"maintenance","config_owner":"","config_owner_hash":"","duration":0,"end_time":1793362419,"entry_time":1793358819.42,"fixed":true,"host_name":"host1","legacy_id":12,"name":"3b5e2f6a-1c1e-4d4f-9a59-2b6c1d7e8f90","package":"_api","parent":"","remove_time":0,"scheduled_by":"","service_name":"service1","source_location":{"first_column":0,"first_line":1,"last_column":69,"last_line":1,"path":"/var/lib/icinga2/api/packages/_api/conf.d/downtimes/host1!service1!3b5e2f6a.conf"},"start_time":1793358819,"templates":["3b5e2f6a-1c1e-4d4f-9a59-2b6c1d7e8f90"],"trigger_time":0,"triggered_by":"","triggers":[],"type":"Downtime","version":1793358819.42,"zone":"master"},"timestamp":1793358819.43,"type":"DowntimeAdded"}"#;

    #[test]
    fn decodes_downtime_without_runtime_only_attributes() -> Result<(), Box<dyn Error>> {
        let jd = &mut serde_json::Deserializer::from_str(EVENT);
        let event: IcingaEvent = serde_path_to_error::deserialize(jd)?;
        let IcingaEvent::DowntimeAdded(event) = event else {
            return Err(format!("expected a DowntimeAdded event, got {event:?}").into());
        };
        assert_eq!(event.downtime.config_object.active, None);
        assert_eq!(event.downtime.config_object.paused, None);
        assert_eq!(event.downtime.config_object.ha_mode, None);
        assert_eq!(event.downtime.author, "admin");
        assert_eq!(
            event.downtime.service_name,
            Some(IcingaServiceName("service1".to_owned()))
        );
        Ok(())
    }

    #[test]
    fn host_downtime_has_no_service() -> Result<(), Box<dyn Error>> {
        // Icinga sends the (unset) service name of a host downtime as an empty string.
        let host_event = EVENT
            .replace(r#""service_name":"service1""#, r#""service_name":"""#)
            .replace("host1!service1!", "host1!");
        let jd = &mut serde_json::Deserializer::from_str(&host_event);
        let event: IcingaEvent = serde_path_to_error::deserialize(jd)?;
        let IcingaEvent::DowntimeAdded(event) = event else {
            return Err(format!("expected a DowntimeAdded event, got {event:?}").into());
        };
        assert_eq!(event.downtime.service_name, None);
        Ok(())
    }
}
