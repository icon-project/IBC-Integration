use ibc::{
    core::{
        ics03_connection::{msgs::conn_open_try::MsgConnectionOpenTry, version::Version},
        ics04_channel::{channel::Order, msgs::acknowledgement::Acknowledgement, packet::Packet},
    },
    signer::Signer,
};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry as RawMsgConnectionOpenTry;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, time::Duration};

use super::*;

#[cw_serde]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Client Messages
    RegisterClient {
        client_type: String,
        client_address: Addr,
    },
    CreateClient {
        client_state: Any,
        consensus_state: Any,
        signer: Signer,
    },
    UpdateClient {
        client_id: String,
        header: Any,
        signer: Signer,
    },
    // Not included in this version of ibc core
    UpgradeClient {},

    // Connection Messsages
    ConnectionOpenInit {
        client_id_on_a: String,
        counterparty: ibc::core::ics03_connection::connection::Counterparty,
        version: Option<Version>,
        delay_period: Duration,
        signer: Signer,
    },
    ConnectionOpenTry {
        msg: RawMsgConnectionOpenTry,
    },
    ConnectionOpenAck {
        /// ConnectionId that chain A has chosen for it's ConnectionEnd
        conn_id_on_a: ConnectionId,
        /// ConnectionId that chain B has chosen for it's ConnectionEnd
        conn_id_on_b: ConnectionId,
        /// ClientState of client tracking chain A on chain B
        client_state_of_a_on_b: Any,
        /// proof of ConnectionEnd stored on Chain B during ConnOpenTry
        proof_conn_end_on_b: Vec<u8>,
        /// proof of ClientState tracking chain A on chain B
        proof_client_state_of_a_on_b: Vec<u8>,
        /// proof that chain B has stored ConsensusState of chain A on its client
        proof_consensus_state_of_a_on_b: Vec<u8>,
        /// Height at which all proofs in this message were taken
        proofs_height_on_b: Height,
        /// height of latest header of chain A that updated the client on chain B
        consensus_height_of_a_on_b: Height,
        version: Version,
        signer: Signer,
    },
    ConnectionOpenConfirm {
        conn_id_on_b: ConnectionId,
        /// proof of ConnectionEnd stored on Chain A during ConnOpenInit
        proof_conn_end_on_a: Vec<u8>,
        /// Height at which `proof_conn_end_on_a` in this message was taken
        proof_height_on_a: Height,
        signer: Signer,
    },

    // Channel Messages
    ChannelOpenInit {
        port_id_on_a: String,
        connection_hops_on_a: Vec<IbcConnectionId>,
        port_id_on_b: String,
        ordering: Order,
        signer: Signer,
        /// Allow a relayer to specify a particular version by providing a non-empty version string
        version_proposal: ibc::core::ics04_channel::Version,
    },
    ChannelOpenTry {
        msg: ibc_proto::ibc::core::channel::v1::MsgChannelOpenTry,
    },
    ChannelOpenAck {
        port_id_on_a: String,
        chan_id_on_a: String,
        chan_id_on_b: String,
        version_on_b: ibc::core::ics04_channel::Version,
        proof_chan_end_on_b: Vec<u8>,
        proof_height_on_b: Height,
        signer: Signer,
    },
    ChannelOpenConfirm {
        port_id_on_b: String,
        chan_id_on_b: String,
        proof_chan_end_on_a: Vec<u8>,
        proof_height_on_a: Height,
        signer: Signer,
    },
    ChannelCloseInit {
        port_id_on_a: String,
        chan_id_on_a: String,
        signer: Signer,
    },
    ChannelCloseConfirm {
        port_id_on_b: String,
        chan_id_on_b: String,
        proof_chan_end_on_a: Vec<u8>,
        proof_height_on_a: Height,
        signer: Signer,
    },

    // Packet Messages
    SendPacket {
        packet: Packet,
    },
    ReceivePacket {
        /// The packet to be received
        packet: Packet,
        /// Proof of packet commitment on the sending chain
        proof_commitment_on_a: Vec<u8>,
        /// Height at which the commitment proof in this message were taken
        proof_height_on_a: Height,
        /// The signer of the message
        signer: Signer,
    },
    AcknowledgementPacket {
        packet: Packet,
        acknowledgement: Acknowledgement,
        /// Proof of packet acknowledgement on the receiving chain
        proof_acked_on_b: Vec<u8>,
        /// Height at which the commitment proof in this message were taken
        proof_height_on_b: Height,
        signer: Signer,
    },
    RequestTimeout {},
    Timeout {
        packet: Packet,
        next_seq_recv_on_b: u64,
        proof_unreceived_on_b: Vec<u8>,
        proof_height_on_b: Height,
        signer: Signer,
    },
    TimeoutOnClose {
        packet: Packet,
        next_seq_recv_on_b: u64,
        proof_unreceived_on_b: Vec<u8>,
        proof_close_on_b: Vec<u8>,
        proof_height_on_b: Height,
        signer: Signer,
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
pub enum QueryMsg {}
