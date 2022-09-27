//! Helper types related to API Actions

use serde::{Deserialize, Serialize};

/// result of Action API calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// the HTTP status code, as a float because Icinga is strange
    pub code: f64,
    /// a textual status response
    pub status: String,
}

/// result of the generate-ticket action API call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateTicketResponse {
    /// the HTTP status code, as a float because Icinga is strange
    pub code: f64,
    /// a textual status response
    pub status: String,
    /// in case of success the ticket generated
    pub ticket: Option<String>,
}

/// result of the execute-command action API call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteCommandResponse {
    /// the HTTP status code, as a float because Icinga is strange
    pub code: f64,
    /// a textual status response
    pub status: String,
    /// the checkable (host or service) on which the command is run
    pub checkable: Option<String>,
    /// the execution UUID
    pub execution: Option<String>,
}
