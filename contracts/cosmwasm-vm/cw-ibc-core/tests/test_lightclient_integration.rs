use cosmwasm_std::{to_binary, Addr, Empty, Uint128, WasmMsg};

use cw_icon_light_client;
use cw_multi_test::{App, Contract, ContractWrapper, Executor, AppResponse};

use cw_ibc_core::{execute, instantiate, query};
use cw_common::core_msg as CoreMsg;
use anyhow::Error as AppError;
use common::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use common::icon::icon::lightclient::v1::ClientState as RawClientState;

pub struct TestContext {
    pub app:App,
    pub lightclient:Addr,
    pub ibc_core:Addr,
    pub sender:Addr,
}

fn mock_app() -> App {
    App::default()
}

pub fn ibc_core_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
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

pub fn setup_test() -> TestContext{
    let mut router = mock_app();
    let sender = Addr::unchecked("sender");
    let light_client_code_id = router.store_code(lightclient_contract());
    let ibc_core_code_id = router.store_code(ibc_core_contract());

    let light_client_addr = router
        .instantiate_contract(
            light_client_code_id,
            sender.clone(),
            &cw_common::client_msg::InstantiateMsg::default(),
            &[],
            "LightClient",
            Some(sender.clone().to_string()),
        )
        .unwrap();

    let ibc_core_addr = router
        .instantiate_contract(
            ibc_core_code_id,
            sender.clone(),
            &cw_common::core_msg::InstantiateMsg {},
            &[],
            "IBCCore",
            Some(sender.clone().to_string()),
        )
        .unwrap();

    TestContext { app: router, lightclient:light_client_addr, ibc_core: ibc_core_addr, sender}
}


pub fn call_register_client_type(ctx:&mut TestContext)->Result<AppResponse,AppError>{
    let res= ctx.app.execute_contract(ctx.sender.clone(), 
    ctx.ibc_core.clone(), 
    &CoreMsg::ExecuteMsg::RegisterClient { client_type: "icon-client".to_string(), 
    client_address: ctx.lightclient.clone()}, &[]);
    res
}

pub fn call_create_client(ctx:&mut TestContext,signed_header:RawSignedHeader){
    let client_state:RawClientState= signed_header.header.clone().unwrap().to_client_state(1000000, 5);
    let consensus_state= signed_header.header.unwrap().to_consensus_state();
    
    

}

#[test]
fn test_register_client(){
    let mut ctx=setup_test();
    let result= call_register_client_type(&mut ctx);
    assert!(result.is_ok());
}