//! API Action generate-ticket
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#generate-ticket)

use serde::{Deserialize, Serialize};

use crate::types::action::GenerateTicketResponse;
use crate::types::query::ResultsWrapper;
use crate::types::rest::{RestApiEndpoint, RestApiResponse};

/// REST API Endpoint for the generate-ticket call
#[derive(Debug, Clone, derive_builder::Builder, Serialize, Deserialize)]
#[builder(build_fn(error = "crate::error::Error"), derive(Debug))]
pub struct GenerateTicket {
    /// The host’s common name for which the ticket should be generated.
    cn: String,
}

impl GenerateTicket {
    /// create a new builder for this endpoint
    ///
    /// this is usually the first step to calling this REST API endpoint
    #[must_use]
    pub fn builder() -> GenerateTicketBuilder {
        GenerateTicketBuilder::default()
    }
}

impl RestApiEndpoint for GenerateTicket {
    type RequestBody = GenerateTicket;

    fn method(&self) -> Result<reqwest::Method, crate::error::Error> {
        Ok(reqwest::Method::POST)
    }

    fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
        base_url
            .join("v1/actions/generate-ticket")
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

impl RestApiResponse<GenerateTicket> for ResultsWrapper<GenerateTicketResponse> {}
