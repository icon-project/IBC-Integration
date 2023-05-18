use cw_common::hex_string::HexString;

use super::*;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<u8>)]
    GetCommitment { key: HexString },
    #[returns(Addr)]
    GetClientRegistry { _type: String },
    #[returns(String)]
    GetClientType { client_id: String },
    #[returns(Addr)]
    GetClientImplementation { client_id: String },
    #[returns(Vec<u8>)]
    GetConnection { connection_id: String },
    #[returns(Vec<u8>)]
    GetChannel { port_id: String, channel_id: String },
    #[returns(u64)]
    GetNextSequenceSend { port_id: String, channel_id: String },
    #[returns(u64)]
    GetNextSequenceReceive { port_id: String, channel_id: String },
    #[returns(u64)]
    GetNextSequenceAcknowledgement { port_id: String, channel_id: String },
    #[returns(Vec<String>)]
    GetCapability { name: HexString },
    #[returns(u64)]
    GetExpectedTimePerBlock,
    #[returns(u64)]
    GetNextClientSequence,
    #[returns(u64)]
    GetNextConnectionSequence,
    #[returns(u64)]
    GetNextChannelSequence,
    #[returns(Vec<u8>)]
    GetClientState { client_id: String },
    #[returns(Vec<u8>)]
    GetConsensusState {
        client_id: String,
        height: HexString,
    },
    #[returns(bool)]
    GetPacketReceipt {
        port_id: String,
        channel_id: String,
        sequence: u64,
    },
    #[returns(Vec<u8>)]
    GetPacketCommitment {
        port_id: String,
        channel_id: String,
        sequence: u64,
    },
    #[returns(Vec<u8>)]
    GetPacketAcknowledgementCommitment {
        port_id: String,
        channel_id: String,
        sequence: u64,
    },
    #[returns(bool)]
    HasPacketReceipt {
        port_id: String,
        channel_id: String,
        sequence: u64,
    },
}
