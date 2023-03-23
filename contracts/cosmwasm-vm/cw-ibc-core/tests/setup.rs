use cosmwasm_std::{
    coins,
    testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        MOCK_CONTRACT_ADDR,
    },
    Addr, BlockInfo, ContractInfo, Empty, Env, MessageInfo, OwnedDeps, Timestamp, TransactionInfo,
};

use cw_ibc_core::types::{ClientId, ClientType};
use ibc::{
    core::{
        ics03_connection::version::{get_compatible_versions, Version},
        ics24_host::identifier::{ChannelId, ConnectionId, PortId},
    },
    mock::{
        client_state::MockClientState, consensus_state::MockConsensusState, header::MockHeader,
        misbehaviour::Misbehaviour,
    },
    signer::Signer,
    Height,
};
use ibc_proto::ibc::core::channel::v1::Channel as RawChannel;
pub use ibc_proto::ibc::core::channel::v1::Packet as RawPacket;
use ibc_proto::ibc::core::channel::v1::{
    MsgChannelCloseConfirm as RawMsgChannelCloseConfirm,
    MsgChannelCloseInit as RawMsgChannelCloseInit, MsgChannelOpenAck as RawMsgChannelOpenAck,
    MsgChannelOpenConfirm as RawMsgChannelOpenConfirm, MsgChannelOpenInit as RawMsgChannelOpenInit,
    MsgChannelOpenTry as RawMsgChannelOpenTry,
};
use ibc_proto::ibc::core::client::v1::Height as RawHeight;
use ibc_proto::ibc::core::client::v1::{
    MsgCreateClient as RawMessageCreateClient, MsgSubmitMisbehaviour as RawMessageMisbehaviour,
    MsgUpdateClient as RawMessageUpdateCelint, MsgUpgradeClient as RawMessageUpgradeClient,
};
use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
use ibc_proto::ibc::core::connection::v1::Counterparty as RawCounterpartyConnection;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit as RawMsgConnectionOpenInit;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry as RawMsgConnectionOpenTry;
use ibc_proto::ibc::core::{
    channel::v1::Counterparty as RawCounterparty, commitment::v1::MerklePrefix,
};
use ibc_proto::ics23::CommitmentProof;

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

pub fn get_dummy_raw_counterparty_for_channel(channel_id: String) -> RawCounterparty {
    RawCounterparty {
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
        prefix: Some(MerklePrefix {
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
    "cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng".to_string()
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

pub fn get_dummy_raw_msg_update_client_message() -> RawMessageUpdateCelint {
    let height = Height::new(10, 15).unwrap();
    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 1).unwrap();
    RawMessageUpdateCelint {
        client_id: client_id.ibc_client_id().to_string(),
        header: Some(MockHeader::new(height).into()),
        signer: "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".to_string(),
    }
}

pub fn get_dummy_raw_msg_upgrade_client(height: Height) -> RawMessageUpgradeClient {
    RawMessageUpgradeClient {
        client_id: "new_client_type".parse().unwrap(),
        client_state: Some(MockClientState::new(MockHeader::new(height)).into()),
        consensus_state: Some(MockConsensusState::new(MockHeader::new(height)).into()),
        proof_upgrade_client: get_dummy_proof(),
        proof_upgrade_consensus_state: get_dummy_proof(),
        signer: get_dummy_bech32_account(),
    }
}

/// Returns a dummy `RawMerkleProof`, for testing only!
pub fn get_dummy_merkle_proof() -> RawMerkleProof {
    let parsed = CommitmentProof { proof: None };
    let mproofs: Vec<CommitmentProof> = vec![parsed];
    RawMerkleProof { proofs: mproofs }
}

pub fn get_dummy_raw_msg_client_mishbehaviour() -> RawMessageMisbehaviour {
    let height = Height::new(10, 15).unwrap();
    let mock_header = MockHeader::new(height);

    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 1).unwrap();

    let mis_b = Misbehaviour {
        client_id: client_id.ibc_client_id().clone(),
        header1: mock_header,
        header2: mock_header,
    };
    RawMessageMisbehaviour {
        client_id: client_id.ibc_client_id().to_string(),
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
        proof_height: Some(ibc_proto::ibc::core::client::v1::Height {
            revision_number: 0,
            revision_height: proof_height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_msg_create_client() -> RawMessageCreateClient {
    let height = Height::new(10, 15).unwrap();
    let mock_header = MockHeader::new(height);
    let mock_client_state = MockClientState::new(mock_header);
    let mock_consenus_state = MockConsensusState::new(mock_header);
    RawMessageCreateClient {
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
    let client_state_height = Height::new(0, consensus_height).unwrap();

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

pub fn get_dummy_raw_packet(timeout_height: u64, timeout_timestamp: u64) -> RawPacket {
    RawPacket {
        sequence: 1,
        source_port: PortId::default().to_string(),
        source_channel: ChannelId::default().to_string(),
        destination_port: PortId::default().to_string(),
        destination_channel: ChannelId::default().to_string(),
        data: vec![0],
        timeout_height: Some(RawHeight {
            revision_number: 0,
            revision_height: timeout_height,
        }),
        timeout_timestamp,
    }
}
