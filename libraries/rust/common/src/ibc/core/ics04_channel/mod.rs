//! ICS 04: Channel implementation that facilitates communication between
//! applications and the chains those applications are built upon.

pub mod channel;

pub mod error;
pub mod msgs;
pub mod packet;
pub mod timeout;

pub mod commitment;
mod version;
pub use version::Version;
