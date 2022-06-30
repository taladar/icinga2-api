//! Definition for the various Event Stream Types
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#event-streams)

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
