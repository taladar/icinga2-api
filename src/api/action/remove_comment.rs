//! API Action remove-comment
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#remove-comment)

use serde::{Deserialize, Serialize};

use crate::types::action::StatusResponse;
use crate::types::enums::object_type::IcingaObjectType;
use crate::types::filter::IcingaFilter;
use crate::types::query::ResultsWrapper;
use crate::types::rest::{RestApiEndpoint, RestApiResponse};

/// REST API Endpoint for the remove-comment call
#[derive(Debug, Clone, derive_builder::Builder, Serialize, Deserialize)]
#[builder(
    build_fn(error = "crate::error::Error", validate = "Self::validate"),
    derive(Debug)
)]
pub struct RemoveComment {
    /// the author of the removal request
    author: String,
    /// filter to target which host and/or service to remove the comment from
    #[builder(default, setter(strip_option, into))]
    #[serde(flatten)]
    filter: Option<IcingaFilter>,
}

impl RemoveComment {
    /// create a new builder for this endpoint
    ///
    /// this is usually the first step to calling this REST API endpoint
    #[must_use]
    pub fn builder() -> RemoveCommentBuilder {
        RemoveCommentBuilder::default()
    }
}

impl RemoveCommentBuilder {
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

impl RestApiEndpoint for RemoveComment {
    type RequestBody = RemoveComment;

    fn method(&self) -> Result<reqwest::Method, crate::error::Error> {
        Ok(reqwest::Method::POST)
    }

    fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
        base_url
            .join("v1/actions/remove-comment")
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

impl RestApiResponse<RemoveComment> for ResultsWrapper<StatusResponse> {}
