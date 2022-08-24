//! Configuration related code

use std::path::{Path, PathBuf};

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

impl Icinga2Instance {
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
        Ok(config)
    }
}
