//! Icinga2 notifications
use serde::Deserialize;

use super::{
    host::IcingaHostAttributes, joins::IcingaJoinResult, service::IcingaServiceAttributes,
    IcingaJoinType,
};
