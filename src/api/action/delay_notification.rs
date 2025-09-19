//! API Action delay-notification
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#delay-notification)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_icinga_timestamp, serialize_icinga_timestamp};
use crate::types::action::StatusResponse;
use crate::types::enums::object_type::IcingaObjectType;
use crate::types::filter::IcingaFilter;
use crate::types::query::ResultsWrapper;
use crate::types::rest::{RestApiEndpoint, RestApiResponse};

/// REST API Endpoint for the delay-notification call
#[derive(Debug, Clone, derive_builder::Builder, Serialize, Deserialize)]
#[builder(
    build_fn(error = "crate::error::Error", validate = "Self::validate"),
    derive(Debug)
)]
pub struct DelayNotification {
    /// delay notifications until this timestamp
    #[serde(
        serialize_with = "serialize_icinga_timestamp",
        deserialize_with = "deserialize_icinga_timestamp"
    )]
    timestamp: time::OffsetDateTime,
    /// filter to target which host and/or service for which to delay notifications
    #[builder(default, setter(strip_option, into))]
    #[serde(flatten)]
    filter: Option<IcingaFilter>,
}

impl DelayNotification {
    /// create a new builder for this endpoint
    ///
    /// this is usually the first step to calling this REST API endpoint
    #[must_use]
    pub fn builder() -> DelayNotificationBuilder {
        DelayNotificationBuilder::default()
    }
}

impl DelayNotificationBuilder {
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

impl RestApiEndpoint for DelayNotification {
    type RequestBody = DelayNotification;

    fn method(&self) -> Result<reqwest::Method, crate::error::Error> {
        Ok(reqwest::Method::POST)
    }

    fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
        base_url
            .join("v1/actions/delay-notification")
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

impl RestApiResponse<DelayNotification> for ResultsWrapper<StatusResponse> {}
