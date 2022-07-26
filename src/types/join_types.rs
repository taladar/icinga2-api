//! The possible Joins for each API query type supporting them

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub mod dependency;
pub mod host;
pub mod notification;
pub mod service;
pub mod user;
pub mod zone;

/// a marker trait for all the various join types for the different objects
pub trait IcingaJoinType {}

/// joins
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum IcingaJoinResult<T> {
    /// a full result we get if we just specified e.g. joins=host
    Full(T),
    /// a partial result we get if we specified individual fields, e.g. joins=host.name
    Partial(BTreeMap<String, serde_json::Value>),
}

/// shared code for all the handlers that have a joins parameters
pub(crate) fn add_joins_to_url<JT: IcingaJoinType + Ord + std::fmt::Display>(
    url: &mut url::Url,
    joins: &IcingaJoins<JT>,
) -> Result<(), crate::error::Error> {
    match joins {
        IcingaJoins::NoJoins => (),
        IcingaJoins::AllJoins => {
            url.query_pairs_mut().append_pair("all_joins", "1");
        }
        IcingaJoins::SpecificJoins { full, partial } => {
            for j in full {
                url.query_pairs_mut().append_pair("joins", &j.to_string());
            }
            for (j, fields) in partial {
                for f in fields {
                    url.query_pairs_mut()
                        .append_pair("joins", &format!("{}.{}", &j.to_string(), &f));
                }
            }
        }
    }
    Ok(())
}
