use common::{ibc::dynamic_typing::AsAny, icon::icon::types::v1::BtpHeader, traits::AnyTypes};
#[cfg(test)]
use cosmwasm_std::{
    coins,
    testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        MOCK_CONTRACT_ADDR,
    },
    Addr, BlockInfo, ContractInfo, Empty, Env, MessageInfo, OwnedDeps, Timestamp, TransactionInfo,
};
use cosmwasm_std::{DepsMut, Storage};
use cw_common::raw_types::Any;
use cw_wasm_light_client::{
    query_handler::QueryHandler,
    utils::{
        get_client_state_key, get_consensus_state_key, to_ibc_height, to_wasm_consensus_state,
    },
};
use ics07_tendermint_cw::ics23::FakeInner;
use prost::Message;

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

    let transaction = Some(TransactionInfo { index: 3 });
    let contract = ContractInfo {
        address: Addr::unchecked(MOCK_CONTRACT_ADDR),
    };

    let mock_env: Env = MockEnvBuilder::new()
        .add_block(block_info)
        .add_txn_info(transaction)
        .add_contract_info(contract)
        .build();

    assert_eq!(mock, mock_env)
}

pub struct TestContext {}

impl Default for TestContext {
    fn default() -> Self {
        TestContext {}
    }
}

impl TestContext {
    pub fn init(&self, storage: &mut dyn Storage, header: &BtpHeader) {
        let client_state = header.to_client_state(1000000, 0);
        let consensus_state = header.to_consensus_state();
        let client_key = get_client_state_key();
        let consensus_key = get_consensus_state_key(to_ibc_height(client_state.latest_height));
        let client_state_any = client_state.to_any();

        let wasm_client = ics08_wasm::client_state::ClientState::<FakeInner, FakeInner, FakeInner> {
            data: client_state_any.encode_to_vec(),
            code_id: vec![1],
            latest_height: to_ibc_height(client_state.latest_height),
            inner: Box::new(FakeInner),
            _phantom: std::marker::PhantomData,
        };
        let wasm_any_bytes = wasm_client.to_any().encode_to_vec();

        storage.set(&client_key, &wasm_any_bytes);

        let wasm_consensus = to_wasm_consensus_state(consensus_state);

        storage.set(&consensus_key, &wasm_consensus)
    }

    pub fn for_instantiate() -> TestContext {
        let context = TestContext::default();
        context
    }
}
