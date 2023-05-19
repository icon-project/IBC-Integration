// TODO: disable unwraps:
// https://github.com/informalsystems/ibc-rs/issues/987
// #![cfg_attr(not(test), deny(clippy::unwrap_used))]

#[cfg(any(test, feature = "std"))]
extern crate std;

mod prelude;

pub mod core;
pub mod dynamic_typing;
mod erased;
pub mod events;
pub mod signer;
pub mod timestamp;
pub mod tx_msg;
pub mod utils;

#[cfg(feature = "serde")]
mod serializers;

/// Re-export of ICS 002 Height domain type
pub type Height = crate::ibc::core::ics02_client::height::Height;

#[cfg(test)]
pub mod mock;
