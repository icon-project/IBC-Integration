//! This module implements the processing logic for ICS4 (channel) messages.
use crate::ibc::events::ModuleEvent;
use crate::ibc::prelude::*;

#[derive(Clone, Debug)]
pub struct ModuleExtras {
    pub events: Vec<ModuleEvent>,
    pub log: Vec<String>,
}

impl ModuleExtras {
    pub fn empty() -> Self {
        ModuleExtras {
            events: Vec::new(),
            log: Vec::new(),
        }
    }
}
