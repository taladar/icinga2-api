//! Main API object (async version)

use std::{path::Path, str::from_utf8};

use serde::{de::DeserializeOwned, Serialize};

use crate::types::rest::{RestApiEndpoint, RestApiResponse};

/// the runtime object for an Icinga2 instance (blocking variant)
#[derive(Debug)]
pub struct Icinga2Async {
    /// the HTTP client to use
    client: reqwest::Client,
    /// the base URL for the Icinga API
    url: url::Url,
    /// username
    pub username: String,
    /// password
    pub password: String,
}

impl Icinga2Async {
    /// create a new Icinga2 instance from a TOML config file
    ///
    /// # Errors
    /// this fails if the configuration file can not be found or parsed
    /// or the CA certificate file mentioned in the configuration file
    /// can not be found or parsed
    pub fn from_config_file(path: &Path) -> Result<Self, crate::error::Error> {
        let content =
            std::fs::read_to_string(path).map_err(crate::error::Error::CouldNotReadConfigFile)?;
        let config: crate::config::Icinga2Instance =
            toml::from_str(&content).map_err(crate::error::Error::CouldNotParseConfig)?;
        let client_builder = reqwest::ClientBuilder::new();
        let client_builder = client_builder.user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ));
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let client_builder = client_builder.default_headers(headers);
        let client_builder = if let Some(ca_certificate) = &config.ca_certificate {
            let ca_cert_content = std::fs::read(ca_certificate)
                .map_err(crate::error::Error::CouldNotReadCACertFile)?;
            let ca_cert = reqwest::Certificate::from_pem(&ca_cert_content)
                .map_err(crate::error::Error::CouldNotParsePEMCACertificate)?;
            let client_builder = client_builder.add_root_certificate(ca_cert);
            client_builder.tls_built_in_root_certs(false)
        } else {
            client_builder
        };
        let client = client_builder
            .build()
            .map_err(crate::error::Error::CouldNotBuildReqwestClientFromSuppliedInformation)?;
        let url =
            url::Url::parse(&config.url).map_err(crate::error::Error::CouldNotParseUrlInConfig)?;
        let username = config.username;
        let password = config.password;
        Ok(Icinga2Async {
            client,
            url,
            username,
            password,
        })
    }

    /// common code for the REST API calls
    ///
    /// # Errors
    ///
    /// this returns an error if encoding, the actual request, or decoding of the response fail
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
        let actual_method = if method == http::Method::GET && request_body.is_some() {
            http::Method::POST
        } else {
            method.to_owned()
        };
        let mut req = self.client.request(actual_method, url.to_owned());
        if method == http::Method::GET && request_body.is_some() {
            tracing::trace!("Sending GET request with body as POST via X-HTTP-Method-Override");
            req = req.header(
                "X-HTTP-Method-Override",
                http::header::HeaderValue::from_static("GET"),
            );
        }
        req = req.basic_auth(&self.username, Some(&self.password));
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
                            match (response_body, segment) {
                                (
                                    serde_json::Value::Array(vs),
                                    serde_path_to_error::Segment::Seq { index },
                                ) => {
                                    if let Some(v) = vs.get(*index) {
                                        response_body = v.to_owned();
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
                                        response_body = v.to_owned();
                                    } else {
                                        // if we can not find the element serde_path_to_error references fall back to just returning the error
                                        return Err(e.into());
                                    }
                                }
                                _ => {
                                    // if we can not find the element serde_path_to_error references fall back to just returning the error
                                    return Err(e.into());
                                }
                            }
                        }
                        tracing::error!("Value in location path references is: {}", response_body);
                    }
                    Err(e.into())
                }
            }
        }
    }
}
