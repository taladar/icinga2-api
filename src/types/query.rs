//! Types related to query API calls

use serde::{Serialize, Deserialize};

use super::{metadata::IcingaMetadata, enums::object_type::IcingaObjectType};

/// wrapper for Json results
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsWrapper<T> {
    /// the internal field in the Icinga2 object containing all an array of the actual results
    results: Vec<T>,
}

/// the result of an icinga query to a type with joins
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResultObjectWithJoins<Obj, ObjJoins> {
    /// dependency attributes
    pub attrs: Obj,
    /// joins
    pub joins: ObjJoins,
    /// metadata, only contains data if the parameter meta contains one or more values
    pub meta: IcingaMetadata,
    /// object name
    pub name: String,
    /// type of icinga object, should always be the one matching Obj
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}

/// the result of an icinga query to a type without joins
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResultObject<Obj> {
    /// dependency attributes
    pub attrs: Obj,
    /// metadata, only contains data if the parameter meta contains one or more values
    pub meta: IcingaMetadata,
    /// object name
    pub name: String,
    /// type of icinga object, should always be the one matching Obj
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}
