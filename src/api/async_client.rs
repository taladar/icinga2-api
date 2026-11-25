//! Main API object (async version)

use std::{path::Path, str::from_utf8};

use futures::Stream;
use futures::StreamExt as _;
use futures::stream::TryStreamExt as _;
use serde::{Serialize, de::DeserializeOwned};
use tokio::io::AsyncBufReadExt as _;
use tokio_stream::wrappers::LinesStream;
use tokio_util::io::StreamReader;

use crate::config::{Icinga2Auth, Icinga2Instance};
use crate::types::{
    enums::event_stream_type::IcingaEventStreamType,
    event_stream::IcingaEvent,
    filter::IcingaFilter,
    rest::{RestApiEndpoint, RestApiResponse},
};

/// the runtime object for an Icinga2 instance (blocking variant)
#[derive(Debug, Clone)]
pub struct Icinga2Async {
    /// the HTTP client to use
    client: reqwest::Client,
    /// the base URL for the Icinga API
    pub url: url::Url,
    /// username, empty when authenticating with a client certificate
    pub username: String,
    /// how to authenticate
    auth: Icinga2Auth,
}

impl Icinga2Async {
    /// create a new Icinga2 instance from a config that was
    /// either manually created or previously loaded via [Icinga2Instance::from_config_file]
    ///
    /// # Errors
    /// this fails if the CA certificate file mentioned in the configuration
    /// can not be found or parsed
    pub fn from_instance_config(config: &Icinga2Instance) -> Result<Self, crate::error::Error> {
        Self::from_parts(
            &config.url,
            config.ca_certificate.as_deref(),
            Icinga2Auth::Basic {
                username: config.username.clone(),
                password: config.password.clone(),
            },
        )
    }

    /// create a new Icinga2 instance from its URL (without the v1 component),
    /// an optional CA certificate to validate the server certificate with
    /// and the authentication to use
    ///
    /// see also [crate::config::instance_file::Icinga2InstanceFile] which
    /// resolves these parts from an instance config file
    ///
    /// # Errors
    /// this fails if the CA certificate file can not be found or parsed,
    /// the client certificate can not be parsed or the URL can not be parsed
    pub fn from_parts(
        url: &str,
        ca_certificate: Option<&Path>,
        auth: Icinga2Auth,
    ) -> Result<Self, crate::error::Error> {
        let parts = crate::config::client_parts(url, ca_certificate, &auth)?;
        let client_builder = reqwest::ClientBuilder::new()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .default_headers(parts.headers);
        let client_builder = if let Some(ca_cert) = parts.ca_certificate {
            client_builder.tls_certs_only([ca_cert])
        } else {
            client_builder
        };
        let client_builder = if let Some(identity) = parts.identity {
            client_builder.identity(identity)
        } else {
            client_builder
        };
        let client = client_builder
            .build()
            .map_err(crate::error::Error::CouldNotBuildReqwestClientFromSuppliedInformation)?;
        Ok(Self {
            client,
            url: parts.url,
            username: auth.username().to_owned(),
            auth,
        })
    }

    /// adds the basic auth header to a request if we use basic auth
    fn authenticate(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.auth {
            Icinga2Auth::Basic { username, password } => req.basic_auth(username, Some(password)),
            Icinga2Auth::ClientCertificate { .. } => req,
        }
    }

    /// create a new Icinga2 instance from a TOML config file
    ///
    /// # Errors
    /// this fails if the configuration file can not be found or parsed
    /// or the CA certificate file mentioned in the configuration file
    /// can not be found or parsed
    pub fn from_config_file(path: &Path) -> Result<Self, crate::error::Error> {
        let icinga_instance = Icinga2Instance::from_config_file(path)?;
        Self::from_instance_config(&icinga_instance)
    }

    /// common code for the REST API calls
    ///
    /// # Errors
    ///
    /// this returns an error if encoding, the actual request, or decoding of the response fail
    #[expect(
        clippy::future_not_send,
        reason = "neither ApiEndpoint nor its RequestBody is required to be Send; callers that need a Send future can wrap with their own bounds"
    )]
    pub async fn rest<ApiEndpoint, Res>(
        &self,
        api_endpoint: ApiEndpoint,
    ) -> Result<Res, crate::error::Error>
    where
        ApiEndpoint: RestApiEndpoint,
        <ApiEndpoint as RestApiEndpoint>::RequestBody: Clone + Serialize + std::fmt::Debug,
        Res: DeserializeOwned + std::fmt::Debug + RestApiResponse<ApiEndpoint>,
    {
        let method = api_endpoint.method()?;
        let url = api_endpoint.url(&self.url)?;
        let request_body: Option<std::borrow::Cow<<ApiEndpoint as RestApiEndpoint>::RequestBody>> =
            api_endpoint.request_body()?;
        let actual_method = if method == reqwest::Method::GET && request_body.is_some() {
            reqwest::Method::POST
        } else {
            method.to_owned()
        };
        let mut req = self.client.request(actual_method, url.to_owned());
        if method == reqwest::Method::GET && request_body.is_some() {
            tracing::trace!("Sending GET request with body as POST via X-HTTP-Method-Override");
            req = req.header(
                "X-HTTP-Method-Override",
                reqwest::header::HeaderValue::from_static("GET"),
            );
        }
        req = self.authenticate(req);
        if let Some(request_body) = request_body {
            tracing::trace!("Request body:\n{:#?}", request_body);
            req = req.json(&request_body);
        }
        let result = req.send().await;
        if let Err(ref e) = result {
            tracing::error!(%url, %method, "Icinga2 send error: {:?}", e);
        }
        let result = result?;
        let status = result.status();
        let response_body = result.bytes().await?;
        match from_utf8(&response_body) {
            Ok(response_body) => {
                tracing::trace!("Response body:\n{}", &response_body);
            }
            Err(e) => {
                tracing::trace!(
                    "Response body that could not be parsed as utf8 because of {}:\n{:?}",
                    &e,
                    &response_body
                );
            }
        }
        if status.is_client_error() {
            tracing::error!(%url, %method, "Icinga2 status error (client error): {:?}", status);
        } else if status.is_server_error() {
            tracing::error!(%url, %method, "Icinga2 status error (server error): {:?}", status);
        }
        if response_body.is_empty() {
            Err(crate::error::Error::EmptyResponseBody(status))
        } else {
            let jd = &mut serde_json::Deserializer::from_slice(&response_body);
            match serde_path_to_error::deserialize(jd) {
                Ok(response_body) => {
                    tracing::trace!("Parsed response body:\n{:#?}", response_body);
                    Ok(response_body)
                }
                Err(e) => {
                    let path = e.path();
                    tracing::error!("Parsing failed at path {}: {}", path.to_string(), e.inner());
                    if let Ok(response_body) = serde_json::from_slice(&response_body) {
                        let mut response_body: serde_json::Value = response_body;
                        for segment in path {
                            let next = match (&response_body, segment) {
                                (
                                    serde_json::Value::Array(vs),
                                    serde_path_to_error::Segment::Seq { index },
                                ) => {
                                    if let Some(v) = vs.get(*index) {
                                        v.clone()
                                    } else {
                                        // if we can not find the element serde_path_to_error references fall back to just returning the error
                                        return Err(e.into());
                                    }
                                }
                                (
                                    serde_json::Value::Object(m),
                                    serde_path_to_error::Segment::Map { key },
                                ) => {
                                    if let Some(v) = m.get(key) {
                                        v.clone()
                                    } else {
                                        // if we can not find the element serde_path_to_error references fall back to just returning the error
                                        return Err(e.into());
                                    }
                                }
                                _ => {
                                    // if we can not find the element serde_path_to_error references fall back to just returning the error
                                    return Err(e.into());
                                }
                            };
                            response_body = next;
                        }
                        tracing::error!("Value in location path references is: {}", response_body);
                    }
                    Err(e.into())
                }
            }
        }
    }

    /// Long-polling on an event stream
    ///
    /// # Errors
    ///
    /// this returns an error if encoding or the actual request fail
    pub async fn event_stream(
        &self,
        types: &[IcingaEventStreamType],
        queue: &str,
        filter: Option<IcingaFilter>,
    ) -> Result<impl Stream<Item = Result<IcingaEvent, std::io::Error>>, crate::error::Error> {
        let method = reqwest::Method::POST;
        let mut url = self
            .url
            .join("v1/events")
            .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
        for t in types {
            url.query_pairs_mut().append_pair("types", &t.to_string());
        }
        url.query_pairs_mut().append_pair("queue", queue);
        let request_body = filter;
        let mut req = self.client.request(method.to_owned(), url.to_owned());
        req = self.authenticate(req);
        if let Some(request_body) = request_body {
            tracing::trace!("Request body:\n{:#?}", request_body);
            req = req.json(&request_body);
        }
        let result = req.send().await;
        if let Err(ref e) = result {
            tracing::error!(%url, %method, "Icinga2 send error: {:?}", e);
        }
        let result = result?;
        let status = result.status();
        if status.is_client_error() {
            tracing::error!(%url, %method, "Icinga2 status error (client error): {:?}", status);
        } else if status.is_server_error() {
            tracing::error!(%url, %method, "Icinga2 status error (server error): {:?}", status);
        }
        let byte_chunk_stream = result.bytes_stream().map_err(std::io::Error::other);
        let stream_reader = StreamReader::new(byte_chunk_stream);
        let line_reader = LinesStream::new(stream_reader.lines());
        let event_reader = line_reader.map(|l| match l {
            Ok(l) => {
                tracing::trace!("Icinga2 received raw event:\n{}", &l);
                let jd = &mut serde_json::Deserializer::from_str(&l);
                match serde_path_to_error::deserialize(jd) {
                    Ok(event) => {
                        tracing::trace!("Icinga2 received event:\n{:#?}", &event);
                        Ok(event)
                    }
                    Err(e) => Err(std::io::Error::other(e)),
                }
            }
            Err(e) => Err(e),
        });
        Ok(event_reader)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[tokio::test]
    async fn test_event_stream_async() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let icinga2 = Icinga2Async::from_config_file(std::path::Path::new(&std::env::var(
            "ICINGA_TEST_INSTANCE_CONFIG",
        )?))?;
        let mut stream = icinga2
            .event_stream(
                &[
                    IcingaEventStreamType::CheckResult,
                    IcingaEventStreamType::StateChange,
                    IcingaEventStreamType::Notification,
                    IcingaEventStreamType::AcknowledgementSet,
                    IcingaEventStreamType::AcknowledgementCleared,
                    IcingaEventStreamType::CommentAdded,
                    IcingaEventStreamType::CommentRemove,
                    IcingaEventStreamType::DowntimeAdded,
                    IcingaEventStreamType::DowntimeRemoved,
                    IcingaEventStreamType::DowntimeStarted,
                    IcingaEventStreamType::DowntimeTriggered,
                    IcingaEventStreamType::ObjectCreated,
                    IcingaEventStreamType::ObjectDeleted,
                    IcingaEventStreamType::ObjectModified,
                    IcingaEventStreamType::Flapping,
                ],
                "test",
                None,
            )
            .await?;
        for _ in 0..100 {
            let event = stream.next().await;
            tracing::trace!("Got event:\n{:#?}", event);
            if let Some(event) = event {
                event?;
            }
        }
        Ok(())
    }
}
