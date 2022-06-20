#![deny(unknown_lints)]
#![deny(renamed_and_removed_lints)]
#![forbid(unsafe_code)]
#![deny(deprecated)]
#![forbid(private_in_public)]
#![forbid(non_fmt_panics)]
#![deny(unreachable_code)]
#![deny(unreachable_patterns)]
#![forbid(unused_doc_comments)]
#![forbid(unused_must_use)]
#![deny(while_true)]
#![deny(unused_parens)]
#![deny(redundant_semicolons)]
#![deny(non_ascii_idents)]
#![deny(confusable_idents)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::cargo_common_metadata)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_debug_implementations)]
#![deny(clippy::mod_module_files)]
//#![warn(clippy::pedantic)]
#![warn(clippy::redundant_else)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::panic)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![doc = include_str!("../README.md")]

use std::{
    path::{Path, PathBuf},
    str::from_utf8,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

/// Error type for icinga2_api
#[derive(Debug, Error)]
pub enum Error {
    /// could not read config file
    #[error("could not read config file: {0}")]
    CouldNotReadConfigFile(std::io::Error),
    /// could not parse config
    #[error("could not parse config: {0}")]
    CouldNotParseConfig(toml::de::Error),
    /// could not parse URL in config
    #[error("could not parse URL in config: {0}")]
    CouldNotParseUrlInConfig(url::ParseError),
    /// could not build reqwest client from supplied information
    #[error("could not build reqwest client from supplied information: {0}")]
    CouldNotBuildReqwestClientFromSuppliedInformation(reqwest::Error),
    /// could not read CA certificate file
    #[error("could not read CA certificate file: {0}")]
    CouldNotReadCACertFile(std::io::Error),
    /// could not parse PEM CA certificate
    #[error("could not parse PEM CA certificate: {0}")]
    CouldNotParsePEMCACertificate(reqwest::Error),
    /// An error occurred when serializing/deserializing JSON
    #[error("error in json serialization/deserialization: {0}")]
    SerdeJsonError(#[from] serde_path_to_error::Error<serde_json::Error>),
    /// Response body was empty so we can not deserialize it as JSON
    #[error("empty response body with status: {0}")]
    EmptyResponseBody(reqwest::StatusCode),
    /// An error occurred in the reqwest library (HTTP)
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

/// this represents the configuration for an Icinga instance we connect to
#[derive(Debug, Deserialize)]
pub struct Icinga2Instance {
    /// the URL to connect to, without the v1 component or anything after that
    pub url: String,
    /// the CA certificate to use to validate the server certificate
    pub ca_certificate: Option<PathBuf>,
    /// username
    pub username: String,
    /// password
    pub password: String,
}

/// the runtime object for an Icinga2 instance
#[derive(Debug)]
pub struct Icinga2 {
    /// the HTTP client to use
    client: reqwest::blocking::Client,
    /// the base URL for the Icinga API
    url: url::Url,
    /// username
    pub username: String,
    /// password
    pub password: String,
}

impl Icinga2 {
    /// create a new Icinga2 instance from a TOML config file
    pub fn from_config_file(path: &Path) -> Result<Self, crate::Error> {
        let content =
            std::fs::read_to_string(path).map_err(crate::Error::CouldNotReadConfigFile)?;
        let config: Icinga2Instance =
            toml::from_str(&content).map_err(crate::Error::CouldNotParseConfig)?;
        let client_builder = reqwest::blocking::ClientBuilder::new();
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
            let ca_cert_content =
                std::fs::read(ca_certificate).map_err(crate::Error::CouldNotReadCACertFile)?;
            let ca_cert = reqwest::Certificate::from_pem(&ca_cert_content)
                .map_err(crate::Error::CouldNotParsePEMCACertificate)?;
            let client_builder = client_builder.add_root_certificate(ca_cert);
            client_builder.tls_built_in_root_certs(false)
        } else {
            client_builder
        };
        let client = client_builder
            .build()
            .map_err(crate::Error::CouldNotBuildReqwestClientFromSuppliedInformation)?;
        let url = url::Url::parse(&config.url).map_err(crate::Error::CouldNotParseUrlInConfig)?;
        let username = config.username;
        let password = config.password;
        Ok(Icinga2 {
            client,
            url,
            username,
            password,
        })
    }

    /// common code for the API calls
    fn rest<Req, Res>(
        &self,
        method: http::Method,
        url: url::Url,
        request_body: Option<Req>,
    ) -> Result<Res, crate::Error>
    where
        Req: Serialize + std::fmt::Debug,
        Res: DeserializeOwned + std::fmt::Debug,
    {
        let mut req = self.client.request(method.to_owned(), url.to_owned());
        req = req.basic_auth(&self.username, Some(&self.password));
        if let Some(request_body) = request_body {
            tracing::trace!("Request body:\n{:#?}", request_body);
            req = req.json(&request_body);
        }
        let result = req.send();
        if let Err(ref e) = result {
            tracing::error!(%url, %method, "Icinga2 send error: {:?}", e);
        }
        let result = result?;
        let status = result.status();
        let response_body = result.bytes()?;
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
            Err(crate::Error::EmptyResponseBody(status))
        } else {
            let jd = &mut serde_json::Deserializer::from_slice(&response_body);
            let response_body: Res = serde_path_to_error::deserialize(jd)?;
            tracing::trace!("Parsed response body:\n{:#?}", response_body);
            Ok(response_body)
        }
    }
}

#[cfg(test)]
mod test {}
