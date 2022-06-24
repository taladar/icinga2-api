//! structs related to the query metadata parameter
//! and the result of queries including metadata
use serde::Deserialize;

use super::config_object::IcingaSourceLocation;

/// possible meta parameter values
#[derive(Debug)]
pub enum IcingaMetadataType {
    /// includes information about the other icinga objects using each returned object
    UsedBy,
    /// includes information about the config file location of each returned object
    Location,
}

impl std::fmt::Display for IcingaMetadataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaMetadataType::UsedBy => write!(f, "used_by"),
            IcingaMetadataType::Location => write!(f, "location"),
        }
    }
}

/// metadata
#[derive(Debug, Deserialize)]
pub struct IcingaMetadata {
    /// which other icinga objects use this object
    pub used_by: Option<Vec<super::IcingaObject>>,
    /// where in the config file this object is defined
    pub location: Option<IcingaSourceLocation>,
}
