use std::collections::HashMap;

use super::*;

#[cw_serde]
pub enum CallServiceResponseType {
    CallServiceIbcError = -2,
    CallServiceResponseFailure,
    CallServiceResponseSucess,
}

#[cw_serde]
pub struct CallServiceMessageReponse {
    sequence_no: u128,
    response_code: CallServiceResponseType,
    message: Vec<u8>,
}

impl CallServiceMessageReponse {
    pub fn new(
        sequence_no: u128,
        response_code: CallServiceResponseType,
        message: Vec<u8>,
    ) -> Self {
        Self {
            sequence_no,
            response_code,
            message,
        }
    }

    pub fn sequence_no(&self) -> u128 {
        self.sequence_no
    }

    pub fn response_code(&self) -> &CallServiceResponseType {
        &self.response_code
    }

    pub fn message(&self) -> &[u8] {
        &self.message
    }
}

pub struct CSMessageResponse(HashMap<u128, CallServiceMessageReponse>);

impl Default for CSMessageResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl CSMessageResponse {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, sequence_no: u128, response: CallServiceMessageReponse) {
        self.0.insert(sequence_no, response);
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
