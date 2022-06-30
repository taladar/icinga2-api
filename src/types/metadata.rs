//! structs related to the query metadata parameter
//! and the result of queries including metadata
use serde::{Deserialize, Serialize};

use super::common::{object::IcingaObject, source_location::IcingaSourceLocation};

/// possible meta parameter values
#[derive(Debug, Clone)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaMetadata {
    /// which other icinga objects use this object
    pub used_by: Option<Vec<IcingaObject>>,
    /// where in the config file this object is defined
    pub location: Option<IcingaSourceLocation>,
}

/// shared code for all handlers that have a meta parameter
pub(crate) fn add_meta_to_url(
    url: &mut url::Url,
    meta: &[IcingaMetadataType],
) -> Result<(), crate::error::Error> {
    if !meta.is_empty() {
        for v in meta {
            url.query_pairs_mut().append_pair("meta", &v.to_string());
        }
    }
    Ok(())
}
