//! API Action process-check-result
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#process-check-result)

use serde::{Deserialize, Serialize};

use crate::serde::{
    deserialize_optional_icinga_timestamp, deserialize_optional_seconds_as_duration,
    serialize_optional_duration_as_seconds, serialize_optional_icinga_timestamp,
};
use crate::types::action::StatusResponse;
use crate::types::common::{command::IcingaCommandLine, performance_data::IcingaPerformanceData};
use crate::types::enums::object_type::IcingaObjectType;
use crate::types::filter::IcingaFilter;
use crate::types::query::ResultsWrapper;
use crate::types::rest::{RestApiEndpoint, RestApiResponse};

/// REST API Endpoint for the process-check-result call
#[derive(Debug, Clone, derive_builder::Builder, Serialize, Deserialize)]
#[builder(
    build_fn(error = "crate::error::Error", validate = "Self::validate"),
    derive(Debug)
)]
pub struct ProcessCheckResult {
    ///  For services: 0=OK, 1=WARNING, 2=CRITICAL, 3=UNKNOWN
    ///  For hosts: 0=UP, 1=DOWN
    pub exit_status: u64,
    /// the plugin output without the performance data
    pub plugin_output: String,
    /// the performance data
    #[builder(default)]
    pub performance_data: Option<Vec<IcingaPerformanceData>>,
    /// the check command
    #[builder(default)]
    pub check_command: Option<IcingaCommandLine>,
    /// usually the name of the command endpoint
    #[builder(default)]
    pub check_source: Option<String>,
    /// the start time of the check command execution
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub execution_start: Option<time::OffsetDateTime>,
    /// the end time of the check command execution
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    pub execution_end: Option<time::OffsetDateTime>,
    /// Time-to-live duration in seconds for this check result. The next expected check result is now + ttl where freshness checks are executed.
    #[serde(
        serialize_with = "serialize_optional_duration_as_seconds",
        deserialize_with = "deserialize_optional_seconds_as_duration"
    )]
    pub ttl: Option<time::Duration>,
    /// filter to target which host and/or service this check result applies to
    #[builder(default, setter(strip_option, into))]
    #[serde(flatten)]
    filter: Option<IcingaFilter>,
}

impl ProcessCheckResult {
    /// create a new builder for this endpoint
    ///
    /// this is usually the first step to calling this REST API endpoint
    #[must_use]
    pub fn builder() -> ProcessCheckResultBuilder {
        ProcessCheckResultBuilder::default()
    }
}

impl ProcessCheckResultBuilder {
    /// makes sure the filter object type is valid for this call (either Host or Service)
    ///
    /// # Errors
    ///
    /// this returns an error if the filter field object type is not Host or Service
    pub fn validate(&self) -> Result<(), crate::error::Error> {
        if let Some(Some(filter)) = &self.filter {
            if filter.object_type != IcingaObjectType::Host
                && filter.object_type != IcingaObjectType::Service
            {
                Err(crate::error::Error::FilterObjectTypeMismatch(
                    vec![IcingaObjectType::Host, IcingaObjectType::Service],
                    filter.object_type.to_owned(),
                ))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl RestApiEndpoint for ProcessCheckResult {
    type RequestBody = ProcessCheckResult;

    fn method(&self) -> Result<http::Method, crate::error::Error> {
        Ok(http::Method::POST)
    }

    fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
        base_url
            .join("v1/actions/process-check-result")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)
    }

    fn request_body(
        &self,
    ) -> Result<Option<std::borrow::Cow<Self::RequestBody>>, crate::error::Error>
    where
        Self::RequestBody: Clone + serde::Serialize + std::fmt::Debug,
    {
        Ok(Some(std::borrow::Cow::Borrowed(self)))
    }
}

impl RestApiResponse<ProcessCheckResult> for ResultsWrapper<StatusResponse> {}
