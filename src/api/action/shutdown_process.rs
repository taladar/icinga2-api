//! API Action shutdown-process
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#shutdown-process)

use serde::{Deserialize, Serialize};

use crate::types::action::StatusResponse;
use crate::types::query::ResultsWrapper;
use crate::types::rest::{RestApiEndpoint, RestApiResponse};

/// REST API Endpoint for the shutdown-process call
#[derive(Debug, Clone, derive_builder::Builder, Serialize, Deserialize)]
pub struct ShutdownProcess {}

impl ShutdownProcess {
    /// create a new builder for this endpoint
    ///
    /// this is usually the first step to calling this REST API endpoint
    #[must_use]
    pub fn builder() -> ShutdownProcessBuilder {
        ShutdownProcessBuilder::default()
    }
}

impl RestApiEndpoint for ShutdownProcess {
    type RequestBody = ShutdownProcess;

    fn method(&self) -> Result<reqwest::Method, crate::error::Error> {
        Ok(reqwest::Method::POST)
    }

    fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
        base_url
            .join("v1/actions/shutdown-process")
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

impl RestApiResponse<ShutdownProcess> for ResultsWrapper<StatusResponse> {}
