use std::str::FromStr;

pub fn mock_height(
    number: u64,
    height: u64,
) -> Result<common::ibc::Height, common::ibc::core::ics02_client::error::ClientError> {
    common::ibc::Height::new(number, height)
}

pub fn to_mock_height(height: Height) -> common::ibc::Height {
    common::ibc::Height::new(height.revision_number(), height.revision_height()).unwrap()
}

pub fn to_mock_client_id(
    client_id: &ClientId,
) -> common::ibc::core::ics24_host::identifier::ClientId {
    common::ibc::core::ics24_host::identifier::ClientId::from_str(&client_id.to_string()).unwrap()
}

use cosmwasm_std::{
    coins,
    testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        MOCK_CONTRACT_ADDR,
    },
    to_binary, Addr, BlockInfo, ContractInfo, ContractResult, Empty, Env, IbcEndpoint, MessageInfo,
    OwnedDeps, SystemResult, Timestamp, TransactionInfo, WasmQuery,
};

use common::{
    client_state::get_default_icon_client_state,
    ibc::{
        core::{
            ics03_connection::version::{get_compatible_versions, Version},
            ics24_host::identifier::{ChannelId, ConnectionId, PortId},
        },
        signer::Signer,
        Height,
    },
    icon::icon::lightclient::v1::{ClientState, ConsensusState},
};
use cw_common::raw_types::channel::*;
use cw_common::raw_types::connection::*;
use cw_common::raw_types::{
    client::{
        RawMsgCreateClient, RawMsgSubmitMisbehaviour, RawMsgUpdateClient, RawMsgUpgradeClient,
    },
    RawCommitmentProof, RawHeight, RawMerkleProof,
};

use common::ibc::core::ics02_client::client_type::ClientType;
use common::ibc::core::ics24_host::identifier::ClientId;

pub struct MockEnvBuilder {
    env: Env,
}

impl MockEnvBuilder {
    pub fn new() -> MockEnvBuilder {
        MockEnvBuilder {
            env: Env {
                block: BlockInfo {
                    height: 0,
                    time: Timestamp::from_nanos(0),
                    chain_id: "".to_string(),
                },
                transaction: None,
                contract: ContractInfo {
                    address: Addr::unchecked("input"),
                },
            },
        }
    }
    pub fn add_block(mut self, block: BlockInfo) -> MockEnvBuilder {
        self.env.block = block;
        self
    }

    pub fn add_txn_info(mut self, txn_info: Option<TransactionInfo>) -> MockEnvBuilder {
        self.env.transaction = txn_info;
        self
    }

    pub fn add_contract_info(mut self, contract_info: ContractInfo) -> MockEnvBuilder {
        self.env.contract = contract_info;
        self
    }

    pub fn build(self) -> Env {
        Env {
            block: self.env.block,
            transaction: self.env.transaction,
            contract: self.env.contract,
        }
    }
}

pub fn create_mock_info(creator: &str, denom: &str, amount: u128) -> MessageInfo {
    let funds = coins(amount, denom);
    mock_info(creator, &funds)
}

pub fn deps() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    mock_dependencies()
}

pub use common::ibc::mock::client_state::MockClientState;
pub use common::ibc::mock::consensus_state::MockConsensusState;
use common::ibc::mock::header::MockHeader;
pub use common::ibc::mock::misbehaviour::Misbehaviour;

#[test]
fn test() {
    let mock = mock_env();

    let block_info = BlockInfo {
        height: 12_345,
        time: Timestamp::from_nanos(1_571_797_419_879_305_533),
        chain_id: "cosmos-testnet-14002".to_string(),
    };

    let transaction = None;
    let contract = ContractInfo {
        address: Addr::unchecked(MOCK_CONTRACT_ADDR),
    };

    let mock_env_builder: Env = MockEnvBuilder::new()
        .add_block(block_info)
        .add_txn_info(transaction)
        .add_contract_info(contract)
        .build();

    assert_ne!(mock, mock_env_builder)
}

pub fn get_dummy_raw_counterparty_for_channel(
    channel_id: String,
) -> cw_common::raw_types::channel::RawCounterparty {
    cw_common::raw_types::channel::RawCounterparty {
        port_id: PortId::default().to_string(),
        channel_id,
    }
}
pub fn get_dummy_raw_counterparty(conn_id: Option<u64>) -> RawCounterpartyConnection {
    let connection_id = match conn_id {
        Some(id) => ConnectionId::new(id).to_string(),
        None => "".to_string(),
    };
    RawCounterpartyConnection {
        client_id: ClientId::default().as_str().to_string(),
        connection_id,
        prefix: Some(RawMerklePrefix {
            key_prefix: b"ibc".to_vec(),
        }),
    }
}

// Returns a dummy `RawChannel`, for testing only!
pub fn get_dummy_raw_channel_end(channel_id: Option<u64>) -> RawChannel {
    let channel_id = match channel_id {
        Some(id) => ChannelId::new(id).to_string(),
        None => "".to_string(),
    };
    RawChannel {
        state: 1,
        ordering: 2,
        counterparty: Some(get_dummy_raw_counterparty_for_channel(channel_id)),
        connection_hops: vec![ConnectionId::default().to_string()],
        version: "".to_string(), // The version is not validated.
    }
}

// Returns a dummy `RawMsgChannelOpenInit`, for testing only!
pub fn get_dummy_raw_msg_chan_open_init(
    counterparty_channel_id: Option<u64>,
) -> RawMsgChannelOpenInit {
    RawMsgChannelOpenInit {
        port_id: PortId::default().to_string(),
        channel: Some(get_dummy_raw_channel_end(counterparty_channel_id)),
        signer: get_dummy_bech32_account(),
    }
}
pub fn get_dummy_bech32_account() -> String {
    "archway19d4lkjwk2wnf4fzraw4gwspvevlqa9kwu2nasl".to_string()
}

pub fn get_dummy_account_id() -> Signer {
    "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".parse().unwrap()
}

// Returns a dummy `RawMsgChannelOpenInit`, for testing only!

pub fn get_dummy_raw_msg_chan_open_ack(proof_height: u64) -> RawMsgChannelOpenAck {
    RawMsgChannelOpenAck {
        port_id: PortId::default().to_string(),
        channel_id: ChannelId::default().to_string(),
        counterparty_channel_id: ChannelId::default().to_string(),
        counterparty_version: "".to_string(),
        proof_try: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: proof_height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_proof() -> Vec<u8> {
    "Y29uc2Vuc3VzU3RhdGUvaWJjb25lY2xpZW50LzIy"
        .as_bytes()
        .to_vec()
}

// Returns a dummy `RawMsgChannelOpenConfirm`, for testing only!
pub fn get_dummy_raw_msg_chan_open_confirm(proof_height: u64) -> RawMsgChannelOpenConfirm {
    RawMsgChannelOpenConfirm {
        port_id: PortId::default().to_string(),
        channel_id: ChannelId::default().to_string(),
        proof_ack: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: proof_height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

// Returns a dummy `RawMsgChannelOpenTry`, for testing only!
pub fn get_dummy_raw_msg_chan_open_try(proof_height: u64) -> RawMsgChannelOpenTry {
    #[allow(deprecated)]
    RawMsgChannelOpenTry {
        port_id: PortId::default().to_string(),
        previous_channel_id: ChannelId::default().to_string(),
        channel: Some(get_dummy_raw_channel_end(Some(0))),
        counterparty_version: "".to_string(),
        proof_init: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: proof_height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_msg_update_client_message() -> RawMsgUpdateClient {
    let height = mock_height(10, 15).unwrap();
    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type, 1).unwrap();
    RawMsgUpdateClient {
        client_id: client_id.to_string(),
        header: Some(MockHeader::new(height).into()),
        signer: "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".to_string(),
    }
}

pub fn get_dummy_raw_msg_upgrade_client(height: Height) -> RawMsgUpgradeClient {
    RawMsgUpgradeClient {
        client_id: "new_client_type".parse().unwrap(),
        client_state: Some(MockClientState::new(MockHeader::new(to_mock_height(height))).into()),
        consensus_state: Some(
            MockConsensusState::new(MockHeader::new(to_mock_height(height))).into(),
        ),
        proof_upgrade_client: get_dummy_proof(),
        proof_upgrade_consensus_state: get_dummy_proof(),
        signer: get_dummy_bech32_account(),
    }
}

/// Returns a dummy `RawMerkleProof`, for testing only!
pub fn get_dummy_merkle_proof() -> RawMerkleProof {
    let parsed = RawCommitmentProof { proof: None };
    let mproofs: Vec<RawCommitmentProof> = vec![parsed];
    RawMerkleProof { proofs: mproofs }
}

pub fn get_dummy_raw_msg_client_mishbehaviour() -> RawMsgSubmitMisbehaviour {
    let height = mock_height(10, 15).unwrap();
    let mock_header = MockHeader::new(height);

    let client_type = ClientType::new("new_client_type".to_string());
    let client_id = ClientId::new(client_type, 1).unwrap();

    let mis_b = Misbehaviour {
        client_id: to_mock_client_id(&client_id),
        header1: mock_header,
        header2: mock_header,
    };
    RawMsgSubmitMisbehaviour {
        client_id: client_id.to_string(),
        misbehaviour: Some(mis_b.into()),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_msg_chan_close_init() -> RawMsgChannelCloseInit {
    RawMsgChannelCloseInit {
        port_id: PortId::default().to_string(),
        channel_id: ChannelId::default().to_string(),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_msg_chan_close_confirm(proof_height: u64) -> RawMsgChannelCloseConfirm {
    RawMsgChannelCloseConfirm {
        port_id: PortId::default().to_string(),
        channel_id: ChannelId::default().to_string(),
        proof_init: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: proof_height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_msg_create_client() -> RawMsgCreateClient {
    let height = mock_height(10, 15).unwrap();
    let mock_header = MockHeader::new(height);
    let mock_client_state = MockClientState::new(mock_header);
    let mock_consenus_state = MockConsensusState::new(mock_header);
    RawMsgCreateClient {
        client_state: Some(mock_client_state.into()),
        consensus_state: Some(mock_consenus_state.into()),
        signer: get_dummy_account_id().as_ref().to_string(),
    }
}

pub fn get_dummy_raw_msg_conn_open_init() -> RawMsgConnectionOpenInit {
    RawMsgConnectionOpenInit {
        client_id: ClientId::default().as_str().to_string(),
        counterparty: Some(get_dummy_raw_counterparty(None)),
        version: Some(Version::default().into()),
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_msg_conn_open_try(
    proof_height: u64,
    consensus_height: u64,
) -> RawMsgConnectionOpenTry {
    let client_state_height = mock_height(0, consensus_height).unwrap();

    #[allow(deprecated)]
    RawMsgConnectionOpenTry {
        client_id: ClientId::default().as_str().to_string(),
        previous_connection_id: ConnectionId::default().to_string(),
        client_state: Some(MockClientState::new(MockHeader::new(client_state_height)).into()),
        counterparty: Some(get_dummy_raw_counterparty(Some(0))),
        delay_period: 0,
        counterparty_versions: get_compatible_versions()
            .iter()
            .map(|v| v.clone().into())
            .collect(),
        proof_init: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: proof_height,
        }),
        proof_consensus: get_dummy_proof(),
        consensus_height: Some(RawHeight {
            revision_number: 0,
            revision_height: consensus_height,
        }),
        proof_client: get_dummy_proof(),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_msg_conn_open_ack(
    proof_height: u64,
    consensus_height: u64,
) -> RawMsgConnectionOpenAck {
    let client_state_height = mock_height(0, consensus_height).unwrap();
    RawMsgConnectionOpenAck {
        connection_id: ConnectionId::new(0).to_string(),
        counterparty_connection_id: ConnectionId::new(1).to_string(),
        proof_try: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: proof_height,
        }),
        proof_consensus: get_dummy_proof(),
        consensus_height: Some(RawHeight {
            revision_number: 0,
            revision_height: consensus_height,
        }),
        client_state: Some(MockClientState::new(MockHeader::new(client_state_height)).into()),
        proof_client: get_dummy_proof(),
        version: Some(Version::default().into()),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_msg_conn_open_confirm() -> RawMsgConnectionOpenConfirm {
    RawMsgConnectionOpenConfirm {
        connection_id: "srcconnection".to_string(),
        proof_ack: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: 10,
        }),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_packet(timeout_height: u64, timeout_timestamp: u64) -> RawPacket {
    let (src, dest) = get_dummy_endpoints();
    RawPacket {
        sequence: 1,
        source_port: src.port_id,
        source_channel: src.channel_id,
        destination_port: dest.port_id,
        destination_channel: dest.channel_id,
        data: vec![0],
        timeout_height: Some(RawHeight {
            revision_number: 0,
            revision_height: timeout_height,
        }),
        timeout_timestamp,
    }
}

pub fn get_dummy_raw_msg_acknowledgement(height: u64) -> RawMsgAcknowledgement {
    get_dummy_raw_msg_ack_with_packet(get_dummy_raw_packet(height, 1), height)
}

pub fn get_dummy_raw_msg_ack_with_packet(packet: RawPacket, height: u64) -> RawMsgAcknowledgement {
    RawMsgAcknowledgement {
        packet: Some(packet),
        acknowledgement: get_dummy_proof(),
        proof_acked: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_msg_timeout(
    proof_height: u64,
    timeout_height: u64,
    timeout_timestamp: u64,
) -> RawMsgTimeout {
    RawMsgTimeout {
        packet: Some(get_dummy_raw_packet(timeout_height, timeout_timestamp)),
        proof_unreceived: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: proof_height,
        }),
        next_sequence_recv: 1,
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_msg_timeout_on_close(
    height: u64,
    timeout_timestamp: u64,
) -> RawMsgTimeoutOnClose {
    RawMsgTimeoutOnClose {
        packet: Some(get_dummy_raw_packet(height, timeout_timestamp)),
        proof_unreceived: get_dummy_proof(),
        proof_close: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: height,
        }),
        next_sequence_recv: 1,
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_mock_env() -> Env {
    let mut env = mock_env();
    env.contract.address = Addr::unchecked("archway19d4lkjwk2wnf4fzraw4gwspvevlqa9kwu2nasl");
    env
}

pub fn mock_lightclient_reply(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>) {
    deps.querier.update_wasm(|r| match r {
        WasmQuery::Smart {
            contract_addr: _,
            msg: _,
        } => SystemResult::Ok(ContractResult::Ok(to_binary(&true).unwrap())),
        _ => todo!(),
    });
}

pub fn get_dummy_endpoints() -> (IbcEndpoint, IbcEndpoint) {
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };
    (src, dst)
}

pub fn get_dummy_client_state() -> ClientState {
    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        ..get_default_icon_client_state()
    };
    client_state
}

pub fn get_dummy_consensus_state() -> ConsensusState {
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
        next_proof_context_hash: vec![1, 2, 3, 4],
    };
    consenus_state
}
