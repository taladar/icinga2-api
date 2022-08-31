//! Icinga API Event Stream Types
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-streams)
//!
//! [Definition in Icinga Source](https://github.com/Icinga/icinga2/blob/master/lib/remote/eventqueue.hpp)

use serde::{Deserialize, Serialize};

/// Icinga API Event Stream Types
#[derive(
    Debug, Clone, Serialize, Deserialize, derive_more::Display, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "enumoid", derive(enumoid::Enumoid))]
pub enum IcingaEventStreamType {
    /// a new check result
    CheckResult,
    /// a host or service state changed
    StateChange,
    /// a notification was sent
    Notification,
    /// a problem was acknowledged
    AcknowledgementSet,
    /// an acknowledgement was removed
    AcknowledgementCleared,
    /// a comment was added
    CommentAdded,
    /// a comment was removed
    CommentRemove,
    /// a downtime was added
    DowntimeAdded,
    /// a downtime was removed
    DowntimeRemoved,
    /// a downtime started
    DowntimeStarted,
    /// a downtime was triggered by another downtime
    DowntimeTriggered,
    /// an object was created
    ObjectCreated,
    /// an object was deleted
    ObjectDeleted,
    /// an object was modified
    ObjectModified,
    /// flapping status changed
    Flapping,
}
