//! structs and trait related to the query joins and all_joins parameters
//! and the result of joins
use std::collections::BTreeMap;

use serde::Deserialize;

/// a marker trait for all the various join types for the different objects
pub trait IcingaJoinType {}

/// joins
#[derive(Debug)]
pub enum IcingaJoins<'a, JT>
where
    JT: IcingaJoinType + Ord + std::fmt::Display,
{
    /// do not include any joins
    NoJoins,
    /// include specific joins
    SpecificJoins {
        /// include the full objects for these joins
        full: Vec<JT>,
        /// include just the specified fields for these joins
        partial: BTreeMap<JT, Vec<&'a str>>,
    },
    /// include full objects for all possible joins
    AllJoins,
}

/// return type for joins, either full or partial
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IcingaJoinResult<T> {
    /// a full result we get if we just specified e.g. joins=host
    Full(T),
    /// a partial result we get if we specified individual fields, e.g. joins=host.name
    Partial(BTreeMap<String, serde_json::Value>),
}
