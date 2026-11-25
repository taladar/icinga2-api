//! API Action schedule-downtime
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#schedule-downtime)

use serde::{Deserialize, Serialize};

use crate::serde::{
    deserialize_icinga_timestamp, deserialize_optional_seconds_as_duration,
    serialize_icinga_timestamp, serialize_optional_duration_as_seconds,
};
use crate::types::action::StatusResponse;
use crate::types::enums::downtime_child_options::IcingaDowntimeChildOptions;
use crate::types::enums::object_type::IcingaObjectType;
use crate::types::filter::IcingaFilter;
use crate::types::names::IcingaDowntimeName;
use crate::types::query::ResultsWrapper;
use crate::types::rest::{RestApiEndpoint, RestApiResponse};

/// REST API Endpoint for the schedule-downtime call
#[derive(Debug, Clone, derive_builder::Builder, Serialize, Deserialize)]
#[builder(
    build_fn(error = "crate::error::Error", validate = "Self::validate"),
    derive(Debug)
)]
pub struct ScheduleDowntime {
    /// the author of the downtime
    author: String,
    /// the body of the downtime comment
    comment: String,
    /// beginning of the downtime
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    start_time: time::OffsetDateTime,
    /// end of the downtime
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    end_time: time::OffsetDateTime,
    /// Defaults to true. If true, the downtime is fixed otherwise flexible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fixed: Option<bool>,
    /// the duration of the downtime
    #[serde(
        serialize_with = "serialize_optional_duration_as_seconds",
        deserialize_with = "deserialize_optional_seconds_as_duration"
    )]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    duration: Option<time::Duration>,
    /// Sets downtime for all services for the matched host objects. If child_options are set, all child hosts and their services will schedule a downtime too. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    all_services: Option<bool>,
    /// Sets the trigger for a triggered downtime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    trigger_name: Option<IcingaDowntimeName>,
    /// Schedule child downtimes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    child_options: Option<IcingaDowntimeChildOptions>,
    /// filter to target which host and/or service to schedule a downtime for
    #[builder(default, setter(strip_option, into))]
    #[serde(flatten)]
    filter: Option<IcingaFilter>,
}

impl ScheduleDowntime {
    /// create a new builder for this endpoint
    ///
    /// this is usually the first step to calling this REST API endpoint
    #[must_use]
    pub fn builder() -> ScheduleDowntimeBuilder {
        ScheduleDowntimeBuilder::default()
    }
}

impl ScheduleDowntimeBuilder {
    /// makes sure the filter object type is valid for this call (either Host or Service)
    ///
    /// validates all_services is only used on host downtimes
    ///
    /// validates duration is specified if fixed is set to false
    ///
    /// # Errors
    ///
    /// this returns an error if the filter field object type is not Host or Service
    pub fn validate(&self) -> Result<(), crate::error::Error> {
        if matches!(&self.fixed, Some(Some(false))) && matches!(&self.duration, None | Some(None)) {
            return Err(crate::error::Error::DurationRequiredOnFlexibleDowntime);
        }
        if let Some(Some(filter)) = &self.filter {
            if filter.object_type != IcingaObjectType::Host
                && filter.object_type != IcingaObjectType::Service
            {
                return Err(crate::error::Error::FilterObjectTypeMismatch(
                    vec![IcingaObjectType::Host, IcingaObjectType::Service],
                    filter.object_type.to_owned(),
                ));
            }
            if filter.object_type == IcingaObjectType::Service
                && matches!(&self.all_services, Some(Some(true)))
            {
                return Err(crate::error::Error::AllServicesInvalidOnServiceDowntime);
            }
        }
        Ok(())
    }
}

impl RestApiEndpoint for ScheduleDowntime {
    type RequestBody = Self;

    fn method(&self) -> Result<reqwest::Method, crate::error::Error> {
        Ok(reqwest::Method::POST)
    }

    fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
        base_url
            .join("v1/actions/schedule-downtime")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)
    }

    fn request_body(
        &self,
    ) -> Result<Option<std::borrow::Cow<'_, Self::RequestBody>>, crate::error::Error>
    where
        Self::RequestBody: Clone + serde::Serialize + std::fmt::Debug,
    {
        Ok(Some(std::borrow::Cow::Borrowed(self)))
    }
}

impl RestApiResponse<ScheduleDowntime> for ResultsWrapper<StatusResponse> {}

#[cfg(test)]
mod test {
    use super::ScheduleDowntime;
    use crate::types::enums::object_type::IcingaObjectType;
    use crate::types::filter::IcingaFilter;
    use pretty_assertions::assert_eq;

    /// a filter for one object of `object_type`
    fn filter(object_type: IcingaObjectType) -> IcingaFilter {
        IcingaFilter {
            object_type,
            filter: "match(\"example\", host.name)".to_owned(),
            filter_vars: std::collections::BTreeMap::new(),
        }
    }

    /// a builder with every required field set
    fn builder(object_type: IcingaObjectType) -> super::ScheduleDowntimeBuilder {
        let start = time::OffsetDateTime::UNIX_EPOCH;
        let mut builder = ScheduleDowntime::builder();
        builder
            .author("test".to_owned())
            .comment("test".to_owned())
            .start_time(start)
            .end_time(start.saturating_add(time::Duration::hours(1)))
            .fixed(Some(true))
            .duration(None)
            .all_services(None)
            .trigger_name(None)
            .child_options(None)
            .filter(filter(object_type));
        builder
    }

    #[test]
    fn service_downtimes_are_valid() -> Result<(), crate::error::Error> {
        builder(IcingaObjectType::Service).build()?;
        Ok(())
    }

    #[test]
    fn all_services_is_invalid_on_service_downtimes() -> Result<(), crate::error::Error> {
        assert!(matches!(
            builder(IcingaObjectType::Service)
                .all_services(Some(true))
                .build(),
            Err(crate::error::Error::AllServicesInvalidOnServiceDowntime)
        ));
        builder(IcingaObjectType::Host)
            .all_services(Some(true))
            .build()?;
        Ok(())
    }

    #[test]
    fn flexible_downtimes_need_a_duration() -> Result<(), crate::error::Error> {
        assert!(matches!(
            builder(IcingaObjectType::Host).fixed(Some(false)).build(),
            Err(crate::error::Error::DurationRequiredOnFlexibleDowntime)
        ));
        builder(IcingaObjectType::Host)
            .fixed(Some(false))
            .duration(Some(time::Duration::minutes(30)))
            .build()?;
        Ok(())
    }

    #[test]
    fn unset_options_are_not_sent() -> Result<(), Box<dyn std::error::Error>> {
        // Icinga rejects e.g. `"child_options": null` with a 400
        let body = serde_json::to_value(builder(IcingaObjectType::Host).build()?)?;
        for key in ["duration", "all_services", "trigger_name", "child_options"] {
            assert!(body.get(key).is_none(), "{key} must not be sent when unset");
        }
        assert_eq!(body.get("fixed"), Some(&serde_json::Value::Bool(true)));
        Ok(())
    }
}
