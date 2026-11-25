//! Configuration related code

pub mod instance_file;

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
            fs_err::read_to_string(path).map_err(crate::error::Error::CouldNotReadConfigFile)?;
        let config: Self =
            toml::from_str(&content).map_err(crate::error::Error::CouldNotParseConfig)?;
        Ok(config)
    }
}

/// how a client authenticates against the Icinga API
///
/// used with `from_parts` on the clients, the [`Debug`] implementation
/// does not print the password or the private key
#[derive(Clone)]
pub enum Icinga2Auth {
    /// HTTP basic auth as an Icinga `ApiUser` with a password
    Basic {
        /// the name of the `ApiUser`
        username: String,
        /// the password of the `ApiUser`
        password: String,
    },
    /// TLS client certificate auth as an Icinga `ApiUser` with a `client_cn`,
    /// e.g. with the certificate of an Icinga agent
    ClientCertificate {
        /// the PEM encoded certificate followed by the PEM encoded private key
        identity_pem: Vec<u8>,
    },
}

impl Icinga2Auth {
    /// client certificate auth from a PEM certificate file and a PEM private
    /// key file, e.g. `/var/lib/icinga2/certs/<fqdn>.crt` and `.key`
    ///
    /// # Errors
    ///
    /// this fails if either file can not be read
    pub fn client_certificate_from_files(
        certificate: &Path,
        key: &Path,
    ) -> Result<Self, std::io::Error> {
        let mut identity_pem = fs_err::read(certificate)?;
        if !identity_pem.ends_with(b"\n") {
            identity_pem.push(b'\n');
        }
        identity_pem.extend(fs_err::read(key)?);
        Ok(Self::ClientCertificate { identity_pem })
    }

    /// the username to report for this auth, empty for client certificates
    #[must_use]
    pub fn username(&self) -> &str {
        match self {
            Self::Basic { username, .. } => username,
            Self::ClientCertificate { .. } => "",
        }
    }
}

impl std::fmt::Debug for Icinga2Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic { username, .. } => f
                .debug_struct("Basic")
                .field("username", username)
                .field("password", &"<redacted>")
                .finish(),
            Self::ClientCertificate { .. } => f
                .debug_struct("ClientCertificate")
                .field("identity_pem", &"<redacted>")
                .finish(),
        }
    }
}

/// builds the settings shared by the async and blocking clients
///
/// # Errors
///
/// this fails if the CA certificate can not be read or parsed, the client
/// identity can not be parsed or the URL can not be parsed
#[cfg(any(feature = "async", feature = "blocking"))]
pub(crate) fn client_parts(
    url: &str,
    ca_certificate: Option<&Path>,
    auth: &Icinga2Auth,
) -> Result<ClientParts, crate::error::Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    let ca_certificate = if let Some(ca_certificate) = ca_certificate {
        let ca_cert_content =
            fs_err::read(ca_certificate).map_err(crate::error::Error::CouldNotReadCACertFile)?;
        Some(
            reqwest::Certificate::from_pem(&ca_cert_content)
                .map_err(crate::error::Error::CouldNotParsePEMCACertificate)?,
        )
    } else {
        None
    };
    let identity = match auth {
        Icinga2Auth::Basic { .. } => None,
        Icinga2Auth::ClientCertificate { identity_pem } => Some(
            reqwest::Identity::from_pem(identity_pem)
                .map_err(crate::error::Error::CouldNotBuildReqwestClientFromSuppliedInformation)?,
        ),
    };
    Ok(ClientParts {
        headers,
        ca_certificate,
        identity,
        url: url::Url::parse(url).map_err(crate::error::Error::CouldNotParseUrlInConfig)?,
    })
}

/// the settings shared by the async and blocking clients, see [`client_parts`]
#[cfg(any(feature = "async", feature = "blocking"))]
pub(crate) struct ClientParts {
    /// the default headers for every request
    pub(crate) headers: reqwest::header::HeaderMap,
    /// the only CA certificate to trust, if one was configured
    pub(crate) ca_certificate: Option<reqwest::Certificate>,
    /// the TLS client identity for client certificate auth
    pub(crate) identity: Option<reqwest::Identity>,
    /// the base URL of the Icinga API
    pub(crate) url: url::Url,
}
