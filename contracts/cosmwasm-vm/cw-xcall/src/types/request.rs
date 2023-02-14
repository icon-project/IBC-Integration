use std::collections::HashMap;

use super::*;

#[cw_serde]
pub struct CallServiceMessageRequest {
    from: Address,
    to: String,
    sequence_no: u128,
    rollback: Vec<u8>,
    data: Vec<u8>,
}

impl CallServiceMessageRequest {
    pub fn new(
        from: Address,
        to: String,
        sequence_no: u128,
        rollback: Vec<u8>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            from,
            to,
            sequence_no,
            rollback,
            data,
        }
    }

    pub fn from(&self) -> &Address {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn sequence_no(&self) -> u128 {
        self.sequence_no
    }

    pub fn rollback(&self) -> &[u8] {
        &self.rollback
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[cw_serde]
pub struct CSMessageRequests(HashMap<u128, CallServiceMessageRequest>);

impl Default for CSMessageRequests {
    fn default() -> Self {
        Self::new()
    }
}

impl CSMessageRequests {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, sequence_no: u128, cs_message_request: CallServiceMessageRequest) {
        self.0.insert(sequence_no, cs_message_request);
    }

    pub fn remove(&mut self, sequence_no: u128) {
        self.0.remove(&sequence_no);
    }

    pub fn contains(&self, sequence_no: u128) -> bool {
        self.0.contains_key(&sequence_no)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
