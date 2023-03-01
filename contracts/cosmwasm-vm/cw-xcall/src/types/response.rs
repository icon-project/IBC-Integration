use common::rlp::Encodable;

use super::*;

#[cw_serde]

pub enum CallServiceResponseType {
    CallServiceIbcError = -2,
    CallServiceResponseFailure,
    CallServiceResponseSucess,
}

pub fn to_int(response_type: &CallServiceResponseType) -> i8 {
    response_type.clone() as i8
}

#[cw_serde]
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

impl Encodable for CallServiceResponseType {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream.begin_list(1);
        match self {
            CallServiceResponseType::CallServiceIbcError => stream.append::<u128>(&2),
            CallServiceResponseType::CallServiceResponseFailure => stream.append::<u128>(&1),
            CallServiceResponseType::CallServiceResponseSucess => stream.append::<u128>(&0),
        };
    }
}

impl Encodable for CallServiceMessageReponse {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream
            .begin_list(3)
            .append(&self.sequence_no())
            .append(self.response_code())
            .append(&self.message());
    }
}
