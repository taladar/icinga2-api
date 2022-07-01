//! Icinga API

pub mod action;
pub mod query;

#[cfg(feature = "async")]
pub mod async_client;
#[cfg(feature = "blocking")]
pub mod blocking;
