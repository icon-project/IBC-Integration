use std::collections::HashMap;

use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage},
    Addr, Empty, OwnedDeps,
};
use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

pub struct TestContext {
    pub app: App,
    pub contracts: HashMap<String, Addr>,
    pub sender: Addr,
}

impl TestContext {
    pub fn get_light_client(&self) -> Addr {
        return self.contracts.get("light_client").unwrap().clone();
    }
    pub fn get_dapp(&self) -> Addr {
        return self.contracts.get("dapp").unwrap().clone();
    }
    pub fn get_ibc_core(&self) -> Addr {
        return self.contracts.get("ibc_core").unwrap().clone();
    }

    pub fn get_xcall_app(&self) -> Addr {
        return self.contracts.get("xcall_app").unwrap().clone();
    }
    pub fn get_xcall_ibc_connection(&self) -> Addr {
        return self.contracts.get("xcall_ibc_connection").unwrap().clone();
    }
}

pub fn mock_dapp_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_mock_dapp::execute,
        cw_mock_dapp::instantiate,
        cw_mock_dapp::query,
    );
    Box::new(contract)
}

pub fn init_mock_dapp_contract(app: &mut App) -> Addr {
    let code_id = app.store_code(mock_dapp_contract());
    let sender = Addr::unchecked("sender");
    let contract_addr = app
        .instantiate_contract(
            code_id,
            sender.clone(),
            &cw_mock_dapp::types::InstantiateMsg {
                address: "someaddr".to_string(),
            },
            &[],
            "MockApp",
            Some(sender.clone().to_string()),
        )
        .unwrap();
    contract_addr
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
    ctx.contracts
        .insert("light_client".to_string(), light_client_addr);

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
    ctx.contracts.insert("ibc_core".to_string(), ibc_core_addr);

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
    ctx.contracts
        .insert("xcall_app".to_string(), xcall_app_contract_addr);
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
    ctx.contracts.insert(
        "xcall_ibc_connection".to_owned(),
        ibc_connection_contract_addr,
    );
    ctx
}

// pub fn setup_contracts(deps: OwnedDeps<MockStorage, MockApi, MockQuerier>) -> TestContext {
//     let mut router = AppBuilder::default()
//         .with_storage(deps.storage)
//         .build(init_fn);
//     let sender = Addr::unchecked("sender");
//     let ibc_connection_contract_code_id = router.store_code(ibc_connection_contract());
//     let xcall_app_contractcode_id = router.store_code(xcall_app_contract());
//     let ibc_host_contract_code_id = router.store_code(ibc_host_contract());

//     let ibc_host_contract_addr = router
//         .instantiate_contract(
//             ibc_host_contract_code_id,
//             sender.clone(),
//             &cw_mock_ibc_core::msg::InstantiateMsg {},
//             &[],
//             "IBCHost",
//             Some(sender.clone().to_string()),
//         )
//         .unwrap();

//     let ibc_connection_contract_addr = router
//         .instantiate_contract(
//             ibc_connection_contract_code_id,
//             sender.clone(),
//             &cw_xcall_ibc_connection::msg::InstantiateMsg {
//                 timeout_height: 1000,
//                 ibc_host: ibc_host_contract_addr.clone(),
//                 protocol_fee: 0,
//             },
//             &[],
//             "IBCConnection",
//             Some(sender.clone().to_string()),
//         )
//         .unwrap();

//     let xcall_app_contract_addr = router
//         .instantiate_contract(
//             xcall_app_contractcode_id,
//             sender.clone(),
//             &cw_xcall_app::msg::InstantiateMsg {
//                 connection_host: ibc_connection_contract_addr.clone(),
//                 timeout_height: 10,
//             },
//             &[],
//             "XCallApp",
//             Some(sender.clone().to_string()),
//         )
//         .unwrap();
//     router.init_modules(init_fn);

//     TestContext {
//         app: router,
//         connection_host: ibc_connection_contract_addr,
//         xcall_app: xcall_app_contract_addr,
//         ibc_host: ibc_host_contract_addr,
//         sender,
//     }
// }

/// setup contracts
///

pub fn setup_xcall_app_host(mut ctx: TestContext) -> TestContext {
    ctx = init_ibc_core_contract(ctx);
    ctx = init_xcall_ibc_connection_contract(ctx);
    ctx = init_xcall_app_contract(ctx);
    ctx
}

pub fn setup_host_lightclient(mut ctx: TestContext) -> TestContext {
    ctx = init_light_client(ctx);
    ctx = init_ibc_core_contract(ctx);

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
