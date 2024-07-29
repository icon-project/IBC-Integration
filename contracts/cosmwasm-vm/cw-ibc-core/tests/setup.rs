use std::{collections::HashMap, str::FromStr};

use cw_ibc_core::conversions::{
    to_ibc_channel, to_ibc_channel_id, to_ibc_client_id, to_ibc_connection_id, to_ibc_height,
    to_ibc_port_id, to_ibc_timeout_height, to_ibc_timestamp,
};
use cw_ibc_core::ics03_connection::State as ConnectionState;
use cw_ibc_core::light_client::light_client::LightClient;

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

use common::ibc::core::ics04_channel::channel::Order;
use common::ibc::core::ics04_channel::Version;

use cosmwasm_std::Storage;
use cosmwasm_std::{
    coins,
    testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        MOCK_CONTRACT_ADDR,
    },
    to_binary, Addr, Binary, BlockInfo, ContractInfo, ContractResult, DepsMut, Empty, Env,
    IbcEndpoint, MessageInfo, OwnedDeps, SystemResult, Timestamp, TransactionInfo, WasmQuery,
};

use common::{
    client_state::get_default_icon_client_state,
    ibc::{
        core::{
            ics03_connection::version::{get_compatible_versions, Version as ConnectionVersion},
            ics24_host::identifier::{ChannelId, ConnectionId, PortId},
        },
        signer::Signer,
        Height,
    },
    icon::icon::lightclient::v1::{ClientState, ConsensusState},
    traits::AnyTypes,
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

impl Default for MockEnvBuilder {
    fn default() -> Self {
        Self::new()
    }
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
        version: Some(ConnectionVersion::default().into()),
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
        version: Some(ConnectionVersion::default().into()),
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

pub fn mock_lightclient_query(
    mocks: HashMap<Binary, Binary>,
    deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
) {
    deps.querier.update_wasm(move |r| match r {
        WasmQuery::Smart {
            contract_addr: _,
            msg,
        } => {
            if mocks.get(msg).is_some() {
                let res = mocks.get(msg).unwrap().clone();
                SystemResult::Ok(ContractResult::Ok(res))
            } else {
                SystemResult::Ok(ContractResult::Ok(to_binary(&true).unwrap()))
            }
        }
        _ => todo!(),
    });
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

pub fn mock_consensus_state_query(
    mut query_map: HashMap<Binary, Binary>,
    client_id: &IbcClientId,
    state: &ConsensusState,
    height: u64,
) -> HashMap<Binary, Binary> {
    let query = LightClient::build_consensus_state_query(client_id, height).unwrap();
    let reply_any = state.to_any();
    let reply = to_binary(&reply_any.encode_to_vec()).unwrap();
    query_map.insert(query, reply);
    query_map
}

pub fn mock_client_state_query(
    mut query_map: HashMap<Binary, Binary>,
    client_id: &IbcClientId,
    state: &ClientState,
) -> HashMap<Binary, Binary> {
    let query = LightClient::build_client_state_query(client_id).unwrap();
    let reply_any = state.to_any();
    let reply = to_binary(&reply_any.encode_to_vec()).unwrap();
    query_map.insert(query, reply);
    query_map
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
use cw_common::ibc_types::IbcClientId;
use cw_ibc_core::context::CwIbcCoreContext;
use cw_ibc_core::ics04_channel::Counterparty;
use cw_ibc_core::ics04_channel::State;
use cw_ibc_core::traits::IbcClient;
use cw_ibc_core::ConnectionEnd;
use cw_ibc_core::{compute_packet_commitment, ChannelEnd, Sequence};
use prost::Message;

use std::time::Duration;
pub fn get_dummy_connection() -> ConnectionEnd {
    let counter_prefix: Result<
        common::ibc::core::ics23_commitment::commitment::CommitmentPrefix,
        common::ibc::core::ics23_commitment::error::CommitmentError,
    > = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        ClientId::default(),
        Some(ConnectionId::default()),
        counter_prefix.unwrap(),
    );

    ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    )
}

pub fn get_dummy_channel_end(port_id: &PortId) -> ChannelEnd {
    ChannelEnd::new(
        State::Open,
        Order::default(),
        Counterparty::new(port_id.clone(), None),
        vec![ConnectionId::default()],
        Version::new("ics20-1".to_string()),
    )
}

enum Direction {
    Send,
    Receive,
}

pub struct TestContext {
    pub client_state: Option<ClientState>,
    pub consensus_state: Option<ConsensusState>,
    pub connection_end: Option<ConnectionEnd>,
    pub channel_end: Option<ChannelEnd>,
    pub lightclient: Option<LightClient>,
    pub module_address: Option<Addr>,
    pub packet: Option<RawPacket>,
    pub client_id: IbcClientId,
    pub client_type: ClientType,
    pub connection_id: ConnectionId,
    pub env: Env,
    pub height: Height,
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub mock_queries: HashMap<Binary, Binary>,
}

impl TestContext {
    pub fn default(env: Env) -> Self {
        Self {
            client_state: Some(get_dummy_client_state()),
            consensus_state: Some(get_dummy_consensus_state()),
            connection_end: Some(get_dummy_connection()),
            channel_end: Some(get_dummy_channel_end(&PortId::default())),
            client_id: ClientId::default(),
            env,
            connection_id: ConnectionId::default(),
            height: Height::new(0, 10).unwrap(),
            port_id: PortId::default(),
            channel_id: ChannelId::default(),
            lightclient: Some(LightClient::new("lightclient".to_string())),
            client_type: ClientType::new("iconclient".to_string()),
            packet: None,
            mock_queries: HashMap::<Binary, Binary>::new(),
            module_address: Some(Addr::unchecked("moduleaddress")),
        }
    }

    pub fn for_receive_packet(env: Env, msg: &RawMsgRecvPacket) -> Self {
        let mut ctx = TestContext::default(env);
        let packet = msg.packet.clone().unwrap();

        ctx = TestContext::setup_channel_end(ctx, State::Open, Direction::Receive, &packet);
        ctx.height = to_ibc_height(msg.proof_height.clone()).unwrap();

        ctx
    }

    pub fn for_connection_open_init(env: Env, msg: &RawMsgConnectionOpenInit) -> Self {
        let mut ctx = TestContext::default(env);
        ctx.client_id = to_ibc_client_id(&msg.client_id).unwrap();
        ctx.connection_end = None;
        ctx
    }

    pub fn for_connection_open_ack(env: Env, msg: &RawMsgConnectionOpenAck) -> Self {
        let mut ctx = TestContext::default(env);
        let mut connection = ctx.connection_end();
        connection.state = ConnectionState::Init;
        ctx.connection_end = Some(connection);
        ctx.height = to_ibc_height(msg.proof_height.clone()).unwrap();
        ctx.connection_id = to_ibc_connection_id(&msg.connection_id).unwrap();

        ctx
    }

    pub fn for_connection_open_confirm(env: Env, msg: &RawMsgConnectionOpenConfirm) -> Self {
        let mut ctx = TestContext::default(env);
        let mut connection = ctx.connection_end();
        connection.state = ConnectionState::TryOpen;
        ctx.connection_end = Some(connection);
        ctx.height = to_ibc_height(msg.proof_height.clone()).unwrap();
        ctx.connection_id = to_ibc_connection_id(&msg.connection_id).unwrap();

        ctx
    }

    pub fn for_connection_open_try(env: Env, msg: &RawMsgConnectionOpenTry) -> Self {
        let mut ctx = TestContext::default(env);
        let connection = ctx.connection_end();
        ctx.connection_end = Some(connection);
        ctx.height = to_ibc_height(msg.proof_height.clone()).unwrap();

        ctx
    }

    pub fn for_acknowledge_packet(env: Env, msg: &RawMsgAcknowledgement) -> Self {
        let mut ctx = TestContext::default(env);
        let packet = msg.packet.clone().unwrap();

        ctx = TestContext::setup_channel_end(ctx, State::Open, Direction::Send, &packet);
        ctx.height = to_ibc_height(msg.proof_height.clone()).unwrap();
        ctx.packet = Some(packet);

        ctx
    }

    pub fn for_send_packet(env: Env, msg: &RawPacket) -> Self {
        let mut ctx = TestContext::default(env);
        let packet = msg.clone();

        ctx = TestContext::setup_channel_end(ctx, State::Open, Direction::Send, &packet);
        ctx.packet = Some(packet);

        ctx
    }

    pub fn for_channel_close_init(env: Env, msg: &RawMsgChannelCloseInit) -> Self {
        let mut ctx = TestContext::default(env);
        let packet = RawPacket {
            source_port: msg.port_id.clone(),
            source_channel: msg.channel_id.clone(),
            destination_port: "dst_port".to_string(),
            destination_channel: "dst_chan".to_string(),
            ..Default::default()
        };

        ctx = TestContext::setup_channel_end(ctx, State::Open, Direction::Send, &packet);
        ctx.packet = Some(packet);

        ctx
    }

    pub fn for_packet_timeout(env: Env, msg: &RawMsgTimeout) -> Self {
        let mut ctx = TestContext::default(env);
        let packet = msg.packet.clone().unwrap();

        ctx = TestContext::setup_channel_end(ctx, State::Open, Direction::Send, &packet);
        ctx.height = to_ibc_height(msg.proof_height.clone()).unwrap();
        ctx.packet = Some(packet);

        ctx
    }

    pub fn for_packet_timeout_on_close(env: Env, msg: &RawMsgTimeoutOnClose) -> Self {
        let mut ctx = TestContext::default(env);
        let packet = msg.packet.clone().unwrap();

        ctx = TestContext::setup_channel_end(ctx, State::Open, Direction::Send, &packet);
        ctx.height = to_ibc_height(msg.proof_height.clone()).unwrap();
        ctx.packet = Some(packet);

        ctx
    }

    pub fn for_channel_close_confirm(env: Env, msg: &RawMsgChannelCloseConfirm) -> Self {
        let mut ctx = TestContext::default(env);
        let packet = RawPacket {
            source_port: "their-port".to_string(),
            source_channel: "their-channel".to_string(),
            destination_port: msg.port_id.clone(),
            destination_channel: msg.channel_id.clone(),
            ..Default::default()
        };

        ctx = TestContext::setup_channel_end(ctx, State::Open, Direction::Receive, &packet);
        ctx.packet = Some(packet);

        ctx
    }

    pub fn for_channel_open_confirm(env: Env, msg: &RawMsgChannelOpenConfirm) -> Self {
        let mut ctx = TestContext::default(env);
        let packet = RawPacket {
            source_port: "their-port".to_string(),
            source_channel: "their-channel".to_string(),
            destination_port: msg.port_id.clone(),
            destination_channel: msg.channel_id.clone(),
            ..Default::default()
        };

        ctx = TestContext::setup_channel_end(ctx, State::TryOpen, Direction::Receive, &packet);

        ctx.packet = Some(packet);

        ctx
    }

    pub fn for_channel_open_try(env: Env, msg: &RawMsgChannelOpenTry) -> Self {
        let mut ctx = TestContext::default(env);
        ctx.port_id = to_ibc_port_id(&msg.port_id).unwrap();

        ctx
    }

    pub fn for_channel_open_ack(env: Env, msg: &RawMsgChannelOpenAck) -> Self {
        let mut ctx = TestContext::default(env);
        let packet = RawPacket {
            source_port: msg.port_id.clone(),
            source_channel: msg.channel_id.clone(),
            destination_port: msg.port_id.clone(),
            destination_channel: msg.counterparty_channel_id.clone(),
            ..Default::default()
        };
        ctx = TestContext::setup_channel_end(ctx, State::Init, Direction::Send, &packet);

        ctx.packet = Some(packet);
        ctx.height = to_ibc_height(msg.proof_height.clone()).unwrap();
        ctx
    }

    pub fn for_channel_open_init(env: Env, msg: &RawMsgChannelOpenInit) -> Self {
        let mut ctx = TestContext::default(env);
        ctx.port_id = to_ibc_port_id(&msg.port_id).unwrap();
        ctx.channel_id = ChannelId::new(0);
        ctx.channel_end = Some(to_ibc_channel(msg.channel.clone()).unwrap());

        ctx
    }

    pub fn for_client_state(env: Env) -> Self {
        let mut ctx = TestContext::default(env);
        ctx.client_id = ClientId::from_str("iconclient-0").unwrap();

        ctx
    }

    fn setup_channel_end(
        mut ctx: TestContext,
        state: State,
        dir: Direction,
        packet: &RawPacket,
    ) -> TestContext {
        let src_port = to_ibc_port_id(&packet.source_port).unwrap();
        let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

        let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
        let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

        match dir {
            Direction::Send => {
                let mut chan_end_on_b = get_dummy_channel_end(&dst_port);
                chan_end_on_b.set_counterparty_channel_id(dst_channel);
                chan_end_on_b.set_state(state);

                ctx.channel_end = Some(chan_end_on_b.clone());
                ctx.port_id = src_port;
                ctx.channel_id = src_channel;
            }
            Direction::Receive => {
                let mut chan_end_on_b = get_dummy_channel_end(&src_port);
                chan_end_on_b.set_counterparty_channel_id(src_channel);
                chan_end_on_b.set_state(state);
                ctx.channel_end = Some(chan_end_on_b.clone());
                ctx.port_id = dst_port;
                ctx.channel_id = dst_channel;
            }
        }

        ctx
    }

    pub fn init_context(&mut self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        self.save_client_state(storage, contract);
        self.save_consensus_state(storage, self.height.revision_height());
        self.save_connection(storage, contract);
        self.save_channel_end(storage, contract);
        self.save_light_client(storage, contract);
        self.save_expected_time_per_block(storage, contract);
    }

    pub fn init_receive_packet(&mut self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
    }

    pub fn init_connection_open_ack(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);

        contract
            .connection_next_sequence_init(storage, u128::default().try_into().unwrap())
            .unwrap();
    }

    pub fn init_connection_open_init(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);

        contract
            .connection_next_sequence_init(storage, u128::default().try_into().unwrap())
            .unwrap();
    }

    pub fn init_connection_open_confirm(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);

        contract
            .connection_next_sequence_init(storage, u128::default().try_into().unwrap())
            .unwrap();
    }

    pub fn init_connection_open_try(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
        init_sequence: bool,
    ) {
        self.init_context(storage, contract);
        if init_sequence {
            contract
                .connection_next_sequence_init(storage, u128::default().try_into().unwrap())
                .unwrap();
        }
    }

    pub fn init_channel_close_init(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
    }

    pub fn init_channel_open_ack(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
        contract
            .init_channel_counter(storage, u64::default())
            .unwrap();
    }

    pub fn init_channel_open_init(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
    }

    pub fn init_channel_close_confirm(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
    }

    pub fn init_channel_open_confirm(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
        contract.init_channel_counter(storage, u64::default());
    }

    pub fn init_channel_open_try(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
        contract
            .init_channel_counter(storage, u64::default())
            .unwrap();
    }

    pub fn init_acknowledge_packet(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
        self.save_packet_commitment(storage, contract);
    }

    pub fn init_send_packet(&mut self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
        self.save_next_sequence_send(storage, contract);
        self.save_packet_commitment(storage, contract);
        self.save_consensus_state(storage, self.client_state.clone().unwrap().latest_height);
    }

    pub fn init_timeout_packet(&mut self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
        self.save_next_sequence_send(storage, contract);
        self.save_packet_commitment(storage, contract);
        self.save_consensus_state(storage, self.client_state.clone().unwrap().latest_height);
    }

    pub fn init_timeout_packet_on_close(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        self.init_context(storage, contract);
        self.register_port(storage, contract);
        self.save_next_sequence_send(storage, contract);
        self.save_packet_commitment(storage, contract);
        self.save_consensus_state(storage, self.client_state.clone().unwrap().latest_height);
    }

    pub fn init_connection_delay(
        &mut self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        contract
            .store_last_processed_on(storage, &self.env, &self.client_id)
            .unwrap();
    }

    pub fn init_client_state(&mut self, deps: DepsMut, contract: &CwIbcCoreContext) {
        self.init_context(deps.storage, contract);
        self.save_consensus_state(
            deps.storage,
            self.client_state.clone().unwrap().latest_height,
        );
        contract.init_client_counter(deps.storage, 0).unwrap();
        self.save_client_commitment(
            deps.storage,
            contract,
            &self.client_state.clone().unwrap().encode_to_vec(),
        );
        self.save_register_client(deps, contract);
    }

    pub fn save_next_sequence_send(&self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        if let Some(packet) = self.packet.clone() {
            contract
                .store_next_sequence_send(
                    storage,
                    &self.port_id,
                    &self.channel_id,
                    &Sequence::from(packet.sequence),
                )
                .unwrap();
        }
    }

    pub fn save_client_state(&mut self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        if let Some(client_state) = self.client_state.clone() {
            let client_state_any = client_state.to_any();
            let query = LightClient::build_client_state_query(&self.client_id).unwrap();
            self.mock_queries
                .insert(query, to_binary(&client_state_any.encode_to_vec()).unwrap());
            contract
                .store_last_processed_on(storage, &self.env, &self.client_id)
                .unwrap();
            self.save_timestamp_at_height(client_state.latest_height, 0_u64);
        }
    }

    pub fn save_timestamp_at_height(&mut self, height: u64, timestamp: u64) {
        let timestamp_query =
            LightClient::get_timestamp_at_height_query(&self.client_id, height).unwrap();
        self.mock_queries
            .insert(timestamp_query, to_binary(&timestamp).unwrap());
    }

    pub fn save_consensus_state(&mut self, _storage: &mut dyn Storage, height: u64) {
        if let Some(consensus_state) = self.consensus_state.clone() {
            let consensus_state_any = consensus_state.to_any();
            let query = LightClient::build_consensus_state_query(&self.client_id, height).unwrap();
            self.mock_queries.insert(
                query,
                to_binary(&consensus_state_any.encode_to_vec()).unwrap(),
            );
        }
    }

    pub fn save_connection(&self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        if let Some(connection_end) = self.connection_end.clone() {
            contract
                .store_connection(storage, &self.connection_id, &connection_end)
                .unwrap();
        }
    }

    pub fn save_connection_to_client(
        &self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        contract
            .store_connection_to_client(storage, &self.client_id, &self.connection_id)
            .unwrap();
    }

    pub fn save_client_commitment(
        &self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
        client_state_hash: &Vec<u8>,
    ) {
        contract
            .store_client_commitment(
                storage,
                &self.env,
                &self.client_id,
                client_state_hash.clone(),
            )
            .unwrap();
    }

    pub fn save_register_client(&self, deps: DepsMut, contract: &CwIbcCoreContext) {
        if let Some(lightclient) = self.lightclient.clone() {
            let addr = Addr::unchecked(lightclient.get_address());

            contract
                .register_client(deps, self.client_type.clone(), addr)
                .unwrap();
        }
    }

    pub fn save_channel_end(&self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        if let Some(channel_end) = self.channel_end.clone() {
            contract
                .store_channel_end(storage, &self.port_id, &self.channel_id, &channel_end)
                .unwrap();
        }
    }

    pub fn save_light_client(&self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        if let Some(lightclient) = self.lightclient.clone() {
            contract
                .store_client_implementations(storage, &self.client_id, lightclient)
                .unwrap();
        }
    }

    pub fn save_packet_commitment(&self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        if let Some(packet) = self.packet.clone() {
            let packet_timeout_height =
                to_ibc_timeout_height(packet.timeout_height.clone()).unwrap();
            let packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp).unwrap();
            let packet_sequence = Sequence::from(packet.sequence);
            let packet_commitment =
                compute_packet_commitment(&packet.data, &packet_timeout_height, &packet_timestamp);
            contract
                .store_packet_commitment(
                    storage,
                    &self.port_id,
                    &self.channel_id,
                    packet_sequence,
                    packet_commitment,
                )
                .unwrap();
        }
    }

    pub fn save_expected_time_per_block(
        &self,
        storage: &mut dyn Storage,
        contract: &CwIbcCoreContext,
    ) {
        contract
            .ibc_store()
            .expected_time_per_block()
            .save(storage, &(self.env.block.time.seconds()))
            .unwrap();
    }

    pub fn register_port(&self, storage: &mut dyn Storage, contract: &CwIbcCoreContext) {
        if let Some(module) = self.module_address.clone() {
            contract
                .bind_port(storage, &self.port_id, module.to_string())
                .unwrap();
        }
    }

    pub fn channel_end(&self) -> ChannelEnd {
        self.channel_end.clone().unwrap()
    }

    pub fn connection_end(&self) -> ConnectionEnd {
        self.connection_end.clone().unwrap()
    }
}
