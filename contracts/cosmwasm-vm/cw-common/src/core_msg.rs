use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::hex_string::HexString;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    // Client Messages
    RegisterClient {
        client_type: String,
        client_address: Addr,
    },
    CreateClient {
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit,
        msg: HexString,
    },
    UpdateClient {
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit,
        msg: HexString,
    },
    // Not included in this version of ibc core
    UpgradeClient {},

    ClientMisbehaviour {},

    // Connection Messsages
    ConnectionOpenInit {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit,
        msg: HexString,
    },
    ConnectionOpenTry {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry
        msg: HexString,
    },
    ConnectionOpenAck {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenAck
        msg: HexString,
    },
    ConnectionOpenConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenConfirm
        msg: HexString,
    },

    // Channel Messages
    ChannelOpenInit {
        // raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenInit
        msg: HexString,
    },
    ChannelOpenTry {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenTry
        msg: HexString,
    },
    ChannelOpenAck {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenAck
        msg: HexString,
    },
    ChannelOpenConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenConfirm
        msg: HexString,
    },
    ChannelCloseInit {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelCloseInit
        msg: HexString,
    },
    ChannelCloseConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelCloseConfirm
        msg: HexString,
    },

    // Packet Messages
    SendPacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::Packet
        packet: HexString,
    },
    ReceivePacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgRecvPacket
        msg: HexString,
    },
    AcknowledgementPacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgAcknowledgement
        msg: HexString,
    },
    RequestTimeout {},
    Timeout {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgTimeout
        msg: HexString,
    },
    TimeoutOnClose {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgTimeoutOnClose
        msg: HexString,
    },

    // Storage Messages
    BindPort {
        port_id: String,
        address: String,
    },
    SetExpectedTimePerBlock {
        block_time: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    GetCommitment { key: HexString },
    #[returns(Addr)]
    GetClientRegistry { _type: String },
    #[returns(String)]
    GetClientType { client_id: String },
    #[returns(Addr)]
    GetClientImplementation { client_id: String },
    #[returns(String)]
    GetConnection { connection_id: String },
    #[returns(String)]
    GetChannel { port_id: String, channel_id: String },
    #[returns(u64)]
    GetNextSequenceSend { port_id: String, channel_id: String },
    #[returns(u64)]
    GetNextSequenceReceive { port_id: String, channel_id: String },
    #[returns(u64)]
    GetNextSequenceAcknowledgement { port_id: String, channel_id: String },
    #[returns(Vec<String>)]
    GetCapability { name: String },
    #[returns(u64)]
    GetExpectedTimePerBlock {},
    #[returns(u64)]
    GetNextClientSequence {},
    #[returns(u64)]
    GetNextConnectionSequence {},
    #[returns(u64)]
    GetNextChannelSequence {},
    #[returns(String)]
    GetClientState { client_id: String },
    #[returns(String)]
    GetConsensusState { client_id: String },
    #[returns(bool)]
    GetPacketReceipt {
        port_id: String,
        channel_id: String,
        sequence: u64,
    },
    #[returns(String)]
    GetPacketCommitment {
        port_id: String,
        channel_id: String,
        sequence: u64,
    },
    #[returns(String)]
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
    #[returns(Vec<String>)]
    GetAllPorts {},
}
