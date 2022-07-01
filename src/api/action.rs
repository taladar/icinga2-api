//! Actions
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#actions)

pub mod process_check_result;
pub mod reschedule_check;
pub mod send_custom_notification;
pub mod delay_notification;
pub mod acknowledge_problem;
pub mod remove_acknowledgement;
pub mod add_comment;
pub mod remove_comment;
pub mod schedule_downtime;
pub mod remove_downtime;
pub mod shutdown_process;
pub mod restart_process;
pub mod generate_ticket;
pub mod execute_command;
