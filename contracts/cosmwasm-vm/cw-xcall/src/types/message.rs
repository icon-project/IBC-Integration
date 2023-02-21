use super::*;

#[cw_serde]
pub enum CallServiceMessageType {
    CallServiceRequest = 1,
    CallServiceResponse,
}

#[cw_serde]
pub struct CallServiceMessage {
    message_type: CallServiceMessageType,
    payload: Vec<u8>,
}

impl CallServiceMessage {
    pub fn new(message_type: CallServiceMessageType, payload: Vec<u8>) -> Self {
        Self {
            message_type,
            payload,
        }
    }
}
