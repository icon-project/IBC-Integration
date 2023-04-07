use ibc::signer::Signer;

use super::*;

pub const VALIDATE_ON_PACKET_TIMEOUT_ON_LIGHT_CLIENT: u64 = 54;
pub const VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE: u64 = 54;

#[cw_serde]
pub enum LightClientPacketMessage {
    VerifyPacketReceiptAbsence {
        height: String,
        prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        receipt_path: Vec<u8>,
    },

    VerifyNextSequenceRecv {
        height: String,
        prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        seq_recv_path: Vec<u8>,
        sequence: u64,
    },
}

pub enum TimeoutMsgType {
    Timeout(MsgTimeout),
    TimeoutOnClose(MsgTimeoutOnClose),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PacketData {
    pub packet: Packet,
    pub signer: Signer,
}
