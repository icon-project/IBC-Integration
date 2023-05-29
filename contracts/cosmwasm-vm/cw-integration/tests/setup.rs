use std::collections::HashMap;

use cosmwasm_std::{
    coins,
    testing::{mock_dependencies, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Empty, IbcEndpoint, MessageInfo, OwnedDeps,
};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use cw_xcall_ibc_connection::state::IbcConfig;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TestApps {
    LightClient,
    Dapp,
    IbcCore,
    XcallApp,
    XcallIbcConnection,
}

pub struct TestContext {
    pub app: App,
    pub contracts: HashMap<TestApps, Addr>,
    pub sender: Addr,
}

impl TestContext {
    pub fn get_light_client(&self) -> Addr {
        return self.contracts.get(&TestApps::LightClient).unwrap().clone();
    }
    pub fn get_dapp(&self) -> Addr {
        return self.contracts.get(&TestApps::Dapp).unwrap().clone();
    }
    pub fn get_ibc_core(&self) -> Addr {
        return self.contracts.get(&TestApps::IbcCore).unwrap().clone();
    }

    pub fn get_xcall_app(&self) -> Addr {
        return self.contracts.get(&TestApps::XcallApp).unwrap().clone();
    }
    pub fn get_xcall_ibc_connection(&self) -> Addr {
        return self
            .contracts
            .get(&TestApps::XcallIbcConnection)
            .unwrap()
            .clone();
    }

    pub fn set_light_client(&mut self, addr: Addr) -> Option<Addr> {
        return self.contracts.insert(TestApps::LightClient, addr);
    }
    pub fn set_dapp(&mut self, addr: Addr) -> Option<Addr> {
        return self.contracts.insert(TestApps::Dapp, addr);
    }
    pub fn set_ibc_core(&mut self, addr: Addr) -> Option<Addr> {
        return self.contracts.insert(TestApps::IbcCore, addr);
    }

    pub fn set_xcall_app(&mut self, addr: Addr) -> Option<Addr> {
        return self.contracts.insert(TestApps::XcallApp, addr);
    }
    pub fn set_xcall_ibc_connection(&mut self, addr: Addr) -> Option<Addr> {
        return self.contracts.insert(TestApps::XcallIbcConnection, addr);
    }
}

pub fn create_mock_info(creator: &str, denom: &str, amount: u128) -> MessageInfo {
    let funds = coins(amount, denom);
    mock_info(creator, &funds)
}

pub fn deps() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    mock_dependencies()
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

pub fn mock_dapp_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_mock_dapp::execute,
        cw_mock_dapp::instantiate,
        cw_mock_dapp::query,
    );
    Box::new(contract)
}

pub fn init_mock_dapp_contract(mut ctx: TestContext) -> TestContext {
    let code_id = ctx.app.store_code(mock_dapp_contract());
    let contract_addr = ctx
        .app
        .instantiate_contract(
            code_id,
            ctx.sender.clone(),
            &cw_mock_dapp::types::InstantiateMsg {
                address: "someaddr".to_string(),
            },
            &[],
            "MockApp",
            Some(ctx.sender.clone().to_string()),
        )
        .unwrap();
    ctx.set_dapp(contract_addr);

    ctx
}

pub fn ibc_core_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_ibc_core::execute,
        cw_ibc_core::instantiate,
        cw_ibc_core::query,
    )
    .with_reply(cw_ibc_core::reply);
    Box::new(contract)
}

pub fn lightclient_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_icon_light_client::contract::execute,
        cw_icon_light_client::contract::instantiate,
        cw_icon_light_client::contract::query,
    );
    Box::new(contract)
}

pub fn init_light_client(mut ctx: TestContext) -> TestContext {
    let light_client_code_id = ctx.app.store_code(lightclient_contract());

    let light_client_addr = ctx
        .app
        .instantiate_contract(
            light_client_code_id,
            ctx.sender.clone(),
            &cw_common::client_msg::InstantiateMsg::default(),
            &[],
            "LightClient",
            Some(ctx.sender.clone().to_string()),
        )
        .unwrap();
    ctx.set_light_client(light_client_addr);

    ctx
}

pub fn init_ibc_core_contract(mut ctx: TestContext) -> TestContext {
    let ibc_core_code_id = ctx.app.store_code(ibc_core_contract());
    let ibc_core_addr = ctx
        .app
        .instantiate_contract(
            ibc_core_code_id,
            ctx.sender.clone(),
            &cw_common::core_msg::InstantiateMsg {},
            &[],
            "IBCCore",
            Some(ctx.sender.clone().to_string()),
        )
        .unwrap();

    ctx.set_ibc_core(ibc_core_addr);

    ctx
}

pub fn init_mock_ibc_core_contract(mut ctx: TestContext) -> TestContext {
    let ibc_core_code_id = ctx.app.store_code(mock_ibc_core_contract());
    let ibc_core_addr = ctx
        .app
        .instantiate_contract(
            ibc_core_code_id,
            ctx.sender.clone(),
            &cw_common::core_msg::InstantiateMsg {},
            &[],
            "IBCCore",
            Some(ctx.sender.clone().to_string()),
        )
        .unwrap();

    ctx.set_ibc_core(ibc_core_addr);
    ctx
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

pub fn mock_ibc_core_contract() -> Box<dyn Contract<Empty>> {
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

pub fn init_xcall_app_contract(mut ctx: TestContext) -> TestContext {
    let xcall_app_contractcode_id = ctx.app.store_code(xcall_app_contract());
    let xcall_app_contract_addr = ctx
        .app
        .instantiate_contract(
            xcall_app_contractcode_id,
            ctx.sender.clone(),
            &cw_xcall_app::msg::InstantiateMsg {
                connection_host: ctx.get_xcall_ibc_connection().clone(),
                timeout_height: 10,
            },
            &[],
            "XCallApp",
            Some(ctx.sender.clone().to_string()),
        )
        .unwrap();

    ctx.set_xcall_app(xcall_app_contract_addr);
    ctx
}

pub fn init_xcall_ibc_connection_contract(mut ctx: TestContext) -> TestContext {
    let ibc_connection_contract_code_id = ctx.app.store_code(ibc_connection_contract());
    let ibc_connection_contract_addr = ctx
        .app
        .instantiate_contract(
            ibc_connection_contract_code_id,
            ctx.sender.clone(),
            &cw_xcall_ibc_connection::msg::InstantiateMsg {
                timeout_height: 1000,
                ibc_host: ctx.get_ibc_core(),
                protocol_fee: 0,
            },
            &[],
            "IBCConnection",
            Some(ctx.sender.clone().to_string()),
        )
        .unwrap();
    ctx.set_xcall_app(ibc_connection_contract_addr);
    ctx
}

pub fn setup_context() -> TestContext {
    let mut router = App::default();
    let sender = Addr::unchecked("sender");
    return TestContext {
        app: router,
        contracts: HashMap::new(),
        sender,
    };
}
