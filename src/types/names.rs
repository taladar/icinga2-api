//! type-safe names for various icinga objects

use serde::{Deserialize, Serialize};

/// a check command name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaCheckCommandName(pub String);

/// a downtime name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaDowntimeName(pub String);

/// an endpoint name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaEndpointName(pub String);

/// an event command name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaEventCommandName(pub String);

/// a host name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaHostName(pub String);

/// a host group name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaHostGroupName(pub String);

/// a notification command name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaNotificationCommandName(pub String);

/// a package name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaPackageName(pub String);

/// a scheduled downtime name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaScheduledDowntimeName(pub String);

/// a service name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaServiceName(pub String);

/// a service group name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaServiceGroupName(pub String);

/// a template name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaTemplateName(pub String);

/// a time period name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaTimePeriodName(pub String);

/// a user name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaUserName(pub String);

/// a user group name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaUserGroupName(pub String);

/// a zone name
#[derive(Debug, Serialize, Deserialize, derive_more::Display, derive_more::FromStr)]
pub struct IcingaZoneName(pub String);
