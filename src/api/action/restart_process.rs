//! API Action restart-process
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#restart-process)

use serde::{Deserialize, Serialize};

use crate::types::action::StatusResponse;
use crate::types::query::ResultsWrapper;
use crate::types::rest::{RestApiEndpoint, RestApiResponse};

/// REST API Endpoint for the restart-process call
#[derive(Debug, Clone, derive_builder::Builder, Serialize, Deserialize)]
pub struct RestartProcess {}

impl RestartProcess {
    /// create a new builder for this endpoint
    ///
    /// this is usually the first step to calling this REST API endpoint
    #[must_use]
    pub fn builder() -> RestartProcessBuilder {
        RestartProcessBuilder::default()
    }
}

impl RestApiEndpoint for RestartProcess {
    type RequestBody = RestartProcess;

    fn method(&self) -> Result<http::Method, crate::error::Error> {
        Ok(http::Method::POST)
    }

    fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
        base_url
            .join("v1/actions/restart-process")
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

impl RestApiResponse<RestartProcess> for ResultsWrapper<StatusResponse> {}
