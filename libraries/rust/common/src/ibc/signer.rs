use core::str::FromStr;

use crate::ibc::prelude::*;

use derive_more::Display;

#[derive(Debug, displaydoc::Display)]
pub enum SignerError {
    /// signer cannot be empty
    EmptySigner,
}

#[cfg(feature = "std")]
impl std::error::Error for SignerError {}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Signer(String);

impl FromStr for Signer {
    type Err = SignerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        if s.trim().is_empty() {
            return Err(SignerError::EmptySigner);
        }
        Ok(Self(s))
    }
}

impl AsRef<str> for Signer {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
