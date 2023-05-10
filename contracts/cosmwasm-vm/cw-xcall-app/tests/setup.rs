use cosmwasm_std::{
    coins,
    testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        MOCK_CONTRACT_ADDR,
    },
    Addr, Api, BlockInfo, ContractInfo, Empty, Env, IbcEndpoint, MessageInfo, OwnedDeps, StdResult,
    Storage, Timestamp, TransactionInfo,
};
use cw_multi_test::{App, AppBuilder, AppResponse, Contract, ContractWrapper, Executor, Router};
pub struct TestContext {
    pub app: App,
    pub connection_host: Addr,
    pub ibc_host: Addr,
    pub xcall_app: Addr,
    pub sender: Addr,
}
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

#[cfg(test)]
pub mod test {
    use super::*;
    pub fn create_mock_info(creator: &str, denom: &str, amount: u128) -> MessageInfo {
        let funds = coins(amount, denom);
        mock_info(creator, &funds)
    }

    pub fn deps() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
        mock_dependencies()
    }

    pub fn ibc_connection_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw_xcall_ibc_connection::execute,
            cw_xcall_ibc_connection::instantiate,
            cw_xcall_ibc_connection::query,
        )
        .with_reply(cw_xcall_ibc_connection::reply);
        Box::new(contract)
    }

    pub fn ibc_host_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw_mock_ibc_core::contract::execute,
            cw_mock_ibc_core::contract::instantiate,
            cw_mock_ibc_core::contract::query,
        )
        .with_reply(cw_mock_ibc_core::contract::reply);
        Box::new(contract)
    }

    pub fn xcall_app_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw_xcall_app::execute,
            cw_xcall_app::instantiate,
            cw_xcall_app::query,
        )
        .with_reply(cw_xcall_app::reply);
        Box::new(contract)
    }
    pub fn init_fn<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT>(
        _: &mut Router<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT>,
        _: &dyn Api,
        storage: &mut dyn Storage,
    ) {
        println!("Calling Build Function");
        let connection_storage = CwIbcConnection::default();


        let ibc_config = mock_ibc_config();

        connection_storage
            .ibc_config()
            .save(storage, &ibc_config)
            .unwrap();
        assert_eq!(
            connection_storage.ibc_config().load(storage).unwrap(),
            mock_ibc_config()
        );
        connection_storage.set_timeout_height(storage, 10).unwrap();
    }

    pub fn setup_contracts(deps: OwnedDeps<MockStorage, MockApi, MockQuerier>) -> TestContext {
        let mut router = AppBuilder::default()
            .with_storage(deps.storage)
            .build(init_fn);
        let sender = Addr::unchecked("sender");
        let ibc_connection_contract_code_id = router.store_code(ibc_connection_contract());
        let xcall_app_contractcode_id = router.store_code(xcall_app_contract());
        let ibc_host_contract_code_id = router.store_code(ibc_host_contract());

        let ibc_host_contract_addr = router
            .instantiate_contract(
                ibc_host_contract_code_id,
                sender.clone(),
                &cw_mock_ibc_core::msg::InstantiateMsg {},
                &[],
                "IBCHost",
                Some(sender.clone().to_string()),
            )
            .unwrap();

        let ibc_connection_contract_addr = router
            .instantiate_contract(
                ibc_connection_contract_code_id,
                sender.clone(),
                &cw_xcall_ibc_connection::msg::InstantiateMsg {
                    timeout_height: 1000,
                    ibc_host: ibc_host_contract_addr.clone(),
                },
                &[],
                "IBCConnection",
                Some(sender.clone().to_string()),
            )
            .unwrap();

        let xcall_app_contract_addr = router
            .instantiate_contract(
                xcall_app_contractcode_id,
                sender.clone(),
                &cw_xcall_app::msg::InstantiateMsg {
                    connection_host: ibc_connection_contract_addr.clone(),
                    timeout_height: 10,
                },
                &[],
                "XCallApp",
                Some(sender.clone().to_string()),
            )
            .unwrap();
        router.init_modules(init_fn);

        TestContext {
            app: router,
            connection_host: ibc_connection_contract_addr,
            xcall_app: xcall_app_contract_addr,
            ibc_host: ibc_host_contract_addr,
            sender,
        }
    }

    pub fn mock_ibc_config() -> IbcConfig {
        let src = IbcEndpoint {
            port_id: "our-port".to_string(),
            channel_id: "channel-1".to_string(),
        };

        let dst = IbcEndpoint {
            port_id: "their-port".to_string(),
            channel_id: "channel-3".to_string(),
        };

        let ibc_config = IbcConfig::new(src, dst);
        ibc_config
    }

    use anyhow::Error as AppError;
    use cosmwasm_std::to_vec;
    use cw_xcall_ibc_connection::state::{CwIbcConnection, IbcConfig};

    pub fn call_send_call_message(
        ctx: &mut TestContext,
        to: &str,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
    ) -> Result<AppResponse, AppError> {
        let res = ctx.app.execute_contract(
            ctx.sender.clone(),
            ctx.xcall_app.clone(),
            &cw_common::xcall_app_msg::ExecuteMsg::SendCallMessage {
                to: to.to_string(),
                data,
                rollback,
            },
            &[],
        );
        res
    }

    pub fn call_set_xcall_host(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
        let res = ctx.app.execute_contract(
            ctx.sender.clone(),
            ctx.connection_host.clone(),
            &cw_common::xcall_connection_msg::ExecuteMsg::SetXCallHost {
                address: ctx.xcall_app.to_string(),
            },
            &[],
        );
        res
    }
    pub fn call_set_ibc_config(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
        let config = to_vec(&mock_ibc_config()).unwrap();
        let res = ctx.app.execute_contract(
            ctx.sender.clone(),
            ctx.connection_host.clone(),
            &cw_common::xcall_connection_msg::ExecuteMsg::SetIbcConfig { ibc_config: config },
            &[],
        );
        res
    }
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
