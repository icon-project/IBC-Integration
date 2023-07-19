use std::{collections::HashMap, str::FromStr};

use cosmwasm_std::{
    coins,
    testing::{mock_dependencies, mock_info, MockApi, MockQuerier, MockStorage},
    Addr, Empty, IbcEndpoint, MessageInfo, OwnedDeps,
};
use cw_integration::TestSteps;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use cw_xcall_ibc_connection::state::IbcConfig;

use test_utils::{IntegrationData, RawPayload};

pub const PORT: &str = "xcall";
pub const COUNTERPARTY_NID: &str = "0x3.icon";

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
    pub test_data: Option<HashMap<TestSteps, RawPayload>>,
    pub admin: Option<String>,
    pub caller: Option<String>,
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
        self.contracts.insert(TestApps::LightClient, addr)
    }
    pub fn set_dapp(&mut self, addr: Addr) -> Option<Addr> {
        self.contracts.insert(TestApps::Dapp, addr)
    }
    pub fn set_ibc_core(&mut self, addr: Addr) -> Option<Addr> {
        self.contracts.insert(TestApps::IbcCore, addr)
    }

    pub fn set_xcall_app(&mut self, addr: Addr) -> Option<Addr> {
        self.contracts.insert(TestApps::XcallApp, addr)
    }
    pub fn set_xcall_ibc_connection(&mut self, addr: Addr) -> Option<Addr> {
        self.contracts.insert(TestApps::XcallIbcConnection, addr)
    }

    pub fn get_test_data(&self, step: &TestSteps) -> RawPayload {
        let payload = self
            .test_data
            .as_ref()
            .and_then(|map| map.get(step))
            .cloned();
        let err = format!("Payload not Found for {step:?}");
        payload.expect(&err)
    }

    pub fn list_contracts(&self) {
        println!("Lightclient {}", self.get_light_client());
        println!("IbcHost {}", self.get_ibc_core());
        println!("IbcConnection {}", self.get_xcall_ibc_connection());
        println!("Xcall {}", self.get_xcall_app());
        println!("Dapp {}", self.get_dapp())
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

    IbcConfig::new(src, dst)
}

pub fn mock_dapp_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_mock_dapp::execute,
        cw_mock_dapp::instantiate,
        cw_mock_dapp::query,
    );
    Box::new(contract)
}

pub fn mock_dapp_multi_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_mock_dapp_multi::execute,
        cw_mock_dapp_multi::instantiate,
        cw_mock_dapp_multi::query,
    );
    Box::new(contract)
}

pub fn init_mock_dapp_multi_contract(mut ctx: TestContext) -> TestContext {
    let code_id = ctx.app.store_code(mock_dapp_multi_contract());
    let contract_addr = ctx
        .app
        .instantiate_contract(
            code_id,
            ctx.sender.clone(),
            &cw_mock_dapp_multi::types::InstantiateMsg {
                address: ctx.get_xcall_app().to_string(),
            },
            &[],
            "MockApp",
            Some(ctx.sender.clone().to_string()),
        )
        .unwrap();
    ctx.set_dapp(contract_addr);

    ctx
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
            &cw_common::client_msg::InstantiateMsg {
                ibc_host: ctx.get_ibc_core(),
            },
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
            ctx.admin.clone(),
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
    let contract = ContractWrapper::new(cw_xcall::execute, cw_xcall::instantiate, cw_xcall::query)
        .with_reply(cw_xcall::reply);
    Box::new(contract)
}

pub fn init_xcall_app_contract(mut ctx: TestContext) -> TestContext {
    let xcall_app_contractcode_id = ctx.app.store_code(xcall_app_contract());
    let xcall_app_contract_addr = ctx
        .app
        .instantiate_contract(
            xcall_app_contractcode_id,
            ctx.sender.clone(),
            &cw_xcall::msg::InstantiateMsg {
                network_id: "nid".to_string(),
                denom: "uarch".to_string(),
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
                ibc_host: ctx.get_ibc_core(),
                denom: "uarch".to_string(),
                port_id: PORT.to_string(),
                xcall_address: ctx.get_xcall_app(),
            },
            &[],
            "IBCConnection",
            Some(ctx.sender.clone().to_string()),
        )
        .unwrap();
    ctx.set_xcall_ibc_connection(ibc_connection_contract_addr);
    ctx
}

pub fn raw_payload_to_map(payloads: Vec<RawPayload>) -> HashMap<TestSteps, RawPayload> {
    let mut map = HashMap::<TestSteps, RawPayload>::new();
    for payload in payloads {
        let key = TestSteps::from_str(&payload.step).unwrap();
        map.insert(key, payload);
    }
    map
}

// pub fn get_icon_to_archway_payloads() -> HashMap<TestSteps, RawPayload> {
//     let payloads = load_raw_payloads_icon_to_archway();
//     return raw_payload_to_map(payloads);
// }

pub fn setup_context(data: Option<IntegrationData>) -> TestContext {
    let router = App::default();
    let sender = Addr::unchecked("sender");
    if let Some(data) = data {
        let test_data = raw_payload_to_map(data.data);
        return TestContext {
            app: router,
            contracts: HashMap::new(),
            sender,
            test_data: Some(test_data),
            admin: Some(data.address.clone()),
            caller: data.caller_address,
        };
    }

    TestContext {
        app: router,
        contracts: HashMap::new(),
        sender,
        test_data: None,
        admin: None,
        caller: None,
    }
}
