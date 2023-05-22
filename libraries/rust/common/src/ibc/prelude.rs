pub use core::prelude::v1::*;

// Re-export according to alloc::prelude::v1 because it is not yet stabilized
// https://doc.rust-lang.org/src/alloc/prelude/v1.rs.html
pub use prost::alloc::borrow::ToOwned;
pub use prost::alloc::boxed::Box;
pub use prost::alloc::string::{String, ToString};
pub use prost::alloc::vec::Vec;

pub use prost::alloc::format;
pub use prost::alloc::vec;
