//! Definition for the various Event Stream Types
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-streams)

use serde::{Deserialize, Serialize};

pub mod acknowledgement_cleared;
pub mod acknowledgement_set;
pub mod check_result;
pub mod comment_added;
pub mod comment_removed;
pub mod downtime_added;
pub mod downtime_removed;
pub mod downtime_started;
pub mod downtime_triggered;
pub mod flapping;
pub mod notification;
pub mod object_created;
pub mod object_deleted;
pub mod object_modified;
pub mod state_change;

/// the enum used to deserialize events from an event stream
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IcingaEvent {
    /// a new check result
    CheckResult(check_result::IcingaEventCheckResult),
    /// a host or service state changed
    StateChange(state_change::IcingaEventStateChange),
    /// a notification was sent
    Notification(notification::IcingaEventNotification),
    /// a problem was acknowledged
    AcknowledgementSet(acknowledgement_set::IcingaEventAcknowledgementSet),
    /// an acknowledgement was removed
    AcknowledgementCleared(acknowledgement_cleared::IcingaEventAcknowledgementCleared),
    /// a comment was added
    CommentAdded(comment_added::IcingaEventCommentAdded),
    /// a comment was removed
    CommentRemove(comment_removed::IcingaEventCommentRemoved),
    /// a downtime was added
    DowntimeAdded(downtime_added::IcingaEventDowntimeAdded),
    /// a downtime was removed
    DowntimeRemoved(downtime_removed::IcingaEventDowntimeRemoved),
    /// a downtime started
    DowntimeStarted(downtime_started::IcingaEventDowntimeStarted),
    /// a downtime was triggered by another downtime
    DowntimeTriggered(downtime_triggered::IcingaEventDowntimeTriggered),
    /// an object was created
    ObjectCreated(object_created::IcingaEventObjectCreated),
    /// an object was deleted
    ObjectDeleted(object_deleted::IcingaEventObjectDeleted),
    /// an object was modified
    ObjectModified(object_modified::IcingaEventObjectModified),
    /// flapping status changed
    Flapping(flapping::IcingaEventFlapping),
}
