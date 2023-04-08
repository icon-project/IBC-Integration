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
        packet_data: Vec<u8>,
    },

    VerifyNextSequenceRecv {
        height: String,
        prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        seq_recv_path: Vec<u8>,
        sequence: u64,
        packet_data: Vec<u8>,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PacketResponse {
    pub seq_on_a: Sequence,
    pub port_id_on_a: IbcPortId,
    pub chan_id_on_a: IbcChannelId,
    pub port_id_on_b: IbcPortId,
    pub chan_id_on_b: IbcChannelId,
    pub data: String,
    pub timeout_height_on_b: TimeoutHeight,
    pub timeout_timestamp_on_b: Timestamp,
}

impl From<PacketResponse> for Packet {
    fn from(packet: PacketResponse) -> Self {
        let data = hex::decode(packet.data).unwrap();
        Packet {
            seq_on_a: packet.seq_on_a,
            port_id_on_a: packet.port_id_on_a,
            chan_id_on_a: packet.chan_id_on_a,
            port_id_on_b: packet.port_id_on_b,
            chan_id_on_b: packet.chan_id_on_b,
            data: data,
            timeout_height_on_b: packet.timeout_height_on_b,
            timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PacketDataResponse {
    pub packet: PacketResponse,
    pub signer: Signer,
}
