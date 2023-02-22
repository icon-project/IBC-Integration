use cosmwasm_std::Binary;

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
    message: Binary,
}

impl CallServiceMessageReponse {
    pub fn new(sequence_no: u128, response_code: CallServiceResponseType, message: Binary) -> Self {
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
