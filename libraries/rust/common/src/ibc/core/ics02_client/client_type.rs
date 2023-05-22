use serde::{Deserialize, Serialize};

use crate::ibc::prelude::*;
use core::fmt::{Display, Error as FmtError, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ClientType(String);

impl Default for ClientType {
    fn default() -> Self {
        Self("default".to_string())
    }
}

impl ClientType {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    /// Yields this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ClientType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "ClientType({})", self.0)
    }
}
