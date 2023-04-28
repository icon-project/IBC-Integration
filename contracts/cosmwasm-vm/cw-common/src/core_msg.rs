use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;

use super::*;

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
        client_state: Vec<u8>,
        consensus_state: Vec<u8>,
        signer: Vec<u8>,
    },
    UpdateClient {
        client_id: String,
        header: Vec<u8>,
        signer: Vec<u8>,
    },
    // Not included in this version of ibc core
    UpgradeClient {},

    ClientMisbehaviour {},

    // Connection Messsages
    ConnectionOpenInit {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit,
        msg: Vec<u8>,
    },
    ConnectionOpenTry {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry
        msg: Vec<u8>,
    },
    ConnectionOpenAck {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenAck
        msg: Vec<u8>,
    },
    ConnectionOpenConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::connection::v1::MsgConnectionOpenConfirm
        msg: Vec<u8>,
    },

    // Channel Messages
    ChannelOpenInit {
        // raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenInit
        msg: Vec<u8>,
    },
    ChannelOpenTry {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenTry
        msg: Vec<u8>,
    },
    ChannelOpenAck {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenAck
        msg: Vec<u8>,
    },
    ChannelOpenConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelOpenConfirm
        msg: Vec<u8>,
    },
    ChannelCloseInit {
        port_id_on_a: String,
        chan_id_on_a: String,
        signer: Vec<u8>,
    },
    ChannelCloseConfirm {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgChannelCloseConfirm
        msg: Vec<u8>,
    },

    // Packet Messages
    SendPacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::Packet
        packet: Vec<u8>,
    },
    ReceivePacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgRecvPacket
        msg: Vec<u8>,
    },
    AcknowledgementPacket {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgAcknowledgement
        msg: Vec<u8>,
    },
    RequestTimeout {},
    Timeout {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgTimeout
        msg: Vec<u8>,
    },
    TimeoutOnClose {
        //raw message bytes:
        //ibc_proto::ibc::core::channel::v1::MsgTimeoutOnClose
        msg: Vec<u8>,
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
    #[returns(u64)]
    SequenceSend { port_id: String, channel_id: String },
}
