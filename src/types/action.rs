//! Helper types related to API Actions

use serde::{Deserialize, Serialize};

/// result of Action API calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// the HTTP status code, as a float because Icinga is strange
    code: f64,
    /// a textual status response
    status: String,
}
