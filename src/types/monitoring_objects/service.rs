//! Service
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/09-object-types/#service)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/icinga/service.ti)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_optional_icinga_timestamp, serialize_optional_icinga_timestamp};
use crate::types::common::custom_var_object::CustomVarHolder;
use crate::types::{
    common::checkable::IcingaCheckable,
    enums::{object_type::IcingaObjectType, service_state::IcingaServiceState},
    names::{IcingaHostName, IcingaServiceGroupName},
};

/// an Icinga monitored service
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IcingaService {
    /// type of icinga object, should always be Service for this
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
    /// all the attributes from the icinga checkable object (shared fields between host and service)
    #[serde(flatten)]
    pub checkable: IcingaCheckable,
    /// a short description of the service
    pub display_name: String,
    /// a list of groups the service belongs to
    pub groups: Vec<IcingaServiceGroupName>,
    /// the hostname for this service
    pub host_name: IcingaHostName,
    /// the previous hard state
    pub last_hard_state: IcingaServiceState,
    /// the previous state
    pub last_state: IcingaServiceState,
    /// when the last CRITICAL state occurred
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_state_critical: Option<time::OffsetDateTime>,
    /// when the last OK state occurred
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_state_ok: Option<time::OffsetDateTime>,
    /// when the last UNKNOWN state occurred
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_state_unknown: Option<time::OffsetDateTime>,
    /// when the last WARNINGE state occurred
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub last_state_warning: Option<time::OffsetDateTime>,
    /// the current state
    pub state: IcingaServiceState,
}

impl CustomVarHolder for IcingaService {
    fn custom_var_value(&self, name: &str) -> Option<&serde_json::Value> {
        self.checkable.custom_var_value(name)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    /// a service that has never been checked, as the Icinga API returns it
    /// (with `last_check_result` and a few other fields `null`)
    const NEVER_CHECKED_SERVICE: &str = r#"{"__name":"host.example.com!subscription_status","acknowledgement":0,"acknowledgement_expiry":0,"acknowledgement_last_change":0,"action_url":"","active":true,"check_attempt":1,"check_command":"subscription_status","check_interval":86400,"check_period":"","check_timeout":null,"command_endpoint":"host.example.com","display_name":"Subscription Status","downtime_depth":0,"enable_active_checks":true,"enable_event_handler":true,"enable_flapping":false,"enable_notifications":true,"enable_passive_checks":true,"enable_perfdata":true,"event_command":"","executions":null,"flapping":false,"flapping_current":0,"flapping_ignore_states":null,"flapping_last_change":0,"flapping_threshold":0,"flapping_threshold_high":30,"flapping_threshold_low":25,"force_next_check":false,"force_next_notification":false,"groups":["Support status"],"ha_mode":0,"handled":false,"host_name":"host.example.com","icon_image":"","icon_image_alt":"","last_check":-1,"last_check_result":null,"last_hard_state":3,"last_hard_state_change":0,"last_reachable":true,"last_state":3,"last_state_change":0,"last_state_critical":0,"last_state_ok":0,"last_state_type":0,"last_state_unknown":0,"last_state_unreachable":0,"last_state_warning":0,"max_check_attempts":3,"name":"subscription_status","next_check":1795610411.7982948,"next_update":1795696811.7982948,"notes":"","notes_url":"","original_attributes":null,"package":"_etc","paused":false,"previous_state_change":0,"problem":false,"retry_interval":60,"severity":16,"source_location":{"first_column":1,"first_line":1298,"last_column":36,"last_line":1298,"path":"/etc/icinga2/conf.d/host.example.com.conf"},"state":3,"state_type":0,"templates":["subscription_status"],"type":"Service","vars":{"serviceGroups":["Support status"],"serviceImportance":"dev","serviceSeverity":"minor","serviceUrgency":"no"},"version":0,"volatile":false,"zone":"master.example.com"}"#;

    #[test]
    fn test_deserialize_never_checked_service() -> Result<(), Box<dyn std::error::Error>> {
        let service: IcingaService = serde_json::from_str(NEVER_CHECKED_SERVICE)?;
        assert_eq!(service.checkable.last_check_result, None);
        assert_eq!(service.host_name.to_string(), "host.example.com");
        Ok(())
    }
}
