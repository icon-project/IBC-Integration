mod setup;
use anyhow::Error as AppError;
use common::constants::ICON_CLIENT_TYPE;
use common::ibc::events::IbcEventType;
use common::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use common::{icon::icon::lightclient::v1::ClientState as RawClientState, traits::AnyTypes};
use cosmwasm_std::{from_binary, to_binary, Addr, Binary, Empty, Querier, QueryRequest};
use cw_common::raw_types::client::{RawMsgCreateClient, RawMsgUpdateClient};
use cw_common::{core_msg as CoreMsg, hex_string::HexString};
use cw_ibc_core::{execute, instantiate, query, reply};
use cw_icon_light_client;
use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};
use prost::Message;
use setup::{
    init_ibc_core_contract, init_light_client, init_xcall_mock_contract, setup_context, TestContext,
};
use test_utils::{
    get_event, get_event_name, get_test_signed_headers, load_raw_messages, RawPayload,
};

fn setup_test() -> TestContext {
    let mut context = setup_context();
    context = setup_contracts(context);
    context
}

pub fn setup_contracts(mut ctx: TestContext) -> TestContext {
    ctx = init_light_client(ctx);
    ctx = init_ibc_core_contract(ctx);
    let ibc_addr = ctx.get_ibc_core().clone();
    ctx = init_xcall_mock_contract(ctx, ibc_addr);
    ctx
}

pub fn call_register_client_type(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core().clone(),
        &CoreMsg::ExecuteMsg::RegisterClient {
            client_type: ICON_CLIENT_TYPE.to_string(),
            client_address: ctx.get_light_client().clone(),
        },
        &[],
    );
    res
}

pub fn call_create_client(ctx: &mut TestContext, msg: HexString) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core().clone(),
        &CoreMsg::ExecuteMsg::CreateClient { msg },
        &[],
    );

    res
}

pub fn call_update_client(ctx: &mut TestContext, msg: HexString) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core().clone(),
        &CoreMsg::ExecuteMsg::UpdateClient { msg },
        &[],
    );

    res
}

pub fn call_connection_open_init(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core().clone(),
        &CoreMsg::ExecuteMsg::ConnectionOpenInit { msg },
        &[],
    );

    res
}

pub fn call_connection_open_try(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core().clone(),
        &CoreMsg::ExecuteMsg::ConnectionOpenTry { msg },
        &[],
    );

    res
}

pub fn call_connection_open_ack(
    ctx: &mut TestContext,
    msg: HexString,
    client_id: &str,
) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core().clone(),
        &CoreMsg::ExecuteMsg::ConnectionOpenAck { msg },
        &[],
    );

    res
}

pub fn call_connection_open_confirm(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core().clone(),
        &CoreMsg::ExecuteMsg::ConnectionOpenConfirm { msg },
        &[],
    );

    res
}

pub fn call_channel_open_try(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core().clone(),
        &CoreMsg::ExecuteMsg::ChannelOpenTry { msg },
        &[],
    );

    res
}

pub fn call_channel_open_confirm(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core().clone(),
        &CoreMsg::ExecuteMsg::ChannelOpenConfirm { msg },
        &[],
    );

    res
}

#[test]
fn test_register_client() {
    let mut ctx = setup_test();
    let result = call_register_client_type(&mut ctx);
    assert!(result.is_ok());
}

#[test]
fn test_create_client() {
    let mut ctx = setup_test();
    call_register_client_type(&mut ctx).unwrap();
    let signed_headers: Vec<RawPayload> = load_raw_messages();
    let result = call_create_client(
        &mut ctx,
        HexString::from_str(signed_headers[0].message.as_str()),
    );
    println!("{:?}", &result);
    assert!(result.is_ok());
}
#[test]
fn test_update_client() {
    let mut ctx = setup_test();
    call_register_client_type(&mut ctx).unwrap();
    let signed_headers: Vec<RawPayload> = load_raw_messages();
    let response = call_create_client(
        &mut ctx,
        HexString::from_str(signed_headers[0].message.as_str()),
    )
    .unwrap();
    let event = get_event(&response, &get_event_name(IbcEventType::CreateClient)).unwrap();
    let client_id = event.get("client_id").unwrap();
    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[1].update.clone().unwrap().as_str()),
    );
    println!("{:?}", &result);
    assert!(result.is_ok());
}

pub fn build_query(contract: String, msg: Binary) -> QueryRequest<Empty> {
    QueryRequest::Wasm(cosmwasm_std::WasmQuery::Smart {
        contract_addr: contract,
        msg,
    })
}

pub fn query_get_capability(app: &App, port_id: String, contract_address: Addr) -> String {
    let query = cw_ibc_core::msg::QueryMsg::GetCapability { name: port_id };
    let query = build_query(contract_address.to_string(), to_binary(&query).unwrap());

    let balance = app.raw_query(&to_binary(&query).unwrap()).unwrap().unwrap();
    println!("balances {:?}", balance);
    let res: String = from_binary(&balance).unwrap();
    res
}

fn call_bind_port(ctx: &mut TestContext, port_name: &str) -> Result<AppResponse, AppError> {
    let res = ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core().clone(),
        &CoreMsg::ExecuteMsg::BindPort {
            port_id: port_name.to_string(),
            address: ctx.get_xcall_app().clone().to_string(),
        },
        &[],
    );

    res
}

#[test]
fn test_connection_open_init() {
    let mut ctx = setup_test();
    let port_name = "mock-7";
    call_bind_port(&mut ctx, port_name.clone()).unwrap();
    call_register_client_type(&mut ctx).unwrap();
    let res = query_get_capability(&ctx.app, port_name.to_string(), ctx.get_ibc_core().clone());

    println!("mock app address {:?}", res);

    let signed_headers: Vec<RawPayload> = load_raw_messages();
    let response = call_create_client(
        &mut ctx,
        HexString::from_str(signed_headers[0].message.as_str()),
    )
    .unwrap();
    let event = get_event(&response, &get_event_name(IbcEventType::CreateClient)).unwrap();
    let client_id = event.get("client_id").unwrap();
    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[1].update.clone().unwrap().as_str()),
    );
    println!("{:?}", &result);
    assert!(result.is_ok());

    let result = call_connection_open_try(
        &mut ctx,
        HexString::from_str(signed_headers[1].message.clone().as_str()),
    );

    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[2].update.clone().unwrap().as_str()),
    );
    println!("{:?}", &result);
    assert!(result.is_ok());

    let result = call_connection_open_confirm(
        &mut ctx,
        HexString::from_str(signed_headers[2].message.clone().as_str()),
    );

    println!("this is nepalllllll{:?}", &result);
    assert!(result.is_ok());

    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[3].update.clone().unwrap().as_str()),
    );
    println!("{:?}", &result);
    assert!(result.is_ok());

    let result = call_channel_open_try(
        &mut ctx,
        HexString::from_str(signed_headers[3].message.clone().as_str()),
    );

    println!("this is nepalllllll{:?}", &result);
    assert!(result.is_ok());

    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[4].update.clone().unwrap().as_str()),
    );
    println!("{:?}", &result);
    assert!(result.is_ok());

    let result = call_channel_open_confirm(
        &mut ctx,
        HexString::from_str(signed_headers[4].message.clone().as_str()),
    );

    println!("this is nepalllllll{:?}", &result);
    assert!(result.is_ok());
}
