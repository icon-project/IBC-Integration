use super::*;

#[cw_serde]
pub enum CallServiceMessageType {
    CallServiceRequest = 1,
    CallServiceResponse,
}

#[cw_serde]
pub enum CallServiceResponseType {
    CallServiceIbcError = -2,
    CallServiceResponseFailure,
    CallServiceResponseSucess,
}

#[cw_serde]
pub struct CallServiceMessage {
    message_type: CallServiceMessageType,
    payload: Vec<u8>,
}

#[cw_serde]
pub struct CallServiceMessageRequest {
    from: Address,
    to: String,
    sequence_no: u128,
    rollback: Vec<u8>,
    data: Vec<u8>,
}

#[cw_serde]
pub struct CallServiceMessageReponse {
    sequence_no: u128,
    response_code: CallServiceResponseType,
    message: Vec<u8>,
}
