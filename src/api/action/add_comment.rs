//! API Action add-comment
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#add-comment)

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_optional_icinga_timestamp, serialize_optional_icinga_timestamp};
use crate::types::action::StatusResponse;
use crate::types::enums::object_type::IcingaObjectType;
use crate::types::filter::IcingaFilter;
use crate::types::query::ResultsWrapper;
use crate::types::rest::{RestApiEndpoint, RestApiResponse};

/// REST API Endpoint for the add-comment call
#[derive(Debug, Clone, derive_builder::Builder, Serialize, Deserialize)]
#[builder(
    build_fn(error = "crate::error::Error", validate = "Self::validate"),
    derive(Debug)
)]
pub struct AddComment {
    /// the author of the comment
    author: String,
    /// the body of the comment
    comment: String,
    /// expiry time for the comment
    #[serde(
        serialize_with = "serialize_optional_icinga_timestamp",
        deserialize_with = "deserialize_optional_icinga_timestamp"
    )]
    expiry: Option<time::OffsetDateTime>,
    /// filter to target which host and/or service to attach the comment to
    #[builder(default, setter(strip_option, into))]
    #[serde(flatten)]
    filter: Option<IcingaFilter>,
}

impl AddComment {
    /// create a new builder for this endpoint
    ///
    /// this is usually the first step to calling this REST API endpoint
    #[must_use]
    pub fn builder() -> AddCommentBuilder {
        AddCommentBuilder::default()
    }
}

impl AddCommentBuilder {
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

impl RestApiEndpoint for AddComment {
    type RequestBody = AddComment;

    fn method(&self) -> Result<reqwest::Method, crate::error::Error> {
        Ok(reqwest::Method::POST)
    }

    fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
        base_url
            .join("v1/actions/add-comment")
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

impl RestApiResponse<AddComment> for ResultsWrapper<StatusResponse> {}
