//! The error type for the library

use thiserror::Error;

use crate::types::enums::object_type::IcingaObjectType;

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
    /// the object type of a query filter did not match the query result object
    #[error("filter object type expected one of {0:?} but was {1}")]
    FilterObjectTypeMismatch(Vec<IcingaObjectType>, IcingaObjectType),
    /// uninitialized field in builder
    #[error("uninitialized field in builder: {0}")]
    UninitializedFieldInBuilder(#[from] derive_builder::UninitializedFieldError),
    /// all_services invalid when targeting a service
    #[error("all_services is invalid when targeting a service for a downtime")]
    AllServicesInvalidOnServiceDowntime,
    /// duration parameter is required for flexible downtimes but not for fixed ones
    #[error("duration is required for flexible downtimes")]
    DurationRequiredOnFlexibleDowntime,
}
