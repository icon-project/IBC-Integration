use cosmwasm_std::Binary;

use super::*;

#[cw_serde]
#[derive(Default, Copy)]
pub enum CallServiceResponseType {
    CallServiceIbcError = -2,
    CallServiceResponseFailure,
    #[default]
    CallServiceResponseSucess,
}

pub fn to_int(response_type: &CallServiceResponseType) -> i8 {
    *response_type as i8
}

#[cw_serde]
#[derive(Default)]
pub struct CallServiceMessageReponse {
    sequence_no: u128,
    response_code: CallServiceResponseType,
    message: String,
}

impl CallServiceMessageReponse {
    pub fn new(sequence_no: u128, response_code: CallServiceResponseType, message: &str) -> Self {
        Self {
            sequence_no,
            response_code,
            message: message.to_string(),
        }
    }

    pub fn sequence_no(&self) -> u128 {
        self.sequence_no
    }

    pub fn response_code(&self) -> &CallServiceResponseType {
        &self.response_code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn set_fields(
        &mut self,
        sequence_no: u128,
        response_code: CallServiceResponseType,
        message: &str,
    ) {
        self.sequence_no.clone_from(&sequence_no);
        self.response_code = response_code;
        self.message = message.to_string()
    }
}
