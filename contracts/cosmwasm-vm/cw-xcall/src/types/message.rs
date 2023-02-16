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

impl Decodable for CallServiceMessage {
    fn decode(rlp: &common::rlp::Rlp) -> Result<Self, common::rlp::DecoderError> {
        todo!()
    }
}
