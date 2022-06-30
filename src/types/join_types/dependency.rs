//! Dependency

use serde::{Deserialize, Serialize};

use crate::types::monitoring_objects::{
    host::IcingaHost, service::IcingaService, time_period::IcingaTimePeriod,
};

use super::{IcingaJoinResult, IcingaJoinType};

/// possible joins parameter values for dependencies
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IcingaDependencyJoinTypes {
    /// the child host of the dependency
    ChildHost,
    /// the child service of the dependency
    ChildService,
    /// the parent host of the dependency
    ParentHost,
    /// the parent service of the dependency
    ParentService,
    /// the period object for which the dependency is valid
    Period,
}

impl IcingaJoinType for IcingaDependencyJoinTypes {}

impl std::fmt::Display for IcingaDependencyJoinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcingaDependencyJoinTypes::ChildHost => write!(f, "child_host"),
            IcingaDependencyJoinTypes::ChildService => write!(f, "child_service"),
            IcingaDependencyJoinTypes::ParentHost => write!(f, "parent_host"),
            IcingaDependencyJoinTypes::ParentService => write!(f, "parent_service"),
            IcingaDependencyJoinTypes::Period => write!(f, "period"),
        }
    }
}

/// return type joins for dependencies
#[derive(Debug, Serialize, Deserialize)]
pub struct IcingaDependencyJoins {
    /// the child host of the dependency
    pub child_host: Option<IcingaJoinResult<IcingaHost>>,
    /// the child service of the dependency
    pub child_service: Option<IcingaJoinResult<IcingaService>>,
    /// the parent host of the dependency
    pub parent_host: Option<IcingaJoinResult<IcingaHost>>,
    /// the parent service of the dependency
    pub parent_service: Option<IcingaJoinResult<IcingaService>>,
    /// the time period for which this dependency applies
    pub period: Option<IcingaJoinResult<IcingaTimePeriod>>,
}
