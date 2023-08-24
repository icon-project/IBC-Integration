mod setup;
use std::{process::Termination, str::FromStr};

use anyhow::Error as AppError;
use common::constants::ICON_CLIENT_TYPE;
use common::ibc::events::IbcEventType;

use cosmwasm_std::{from_binary, to_binary, Addr, Empty, Querier, QueryRequest};

use cw_common::{core_msg as CoreMsg, hex_string::HexString, query_helpers::build_smart_query};

use cw_integration::TestSteps;
use cw_multi_test::{App, AppResponse, Executor};

use cw_xcall_lib::network_address::{NetId, NetworkAddress};
use setup::{
    init_ibc_core_contract, init_light_client, init_mock_dapp_multi_contract,
    init_xcall_app_contract, init_xcall_ibc_connection_contract, setup_context, TestContext,
    COUNTERPARTY_NID, PORT,
};
use test_utils::{get_event, get_event_name, load_raw_payloads};

fn setup_test(payload_file: &str) -> TestContext {
    let integration_data = load_raw_payloads(payload_file);

    let mut context = setup_context(Some(integration_data));
    context = setup_xcall_multi_contracts(context);
    context
}

pub fn setup_xcall_multi_contracts(mut ctx: TestContext) -> TestContext {
    ctx = init_ibc_core_contract(ctx);
    ctx = init_light_client(ctx);
    ctx = init_xcall_app_contract(ctx);
    ctx = init_xcall_ibc_connection_contract(ctx);
    ctx = init_mock_dapp_multi_contract(ctx);
    ctx
}

pub fn call_multi_dapp_send_message(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_dapp(),
        &cw_mock_dapp_multi::msg::ExecuteMsg::SendCallMessage {
            to: NetworkAddress::new(COUNTERPARTY_NID, "someaddress"),
            data: vec![72, 101, 108, 108, 111],
            rollback: None,
        },
        &[],
    )
}

pub fn call_multi_dapp_add_connection(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_dapp(),
        &cw_mock_dapp_multi::msg::ExecuteMsg::AddConnection {
            src_endpoint: ctx.get_xcall_ibc_connection().to_string(),
            dest_endpoint: "cx00000".to_string(),
            network_id: COUNTERPARTY_NID.to_string(),
        },
        &[],
    )
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

pub fn call_create_client(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::CreateClient);
    let msg = HexString::from_str(&payload.message).unwrap();

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

pub fn call_connection_open_init(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ConnectionOpenInit);
    let msg = HexString::from_str(&payload.message).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ConnectionOpenInit { msg },
        &[],
    )
}

pub fn call_connection_open_try(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ConnectionOpenTry);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ConnectionOpenTry { msg },
        &[],
    )
}

pub fn call_connection_open_ack(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ConnectionOpenAck);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ConnectionOpenAck { msg },
        &[],
    )
}

pub fn call_connection_open_confirm(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ConnectionOpenConfirm);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ConnectionOpenConfirm { msg },
        &[],
    )
}

pub fn call_channel_open_init(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ChannelOpenInit);
    let msg = HexString::from_str(&payload.message).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ChannelOpenInit { msg },
        &[],
    )
}

pub fn call_channel_open_try(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ChannelOpenTry);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ChannelOpenTry { msg },
        &[],
    )
}

pub fn call_channel_open_ack(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ChannelOpenAck);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ChannelOpenAck { msg },
        &[],
    )
}

pub fn call_channel_close_init(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ChannelCloseInit);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ChannelCloseInit { msg },
        &[],
    )
}

pub fn call_channel_close_confirm(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ChannelCloseConfirm);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ChannelCloseConfirm { msg },
        &[],
    )
}

pub fn call_channel_open_confirm(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ChannelOpenConfirm);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ChannelOpenConfirm { msg },
        &[],
    )
}

pub fn call_receive_packet(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::ReceivePacket);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::ReceivePacket { msg },
        &[],
    )
}

fn call_bind_port(
    ctx: &mut TestContext,
    port_name: &str,
    addr: &str,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::BindPort {
            port_id: port_name.to_string(),
            address: addr.to_string(),
        },
        &[],
    )
}

fn call_xcall_app_message(ctx: &mut TestContext, data: Vec<u8>) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        Addr::unchecked(ctx.caller.as_ref().cloned().unwrap()),
        ctx.get_xcall_app(),
        &cw_xcall_lib::xcall_msg::ExecuteMsg::SendCallMessage {
            to: NetworkAddress::new(
                COUNTERPARTY_NID,
                "cx284306db853ba518220b7e553a710ddb12575605",
            ),
            sources: Some(vec![]),
            destinations: Some(vec![]),
            data,
            rollback: None,
        },
        &[],
    )
}

pub fn call_acknowledge_packet(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::AcknowledgementPacket);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::AcknowledgementPacket { msg },
        &[],
    )
}

pub fn call_timeout_packet(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    let payload = ctx.get_test_data(&TestSteps::TimeoutPacket);
    let msg = HexString::from_str(&payload.message).unwrap();
    let update = HexString::from_str(&payload.update.unwrap()).unwrap();
    call_update_client(ctx, update).unwrap();
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_ibc_core(),
        &CoreMsg::ExecuteMsg::TimeoutPacket { msg },
        &[],
    )
}

pub fn query_get_capability(app: &App, port_id: String, contract_address: Addr) -> String {
    let query = cw_common::core_msg::QueryMsg::GetCapability { name: port_id };
    let query: QueryRequest<Empty> =
        build_smart_query(contract_address.to_string(), to_binary(&query).unwrap());

    let balance = app.raw_query(&to_binary(&query).unwrap()).unwrap().unwrap();
    let res: String = from_binary(&balance).unwrap();
    res
}

pub fn call_set_xcall_host(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_xcall_ibc_connection(),
        &cw_common::xcall_connection_msg::ExecuteMsg::SetXCallHost {
            address: ctx.get_xcall_app().to_string(),
        },
        &[],
    )
}

pub fn call_configure_connection(
    ctx: &mut TestContext,
    connection_id: String,
    nid: String,
    client_id: String,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_xcall_ibc_connection(),
        &cw_common::xcall_connection_msg::ExecuteMsg::ConfigureConnection {
            connection_id,
            counterparty_port_id: PORT.to_string(),
            counterparty_nid: NetId::from(nid),
            client_id,
            timeout_height: 10,
        },
        &[],
    )
}

pub fn call_set_default_connection(
    ctx: &mut TestContext,
    nid: String,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_xcall_app(),
        &cw_xcall_lib::xcall_msg::ExecuteMsg::SetDefaultConnection {
            nid: NetId::from(nid),
            address: ctx.get_xcall_ibc_connection(),
        },
        &[],
    )
}

#[test]
fn test_register_client() {
    let mut ctx = setup_test("icon_to_archway_raw.json");
    let result = call_register_client_type(&mut ctx);
    assert!(result.is_ok());
}

#[test]
fn test_create_client() {
    let mut ctx = setup_test("icon_to_archway_raw.json");
    call_register_client_type(&mut ctx).unwrap();
    let result = call_create_client(&mut ctx);
    assert!(result.is_ok());
}
#[test]
fn test_update_client() {
    let mut ctx = setup_test("icon_to_archway_raw.json");
    call_register_client_type(&mut ctx).unwrap();
    let response = call_create_client(&mut ctx).unwrap();
    let event = get_event(&response, &get_event_name(IbcEventType::CreateClient)).unwrap();
    let _client_id = event.get("client_id").unwrap();
    let payload = ctx.get_test_data(&TestSteps::ConnectionOpenTry);
    let result = call_update_client(
        &mut ctx,
        HexString::from_str(payload.update.unwrap().as_str()).unwrap(),
    );

    assert!(result.is_ok());
}

#[test]
fn test_packet_receiver() {
    let mut ctx = test_icon_to_arcway_handshake();

    let result = call_receive_packet(&mut ctx);

    assert!(result.is_ok());
}

#[test]
fn test_packet_send_multi_dapp() {
    let mut ctx = test_icon_to_arcway_handshake();
    call_multi_dapp_add_connection(&mut ctx).unwrap();

    let result = call_multi_dapp_send_message(&mut ctx);

    ctx.list_contracts();
    assert!(result.is_ok());
}

#[test]
fn test_packet_send() {
    let mut ctx = test_archway_to_icon_handshake();

    let data = [123, 100, 95, 112, 97];
    let result = call_xcall_app_message(&mut ctx, data.into());
    assert!(result.is_ok());
}

pub fn get_client_id(res: &AppResponse) -> String {
    let event = get_event(res, &get_event_name(IbcEventType::CreateClient)).unwrap();
    let client_id = event.get("client_id").unwrap().to_string();
    client_id
}

pub fn get_connection_id(res: &AppResponse, event: IbcEventType) -> String {
    let event = get_event(res, &get_event_name(event)).unwrap();
    let connection_id = event.get("connection_id").unwrap().to_string();
    connection_id
}

#[test]
fn test_icon_to_arcway_handshake() -> TestContext {
    let mut ctx = setup_test("icon_to_archway_raw.json");
    let port_name = PORT;
    let module_address = ctx.get_xcall_ibc_connection().to_string();
    call_bind_port(&mut ctx, port_name.clone(), &module_address).unwrap();
    call_register_client_type(&mut ctx).unwrap();
    call_set_xcall_host(&mut ctx).unwrap();

    let res = query_get_capability(&ctx.app, port_name.to_string(), ctx.get_ibc_core());

    println!("mock app address {res:?}");

    let response = call_create_client(&mut ctx);

    assert!(response.is_ok());
    println!("Create Client OK");
    let client_id = get_client_id(&response.unwrap());
    println!("Clientid is {}", client_id);

    let result = call_connection_open_try(&mut ctx);
    println!("Conn Open Try Ok {:?}", &result);
    assert!(result.is_ok());

    let result = call_connection_open_confirm(&mut ctx);

    assert!(result.is_ok());
    println!("Conn Open Confirm Ok {:?}", &result);
    // now need to setup connection configuration for multi call

    let connection_id = get_connection_id(&result.unwrap(), IbcEventType::OpenConfirmConnection);
    let nid = "0x3.icon".to_string();

    let result = call_configure_connection(&mut ctx, connection_id, nid.clone(), client_id);

    assert!(result.is_ok());
    println!("Configure Connection Ok {:?}", &result);

    let result = call_set_default_connection(&mut ctx, nid);

    assert!(result.is_ok());
    println!("Set Default Connection Ok {:?}", &result);

    let result = call_channel_open_try(&mut ctx);

    assert!(result.is_ok());
    println!("Channel Open Try Ok{:?}", &result);

    let result = call_channel_open_confirm(&mut ctx);
    assert!(result.is_ok());

    println!("Channel Open Confirm Ok {:?}", &result);
    ctx
}

#[test]
fn test_archway_to_icon_handshake() -> TestContext {
    // complete handshake
    let mut ctx = setup_test("archway_to_icon_raw.json");
    let port_name = PORT;
    let module_address = ctx.get_xcall_ibc_connection().to_string();
    call_bind_port(&mut ctx, port_name.clone(), &module_address).unwrap();
    call_register_client_type(&mut ctx).unwrap();
    let res = query_get_capability(&ctx.app, port_name.to_string(), ctx.get_ibc_core());

    println!("mock app address {res:?}");

    let response = call_create_client(&mut ctx);

    assert!(response.is_ok());
    println!("Create Client OK");
    let client_id = get_client_id(&response.unwrap());
    println!("Clientid is {}", client_id);

    let result = call_connection_open_init(&mut ctx);

    assert!(result.is_ok());
    println!("Conn Open init Ok {:?}", &result);

    let result = call_connection_open_ack(&mut ctx);

    assert!(result.is_ok());
    println!("Conn Open ack Ok {:?}", &result);

    let connection_id = get_connection_id(&result.unwrap(), IbcEventType::OpenAckConnection);
    let nid = COUNTERPARTY_NID.to_string();

    let result = call_configure_connection(&mut ctx, connection_id, nid.clone(), client_id);
    assert!(result.is_ok());
    println!("Configure Connection Ok {:?}", &result);

    let result = call_set_default_connection(&mut ctx, nid);

    assert!(result.is_ok());
    println!("Set Default Connection Ok {:?}", &result);

    let result = call_channel_open_init(&mut ctx);
    println!("{result:?}");

    assert!(result.is_ok());
    println!("Channel Open init Ok{:?}", &result);

    let result = call_channel_open_ack(&mut ctx);
    assert!(result.is_ok());

    // println!("Channel Open ack Ok {:?}", &result);
    ctx
}

impl Termination for TestContext {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::SUCCESS
    }
}
