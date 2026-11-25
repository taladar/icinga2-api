//! Icinga instance config files, one TOML file per Icinga master
//!
//! A directory of these files can be shared by several tools that talk to the
//! same Icinga masters. The file stem is the instance name. A file looks like
//! this:
//!
//! ```toml
//! url = "https://icinga.example.com:5665"
//! ca_certificate = "/path/to/ca.pem"
//! web_base_url = "https://icinga.example.com/monitoring/"
//! username = "api-user"
//! password_command = ["pass", "show", "icinga/api-user"]
//! password_command_env = { PASSWORD_STORE_DIR = "/path/to/store" }
//! ```
//!
//! Instead of `password_command` a literal `password` can be used, and
//! instead of `username` and a password a TLS client certificate can be used
//! with `client_certificate` and `client_key` (PEM files), e.g. the
//! certificate of an Icinga agent for an `ApiUser` with a `client_cn`.

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::config::Icinga2Auth;

/// errors when loading instance config files or resolving their auth
#[derive(Debug, thiserror::Error)]
pub enum Icinga2InstanceFileError {
    /// could not read the instance config directory
    #[error("could not read instance config directory: {0}")]
    ReadDir(std::io::Error),
    /// could not read an instance config file
    #[error("could not read instance config file: {0}")]
    ReadFile(std::io::Error),
    /// could not parse an instance config file
    #[error("could not parse instance config file {path}: {source}")]
    Parse {
        /// the file that could not be parsed
        path: PathBuf,
        /// the parse error
        source: toml::de::Error,
    },
    /// the password command is empty
    #[error("password_command is empty")]
    PasswordCommandEmpty,
    /// the password command could not be started
    #[error("could not run password command `{command}`: {source}")]
    PasswordCommandExec {
        /// the command line
        command: String,
        /// the error starting it
        source: std::io::Error,
    },
    /// the password command failed
    #[error("password command `{command}` failed with exit code {exit_code:?}")]
    PasswordCommandFailed {
        /// the command line
        command: String,
        /// its exit code, `None` if it was killed by a signal
        exit_code: Option<i32>,
    },
    /// the password command did not finish in time
    #[error("password command `{command}` timed out after {timeout:?}")]
    PasswordCommandTimeout {
        /// the command line
        command: String,
        /// the timeout that expired
        timeout: std::time::Duration,
    },
    /// the password command output was not UTF-8
    #[error("password command `{command}` printed invalid UTF-8: {source}")]
    PasswordCommandOutput {
        /// the command line
        command: String,
        /// the decoding error
        source: std::string::FromUtf8Error,
    },
    /// the password command printed nothing
    #[error("password command `{command}` printed no password")]
    PasswordCommandNoPassword {
        /// the command line
        command: String,
    },
    /// could not read the client certificate or key
    #[error("could not read client certificate or key: {0}")]
    ReadClientCertificate(std::io::Error),
    /// could not build the client
    #[error(transparent)]
    Client(#[from] crate::error::Error),
}

/// how an instance config file authenticates
#[derive(Clone)]
pub enum Icinga2InstanceFileAuth {
    /// basic auth with a literal password from the file
    Password {
        /// the name of the `ApiUser`
        username: String,
        /// the password
        password: String,
    },
    /// basic auth with a password printed by a command
    PasswordCommand {
        /// the name of the `ApiUser`
        username: String,
        /// program and arguments, the first line of its output is the password
        password_command: Vec<String>,
        /// extra environment variables for the command, e.g. `PASSWORD_STORE_DIR`
        password_command_env: BTreeMap<String, String>,
    },
    /// TLS client certificate auth
    ClientCertificate {
        /// the PEM certificate file
        certificate: PathBuf,
        /// the PEM private key file
        key: PathBuf,
    },
}

impl std::fmt::Debug for Icinga2InstanceFileAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Password { username, .. } => f
                .debug_struct("Password")
                .field("username", username)
                .field("password", &"<redacted>")
                .finish(),
            Self::PasswordCommand {
                username,
                password_command,
                password_command_env,
            } => f
                .debug_struct("PasswordCommand")
                .field("username", username)
                .field("password_command", password_command)
                .field("password_command_env", password_command_env)
                .finish(),
            Self::ClientCertificate { certificate, key } => f
                .debug_struct("ClientCertificate")
                .field("certificate", certificate)
                .field("key", key)
                .finish(),
        }
    }
}

/// one Icinga instance config file
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "RawInstanceFile")]
pub struct Icinga2InstanceFile {
    /// the API URL, without the v1 component, e.g. `https://icinga.example.com:5665`
    pub url: String,
    /// the CA certificate to validate the server certificate with
    pub ca_certificate: Option<PathBuf>,
    /// the Icinga Web base URL, e.g. `https://icinga.example.com/monitoring/`,
    /// not used by this crate, for tools that link to the web interface
    pub web_base_url: Option<String>,
    /// how to authenticate
    pub auth: Icinga2InstanceFileAuth,
}

/// the instance config file as written, before checking which auth it uses
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawInstanceFile {
    /// see [`Icinga2InstanceFile::url`]
    url: String,
    /// see [`Icinga2InstanceFile::ca_certificate`]
    ca_certificate: Option<PathBuf>,
    /// see [`Icinga2InstanceFile::web_base_url`]
    web_base_url: Option<String>,
    /// basic auth username
    username: Option<String>,
    /// literal basic auth password
    password: Option<String>,
    /// command printing the basic auth password
    password_command: Option<Vec<String>>,
    /// environment for the password command
    password_command_env: Option<BTreeMap<String, String>>,
    /// client certificate file
    client_certificate: Option<PathBuf>,
    /// client key file
    client_key: Option<PathBuf>,
}

impl TryFrom<RawInstanceFile> for Icinga2InstanceFile {
    type Error = String;

    fn try_from(raw: RawInstanceFile) -> Result<Self, Self::Error> {
        let auth = match (
            raw.username,
            raw.password,
            raw.password_command,
            raw.client_certificate,
            raw.client_key,
        ) {
            (Some(username), Some(password), None, None, None) => {
                if raw.password_command_env.is_some() {
                    return Err("password_command_env requires password_command".to_owned());
                }
                Icinga2InstanceFileAuth::Password { username, password }
            }
            (Some(username), None, Some(password_command), None, None) => {
                Icinga2InstanceFileAuth::PasswordCommand {
                    username,
                    password_command,
                    password_command_env: raw.password_command_env.unwrap_or_default(),
                }
            }
            (None, None, None, Some(certificate), Some(key)) => {
                if raw.password_command_env.is_some() {
                    return Err("password_command_env requires password_command".to_owned());
                }
                Icinga2InstanceFileAuth::ClientCertificate { certificate, key }
            }
            _ => {
                return Err("expected exactly one of: username with password, \
                     username with password_command, \
                     or client_certificate with client_key"
                    .to_owned());
            }
        };
        Ok(Self {
            url: raw.url,
            ca_certificate: raw.ca_certificate,
            web_base_url: raw.web_base_url,
            auth,
        })
    }
}

/// the first line of the password command output, trimmed
///
/// # Errors
///
/// fails if the command failed, its output is not UTF-8 or it is empty
fn password_from_output(
    command: &str,
    output: std::process::Output,
) -> Result<String, Icinga2InstanceFileError> {
    if !output.status.success() {
        return Err(Icinga2InstanceFileError::PasswordCommandFailed {
            command: command.to_owned(),
            exit_code: output.status.code(),
        });
    }
    let stdout = String::from_utf8(output.stdout).map_err(|source| {
        Icinga2InstanceFileError::PasswordCommandOutput {
            command: command.to_owned(),
            source,
        }
    })?;
    let password = stdout.lines().next().unwrap_or_default().trim();
    if password.is_empty() {
        return Err(Icinga2InstanceFileError::PasswordCommandNoPassword {
            command: command.to_owned(),
        });
    }
    Ok(password.to_owned())
}

impl Icinga2InstanceFile {
    /// read one instance config file
    ///
    /// # Errors
    ///
    /// fails if the file can not be read or parsed
    pub fn from_file(path: &Path) -> Result<Self, Icinga2InstanceFileError> {
        let content = fs_err::read_to_string(path).map_err(Icinga2InstanceFileError::ReadFile)?;
        toml::from_str(&content).map_err(|source| Icinga2InstanceFileError::Parse {
            path: path.to_path_buf(),
            source,
        })
    }

    /// read all `*.toml` files in a directory, sorted by name
    ///
    /// the name of each instance is its file stem
    ///
    /// # Errors
    ///
    /// fails if the directory or any of the files can not be read or parsed
    pub fn load_dir(dir: &Path) -> Result<Vec<(String, Self)>, Icinga2InstanceFileError> {
        let mut result = Vec::new();
        for entry in fs_err::read_dir(dir).map_err(Icinga2InstanceFileError::ReadDir)? {
            let path = entry.map_err(Icinga2InstanceFileError::ReadDir)?.path();
            if path.extension().and_then(|s| s.to_str()) != Some("toml") {
                continue;
            }
            let Some(name) = path.file_stem().map(|s| s.to_string_lossy().into_owned()) else {
                continue;
            };
            result.push((name, Self::from_file(&path)?));
        }
        result.sort_by(|(a, _), (b, _)| a.cmp(b));
        Ok(result)
    }

    /// the basic auth username, `None` for client certificate auth
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match &self.auth {
            Icinga2InstanceFileAuth::Password { username, .. }
            | Icinga2InstanceFileAuth::PasswordCommand { username, .. } => Some(username),
            Icinga2InstanceFileAuth::ClientCertificate { .. } => None,
        }
    }

    /// resolve the auth, running the password command or reading the client
    /// certificate if necessary
    ///
    /// the password command gets no stdin and inherits stderr
    ///
    /// # Errors
    ///
    /// fails if the password command fails or prints no password, or if the
    /// client certificate or key can not be read
    pub fn resolve_auth(&self) -> Result<Icinga2Auth, Icinga2InstanceFileError> {
        match &self.auth {
            Icinga2InstanceFileAuth::Password { username, password } => Ok(Icinga2Auth::Basic {
                username: username.to_owned(),
                password: password.to_owned(),
            }),
            Icinga2InstanceFileAuth::PasswordCommand {
                username,
                password_command,
                password_command_env,
            } => {
                let (program, args) = password_command
                    .split_first()
                    .ok_or(Icinga2InstanceFileError::PasswordCommandEmpty)?;
                let command = password_command.join(" ");
                let output = std::process::Command::new(program)
                    .args(args)
                    .envs(password_command_env)
                    .stdin(std::process::Stdio::null())
                    .stderr(std::process::Stdio::inherit())
                    .output()
                    .map_err(|source| Icinga2InstanceFileError::PasswordCommandExec {
                        command: command.clone(),
                        source,
                    })?;
                Ok(Icinga2Auth::Basic {
                    username: username.to_owned(),
                    password: password_from_output(&command, output)?,
                })
            }
            Icinga2InstanceFileAuth::ClientCertificate { certificate, key } => {
                Icinga2Auth::client_certificate_from_files(certificate, key)
                    .map_err(Icinga2InstanceFileError::ReadClientCertificate)
            }
        }
    }

    /// like [`Self::resolve_auth`] but runs the password command
    /// asynchronously, killing it if it does not finish within `timeout`
    /// (e.g. while a password store waits for a locked key)
    ///
    /// # Errors
    ///
    /// fails like [`Self::resolve_auth`] or if the timeout expires
    #[cfg(feature = "async")]
    pub async fn resolve_auth_async(
        &self,
        timeout: Option<std::time::Duration>,
    ) -> Result<Icinga2Auth, Icinga2InstanceFileError> {
        let Icinga2InstanceFileAuth::PasswordCommand {
            username,
            password_command,
            password_command_env,
        } = &self.auth
        else {
            return self.resolve_auth();
        };
        let (program, args) = password_command
            .split_first()
            .ok_or(Icinga2InstanceFileError::PasswordCommandEmpty)?;
        let command = password_command.join(" ");
        let output_future = tokio::process::Command::new(program)
            .args(args)
            .envs(password_command_env)
            .stdin(std::process::Stdio::null())
            .stderr(std::process::Stdio::inherit())
            .kill_on_drop(true)
            .output();
        let output = if let Some(timeout) = timeout {
            tokio::time::timeout(timeout, output_future)
                .await
                .map_err(
                    |_elapsed| Icinga2InstanceFileError::PasswordCommandTimeout {
                        command: command.clone(),
                        timeout,
                    },
                )?
        } else {
            output_future.await
        }
        .map_err(|source| Icinga2InstanceFileError::PasswordCommandExec {
            command: command.clone(),
            source,
        })?;
        Ok(Icinga2Auth::Basic {
            username: username.to_owned(),
            password: password_from_output(&command, output)?,
        })
    }

    /// resolve the auth and build a blocking client
    ///
    /// # Errors
    ///
    /// fails if the auth can not be resolved or the client can not be built
    #[cfg(feature = "blocking")]
    pub fn connect_blocking(
        &self,
    ) -> Result<crate::api::blocking::Icinga2, Icinga2InstanceFileError> {
        Ok(crate::api::blocking::Icinga2::from_parts(
            &self.url,
            self.ca_certificate.as_deref(),
            self.resolve_auth()?,
        )?)
    }

    /// resolve the auth asynchronously (see [`Self::resolve_auth_async`]) and
    /// build an async client
    ///
    /// # Errors
    ///
    /// fails if the auth can not be resolved or the client can not be built
    #[cfg(feature = "async")]
    pub async fn connect_async(
        &self,
        timeout: Option<std::time::Duration>,
    ) -> Result<crate::api::async_client::Icinga2Async, Icinga2InstanceFileError> {
        Ok(crate::api::async_client::Icinga2Async::from_parts(
            &self.url,
            self.ca_certificate.as_deref(),
            self.resolve_auth_async(timeout).await?,
        )?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_password_command() -> Result<(), Box<dyn std::error::Error>> {
        let file: Icinga2InstanceFile = toml::from_str(
            r#"
url = "https://icinga.example.com:5665"
ca_certificate = "/path/to/ca.pem"
username = "api-user"
password_command = [
  "pass",
  "show",
  "icinga/api-user",
]
password_command_env = { PASSWORD_STORE_DIR = "/path/to/store" }
"#,
        )?;
        assert_eq!(file.url, "https://icinga.example.com:5665");
        assert_eq!(file.ca_certificate, Some(PathBuf::from("/path/to/ca.pem")));
        assert_eq!(file.web_base_url, None);
        assert_eq!(file.username(), Some("api-user"));
        let Icinga2InstanceFileAuth::PasswordCommand {
            password_command,
            password_command_env,
            ..
        } = file.auth
        else {
            return Err("expected password command auth".into());
        };
        assert_eq!(password_command, vec!["pass", "show", "icinga/api-user"]);
        assert_eq!(
            password_command_env
                .get("PASSWORD_STORE_DIR")
                .map(String::as_str),
            Some("/path/to/store")
        );
        Ok(())
    }

    #[test]
    fn test_parse_password_and_web_base_url() -> Result<(), Box<dyn std::error::Error>> {
        let file: Icinga2InstanceFile = toml::from_str(
            r#"
url = "https://icinga.example.com:5665"
web_base_url = "https://icinga.example.com/monitoring/"
username = "api-user"
password = "secret"
"#,
        )?;
        assert_eq!(
            file.web_base_url.as_deref(),
            Some("https://icinga.example.com/monitoring/")
        );
        let Icinga2Auth::Basic { username, password } = file.resolve_auth()? else {
            return Err("expected basic auth".into());
        };
        assert_eq!(username, "api-user");
        assert_eq!(password, "secret");
        Ok(())
    }

    #[test]
    fn test_parse_client_certificate() -> Result<(), Box<dyn std::error::Error>> {
        let file: Icinga2InstanceFile = toml::from_str(
            r#"
url = "https://icinga.example.com:5665"
ca_certificate = "/var/lib/icinga2/certs/ca.crt"
client_certificate = "/var/lib/icinga2/certs/host.example.com.crt"
client_key = "/var/lib/icinga2/certs/host.example.com.key"
"#,
        )?;
        assert_eq!(file.username(), None);
        assert!(
            matches!(file.auth, Icinga2InstanceFileAuth::ClientCertificate { .. }),
            "expected client certificate auth"
        );
        Ok(())
    }

    #[test]
    fn test_reject_ambiguous_or_unknown() {
        for content in [
            // password and password command
            "url = \"https://x:5665\"\nusername = \"u\"\npassword = \"p\"\npassword_command = [\"true\"]\n",
            // no username
            "url = \"https://x:5665\"\npassword = \"p\"\n",
            // certificate without key
            "url = \"https://x:5665\"\nclient_certificate = \"/c\"\n",
            // unknown field
            "url = \"https://x:5665\"\nusername = \"u\"\npassword = \"p\"\npasword_command = []\n",
        ] {
            assert!(
                toml::from_str::<Icinga2InstanceFile>(content).is_err(),
                "should be rejected: {content}"
            );
        }
    }

    #[test]
    fn test_resolve_password_command() -> Result<(), Box<dyn std::error::Error>> {
        let file: Icinga2InstanceFile = toml::from_str(
            r#"
url = "https://icinga.example.com:5665"
username = "api-user"
password_command = ["sh", "-c", "printf '%s\nsecond line\n' \"$SECRET\""]
password_command_env = { SECRET = " from-env " }
"#,
        )?;
        let Icinga2Auth::Basic { password, .. } = file.resolve_auth()? else {
            return Err("expected basic auth".into());
        };
        assert_eq!(password, "from-env");
        Ok(())
    }

    #[test]
    fn test_resolve_failing_password_command() -> Result<(), Box<dyn std::error::Error>> {
        let file: Icinga2InstanceFile = toml::from_str(
            r#"
url = "https://icinga.example.com:5665"
username = "api-user"
password_command = ["false"]
"#,
        )?;
        assert!(
            matches!(
                file.resolve_auth(),
                Err(Icinga2InstanceFileError::PasswordCommandFailed { .. })
            ),
            "expected a failed password command"
        );
        Ok(())
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_resolve_password_command_timeout() -> Result<(), Box<dyn std::error::Error>> {
        let file: Icinga2InstanceFile = toml::from_str(
            r#"
url = "https://icinga.example.com:5665"
username = "api-user"
password_command = ["sleep", "10"]
"#,
        )?;
        assert!(
            matches!(
                file.resolve_auth_async(Some(std::time::Duration::from_millis(100)))
                    .await,
                Err(Icinga2InstanceFileError::PasswordCommandTimeout { .. })
            ),
            "expected a timeout"
        );
        Ok(())
    }

    #[test]
    fn test_debug_redacts_password() -> Result<(), Box<dyn std::error::Error>> {
        let file: Icinga2InstanceFile =
            toml::from_str("url = \"https://x:5665\"\nusername = \"u\"\npassword = \"hunter2\"\n")?;
        assert!(
            !format!("{file:?}").contains("hunter2"),
            "password in debug output"
        );
        assert!(
            !format!("{:?}", file.resolve_auth()?).contains("hunter2"),
            "password in debug output"
        );
        Ok(())
    }
}
