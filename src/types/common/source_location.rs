//! SourceLocation
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/base/configobject.cpp)
//!
//! Strictly speaking on the Icinga side this is defined as a dictionary with arbitrary keys but it is only
//! ever used with these five keys

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// an icinga source location inside the icinga config files
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IcingaSourceLocation {
    /// path of the config file
    pub path: PathBuf,
    /// start line
    pub first_line: u64,
    /// start column
    pub first_column: u64,
    /// end line
    pub last_line: u64,
    /// end column
    pub last_column: u64,
}
