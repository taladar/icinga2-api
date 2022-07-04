//! API Action execute-command
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#execute-command)

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::serde::{deserialize_seconds_as_duration, serialize_duration_as_seconds};
use crate::types::action::ExecuteCommandResponse;
use crate::types::enums::command_type::IcingaCommandType;
use crate::types::names::IcingaUserName;
use crate::types::query::ResultsWrapper;
use crate::types::rest::{RestApiEndpoint, RestApiResponse};

/// REST API Endpoint for the execute-command call
#[derive(Debug, Clone, derive_builder::Builder, Serialize, Deserialize)]
#[builder(build_fn(error = "crate::error::Error"), derive(Debug))]
pub struct ExecuteCommand {
    /// The time to live of the execution
    #[serde(
        serialize_with = "serialize_duration_as_seconds",
        deserialize_with = "deserialize_seconds_as_duration"
    )]
    ttl: time::Duration,
    /// the command type (check, event or notification)
    command_type: Option<IcingaCommandType>,
    /// The command to execute. Its type must the same as command_type. It can be a macro string. Default: depending on the command_type it’s either $check_command$, $event_command$ or $notification_command$
    command: Option<String>,
    /// The endpoint to execute the command on. It can be a macro string. Default: $command_endpoint$.
    endpoint: Option<String>,
    /// Macro overrides. Default: {}
    macros: Option<BTreeMap<String, serde_json::Value>>,
    /// The user used for the notification command.
    user: Option<IcingaUserName>,
    /// The notification used for the notification command.
    notification: Option<String>,
}

impl ExecuteCommand {
    /// create a new builder for this endpoint
    ///
    /// this is usually the first step to calling this REST API endpoint
    #[must_use]
    pub fn builder() -> ExecuteCommandBuilder {
        ExecuteCommandBuilder::default()
    }
}

impl RestApiEndpoint for ExecuteCommand {
    type RequestBody = ExecuteCommand;

    fn method(&self) -> Result<http::Method, crate::error::Error> {
        Ok(http::Method::POST)
    }

    fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
        base_url
            .join("v1/actions/execute-command")
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

impl RestApiResponse<ExecuteCommand> for ResultsWrapper<ExecuteCommandResponse> {}
