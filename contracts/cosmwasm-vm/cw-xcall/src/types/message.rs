use super::*;

pub enum CallServiceMessageType {
    CallServiceRequest = 1,
    CallServiceResponse,
}

pub enum CallServiceResponseType {
    CallServiceIbcError = -2,
    CallServiceResponseFailure,
    CallServiceResponseSucess,
}

pub struct CallServiceMessage {
    message_type: CallServiceMessageType,
    payload: Vec<u8>,
}

pub struct CallServiceMessageRequest {
    from: Address,
    to: String,
    sequence_no: u128,
    rollback: Vec<u8>,
    data: Vec<u8>,
}

pub struct CallServiceMessageReponse {
    sequence_no: u128,
    response_code: CallServiceResponseType,
    message: Vec<u8>,
}
