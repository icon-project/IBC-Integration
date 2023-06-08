mod setup;
use std::{process::Termination, str::FromStr};

use anyhow::Error as AppError;
use common::constants::ICON_CLIENT_TYPE;
use common::ibc::events::IbcEventType;

use cosmwasm_std::{from_binary, to_binary, Addr, Empty, Querier, QueryRequest};

use cw_common::{core_msg as CoreMsg, hex_string::HexString, query_helpers::build_smart_query};

use cw_multi_test::{App, AppResponse, Executor};

use setup::{
    init_ibc_core_contract, init_light_client, init_xcall_mock_contract, setup_context, TestContext,
};
use test_utils::{get_event, get_event_name, load_a2i_raw_messages, load_raw_messages, RawPayload};

fn setup_test() -> TestContext {
    let mut context = setup_context();
    context = setup_contracts(context);
    context
}

pub fn setup_contracts(mut ctx: TestContext) -> TestContext {
    ctx = init_light_client(ctx);
    ctx = init_ibc_core_contract(ctx);
    let ibc_addr = ctx.get_ibc_core();
    ctx = init_xcall_mock_contract(ctx, ibc_addr);
    ctx
}

pub fn call_register_client_type(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::RegisterClient {
            client_type: ICON_CLIENT_TYPE.to_string(),
            client_address: ctx.get_light_client(),
        },
        &[],
    )
}

pub fn call_create_client(ctx: &mut TestContext, msg: HexString) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::CreateClient { msg },
        &[],
    )
}

pub fn call_update_client(ctx: &mut TestContext, msg: HexString) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::UpdateClient { msg },
        &[],
    )
}

pub fn call_connection_open_init(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ConnectionOpenInit { msg },
        &[],
    )
}

pub fn call_connection_open_try(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ConnectionOpenTry { msg },
        &[],
    )
}

pub fn call_connection_open_ack(
    ctx: &mut TestContext,
    msg: HexString,
    _client_id: &str,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ConnectionOpenAck { msg },
        &[],
    )
}

pub fn call_connection_open_confirm(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ConnectionOpenConfirm { msg },
        &[],
    )
}

pub fn call_channel_open_init(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ChannelOpenInit { msg },
        &[],
    )
}

pub fn call_channel_open_try(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ChannelOpenTry { msg },
        &[],
    )
}

pub fn call_channel_open_ack(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ChannelOpenAck { msg },
        &[],
    )
}

pub fn call_channel_open_confirm(
    ctx: &mut TestContext,
    msg: HexString,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ChannelOpenConfirm { msg },
        &[],
    )
}

pub fn call_receive_packet(ctx: &mut TestContext, msg: HexString) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ReceivePacket { msg },
        &[],
    )
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
        HexString::from_str(signed_headers[0].message.as_str()).unwrap(),
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
        HexString::from_str(signed_headers[0].message.as_str()).unwrap(),
    )
    .unwrap();
    let event = get_event(&response, &get_event_name(IbcEventType::CreateClient)).unwrap();
    let _client_id = event.get("client_id").unwrap();
    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[1].update.clone().unwrap().as_str()).unwrap(),
    );
    println!("{:?}", &result);
    assert!(result.is_ok());
}

pub fn query_get_capability(app: &App, port_id: String, contract_address: Addr) -> String {
    let query = cw_common::core_msg::QueryMsg::GetCapability { name: port_id };
    let query: QueryRequest<Empty> =
        build_smart_query(contract_address.to_string(), to_binary(&query).unwrap());

    let balance = app.raw_query(&to_binary(&query).unwrap()).unwrap().unwrap();
    println!("balances {balance:?}");
    let res: String = from_binary(&balance).unwrap();
    res
}

// pub fn query_host_next_seq_send(app: &App, contract_address: Addr) -> u64 {
//     let query = cw_xcall::msg::QueryMsg::GetHostSendSeq {};
//     let query = build_query(contract_address.to_string(), to_binary(&query).unwrap());

//     let balance = app.raw_query(&to_binary(&query).unwrap()).unwrap().unwrap();
//     println!("balances {:?}", balance);
//     let res: u64 = from_binary(&balance).unwrap();
//     res
// }

fn call_bind_port(ctx: &mut TestContext, port_name: &str) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::BindPort {
            port_id: port_name.to_string(),
            address: ctx.get_xcall_app().to_string(),
        },
        &[],
    )
}

fn call_xcall_message(ctx: &mut TestContext, data: Vec<u8>) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_xcall_app(),
        &cw_common::xcall_msg::ExecuteMsg::SendCallMessage {
            to: "eth1".to_string(),
            data,
            rollback: None,
        },
        &[],
    )
}

#[test]
fn test_packet_receiver() {
    let mut ctx = test_connection_open_init();

    let signed_headers: Vec<RawPayload> = load_raw_messages();

    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[5].update.clone().unwrap().as_str()).unwrap(),
    );

    println!("{:?}", &result);
    assert!(result.is_ok());

    let result = call_receive_packet(
        &mut ctx,
        HexString::from_str(signed_headers[5].message.clone().as_str()).unwrap(),
    );

    println!("{:?}", &result);
    assert!(result.is_ok());
}

#[test]
fn test_packet_send() {
    let mut ctx = test_connection_open_init();

    let data = [90, 91, 90, 91];
    let result = call_xcall_message(&mut ctx, data.into());

    //  let result = query_host_next_seq_send(&ctx.app, ctx.get_xcall_app());

    println!("{:?}", &result);
}

#[test]
fn test_conn_handshake_start_on_archway() -> TestContext {
    let mut ctx = setup_test();
    let port_name = "mock";
    call_bind_port(&mut ctx, port_name.clone()).unwrap();
    call_register_client_type(&mut ctx).unwrap();
    let res = query_get_capability(&ctx.app, port_name.to_string(), ctx.get_ibc_core());

    println!("mock app address {res:?}");

    let signed_headers: Vec<RawPayload> = load_a2i_raw_messages();
    let response = call_create_client(
        &mut ctx,
        HexString::from_str(signed_headers[0].message.as_str()).unwrap(),
    )
    .unwrap();
    let event = get_event(&response, &get_event_name(IbcEventType::CreateClient)).unwrap();
    let _client_id = event.get("client_id").unwrap();

    println!("Client Id:: {:?}", _client_id);

    let result = call_connection_open_init(
        &mut ctx,
        HexString::from_str(signed_headers[1].message.clone().as_str()).unwrap(),
    );

    println!(" call_connection_open_init {:?}", &result);
    assert!(result.is_ok());

    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[2].update.clone().unwrap().as_str()).unwrap(),
    );
    println!("call_update client {:?}", &result);
    assert!(result.is_ok());

    let result = call_connection_open_ack(
        &mut ctx,
        HexString::from_str(signed_headers[2].message.clone().as_str()).unwrap(),
        &_client_id,
    );

    println!(" call_connection_open_ack {:?}", &result);
    assert!(result.is_ok());
    ctx
}

#[test]
fn test_channel_handshake_start_on_archway() -> TestContext {
    let mut ctx = test_conn_handshake_start_on_archway();
    let signed_headers: Vec<RawPayload> = load_a2i_raw_messages();

    let result = call_channel_open_init(
        &mut ctx,
        HexString::from_str(signed_headers[3].message.clone().as_str()).unwrap(),
    );

    println!("channel open init ::> {:?}", &result);
    assert!(result.is_ok());

    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[4].update.clone().unwrap().as_str()).unwrap(),
    );
    println!("call_update client {:?}", &result);
    assert!(result.is_ok());

    let result = call_channel_open_ack(
        &mut ctx,
        HexString::from_str(signed_headers[4].message.clone().as_str()).unwrap(),
    );

    println!("channel open ack ::> {:?}", &result);
    assert!(result.is_ok());

    ctx
}

#[test]
fn test_connection_open_init() -> TestContext {
    // complete handshake
    let mut ctx = setup_test();
    let port_name = "mock";
    call_bind_port(&mut ctx, port_name.clone()).unwrap();
    call_register_client_type(&mut ctx).unwrap();
    let res = query_get_capability(&ctx.app, port_name.to_string(), ctx.get_ibc_core());

    println!("mock app address {res:?}");

    let signed_headers: Vec<RawPayload> = load_raw_messages();
    let response = call_create_client(
        &mut ctx,
        HexString::from_str(signed_headers[0].message.as_str()).unwrap(),
    )
    .unwrap();
    let event = get_event(&response, &get_event_name(IbcEventType::CreateClient)).unwrap();
    let _client_id = event.get("client_id").unwrap();
    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[1].update.clone().unwrap().as_str()).unwrap(),
    );
    println!("call_update client {:?}", &result);
    assert!(result.is_ok());

    let _result = call_connection_open_try(
        &mut ctx,
        HexString::from_str(signed_headers[1].message.clone().as_str()).unwrap(),
    );

    println!(" call_update client {:?}", &result);
    assert!(result.is_ok());

    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[2].update.clone().unwrap().as_str()).unwrap(),
    );
    println!(" call_update client {:?}", &result);
    assert!(result.is_ok());

    let result = call_connection_open_confirm(
        &mut ctx,
        HexString::from_str(signed_headers[2].message.clone().as_str()).unwrap(),
    );

    println!("connection_open_confirm ::> {:?}", &result);
    assert!(result.is_ok());

    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[3].update.clone().unwrap().as_str()).unwrap(),
    );
    println!("{:?}", &result);
    assert!(result.is_ok());

    let result = call_channel_open_try(
        &mut ctx,
        HexString::from_str(signed_headers[3].message.clone().as_str()).unwrap(),
    );

    println!("channel_open_try ::> {:?}", &result);
    assert!(result.is_ok());

    let result = call_update_client(
        &mut ctx,
        HexString::from_str(signed_headers[4].update.clone().unwrap().as_str()).unwrap(),
    );
    println!("{:?}", &result);
    assert!(result.is_ok());

    let result = call_channel_open_confirm(
        &mut ctx,
        HexString::from_str(signed_headers[4].message.clone().as_str()).unwrap(),
    );

    println!("channel_open_confirm ::> {:?}", &result);
    assert!(result.is_ok());
    ctx
}

impl Termination for TestContext {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::SUCCESS
    }
}
