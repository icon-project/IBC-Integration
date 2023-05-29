use common::rlp;
use common::rlp::{Decodable, Encodable};
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct XCallPacket {
    // CallServiceMessage
    msg: Vec<u8>,
    sn: u64,
}

impl Encodable for XCallPacket {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream.begin_list(2).append(&self.msg).append(&self.sn);
    }
}

impl Decodable for XCallPacket {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        Ok(Self {
            msg: rlp.val_at(1)?,
            sn: rlp.val_at(0)?,
        })
    }
}
