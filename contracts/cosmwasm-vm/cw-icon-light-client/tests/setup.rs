use cosmwasm_std::{
    coins,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, BlockInfo, ContractInfo, Empty, Env, MessageInfo, OwnedDeps, Timestamp, TransactionInfo,
};

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

    pub fn add_txn_info(mut self, txn_info: TransactionInfo) -> MockEnvBuilder {
        self.env.transaction = Some(txn_info);
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

    let mock_env_builder: Env = MockEnvBuilder::new().add_block(mock.block.clone()).build();

    assert_eq!(mock.block, mock_env_builder.block)
}
