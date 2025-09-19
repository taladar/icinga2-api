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
    fixed: Option<bool>,
    /// the duration of the downtime
    #[serde(
        serialize_with = "serialize_optional_duration_as_seconds",
        deserialize_with = "deserialize_optional_seconds_as_duration"
    )]
    duration: Option<time::Duration>,
    /// Sets downtime for all services for the matched host objects. If child_options are set, all child hosts and their services will schedule a downtime too. Defaults to false.
    all_services: Option<bool>,
    /// Sets the trigger for a triggered downtime.
    trigger_name: Option<IcingaDowntimeName>,
    /// Schedule child downtimes.
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
        if let Some(Some(false)) = &self.fixed {
            if self.duration.is_none() {
                return Err(crate::error::Error::DurationRequiredOnFlexibleDowntime);
            }
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
            if filter.object_type == IcingaObjectType::Service {
                return Err(crate::error::Error::AllServicesInvalidOnServiceDowntime);
            }
        }
        Ok(())
    }
}

impl RestApiEndpoint for ScheduleDowntime {
    type RequestBody = ScheduleDowntime;

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
