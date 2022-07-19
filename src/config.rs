//! Configuration related code

use std::path::PathBuf;

use serde::Deserialize;

/// this represents the configuration for an Icinga instance we connect to
#[derive(Debug, Clone, Deserialize)]
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
