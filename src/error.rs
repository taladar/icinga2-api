//! The error type for the library

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
    /// could not parse URL fragment
    #[error("could not parse URL fragment: {0}")]
    CouldNotParseUrlFragment(url::ParseError),
    /// error writing filter expression
    #[error("error writing filter expression: {0}")]
    WritingFilterExpression(std::fmt::Error),
}
