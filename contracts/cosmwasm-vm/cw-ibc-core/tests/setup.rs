use cosmwasm_std::{
    coins,
    testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        MOCK_CONTRACT_ADDR,
    },
    Addr, BlockInfo, ContractInfo, Empty, Env, MessageInfo, OwnedDeps, Timestamp, TransactionInfo,
};
use ibc::core::ics03_connection::version::get_compatible_versions;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::core::ics24_host::identifier::ConnectionId;
use ibc::Height;
use ibc::{
    core::ics03_connection::version::Version,
    mock::{client_state::MockClientState, header::MockHeader},
};
use ibc_proto::ibc::core::client::v1::Height as RawHeight;
use ibc_proto::ibc::core::commitment::v1::MerklePrefix;
use ibc_proto::ibc::core::connection::v1::Counterparty as RawCounterparty;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenInit as RawMsgConnectionOpenInit;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry as RawMsgConnectionOpenTry;

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

pub fn get_dummy_raw_msg_conn_open_init() -> RawMsgConnectionOpenInit {
    RawMsgConnectionOpenInit {
        client_id: ClientId::default().to_string(),
        counterparty: Some(get_dummy_raw_counterparty(None)),
        version: Some(Version::default().into()),
        delay_period: 0,
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_raw_counterparty(conn_id: Option<u64>) -> RawCounterparty {
    let connection_id = match conn_id {
        Some(id) => ConnectionId::new(id).to_string(),
        None => "".to_string(),
    };
    RawCounterparty {
        client_id: ClientId::default().to_string(),
        connection_id,
        prefix: Some(MerklePrefix {
            key_prefix: b"ibc".to_vec(),
        }),
    }
}

pub fn get_dummy_bech32_account() -> String {
    "cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng".to_string()
}

pub fn get_dummy_raw_msg_conn_open_try(
    proof_height: u64,
    consensus_height: u64,
) -> RawMsgConnectionOpenTry {
    let client_state_height = Height::new(0, consensus_height).unwrap();

    #[allow(deprecated)]
    RawMsgConnectionOpenTry {
        client_id: ClientId::default().to_string(),
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

pub fn get_dummy_proof() -> Vec<u8> {
    "Y29uc2Vuc3VzU3RhdGUvaWJjb25lY2xpZW50LzIy"
        .as_bytes()
        .to_vec()
}
