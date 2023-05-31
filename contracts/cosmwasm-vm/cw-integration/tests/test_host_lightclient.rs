mod setup;
use anyhow::Error as AppError;
use common::constants::ICON_CLIENT_TYPE;
use common::ibc::events::IbcEventType;
use common::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use common::{icon::icon::lightclient::v1::ClientState as RawClientState, traits::AnyTypes};
use cosmwasm_std::{Addr, Empty};
use cw_common::raw_types::client::{RawMsgCreateClient, RawMsgUpdateClient};
use cw_common::{core_msg as CoreMsg, hex_string::HexString};
use cw_ibc_core::{execute, instantiate, query, reply};
use cw_icon_light_client;
use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};
use prost::Message;
use setup::{init_ibc_core_contract, init_light_client, setup_context, TestContext};
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

#[test]
fn test_connection_open_init() {
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

    println!("not reached here");

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

    println!("not reached here");

    let result = call_connection_open_confirm(
        &mut ctx,
        HexString::from_str(signed_headers[2].message.clone().as_str()),
    );
    println!("this is nepalllllll{:?}", &result);
    assert!(result.is_ok());
}
