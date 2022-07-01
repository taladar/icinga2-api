//! Actions
//!
//! [Official Documentation](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#actions)

pub mod acknowledge_problem;
pub mod add_comment;
pub mod delay_notification;
pub mod execute_command;
pub mod generate_ticket;
pub mod process_check_result;
pub mod remove_acknowledgement;
pub mod remove_comment;
pub mod remove_downtime;
pub mod reschedule_check;
pub mod restart_process;
pub mod schedule_downtime;
pub mod send_custom_notification;
pub mod shutdown_process;
