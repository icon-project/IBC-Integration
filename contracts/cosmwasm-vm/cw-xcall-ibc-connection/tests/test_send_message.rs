pub mod setup;
use std::str::FromStr;

use cosmwasm_std::{
    testing::mock_env, to_binary, Addr,IbcEndpoint, IbcPacket, IbcTimeout, IbcTimeoutBlock,
};


use cw_xcall_ibc_connection::types::config::Config;
use cw_xcall_ibc_connection::types::network_fees::NetworkFees;
use cw_xcall_lib::network_address::NetId;

use cw_xcall_ibc_connection::types::channel_config::ChannelConfig;
use setup::*;
pub mod account;
use account::alice;

use cosmwasm_std::{ WasmQuery, SystemResult, ContractResult, OwnedDeps, Env};
use cw_common::xcall_connection_msg::ExecuteMsg;

use cw_xcall_ibc_connection::state::{CwIbcConnection, IbcConfig};


fn send_message_setup() -> (CwIbcConnection<'static>, OwnedDeps<cosmwasm_std::MemoryStorage, cosmwasm_std::testing::MockApi, cosmwasm_std::testing::MockQuerier>, Env,NetId) {
    let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, cosmwasm_std::testing::MockApi, cosmwasm_std::testing::MockQuerier> = deps();

    let mock_env = mock_env();

    let contract = CwIbcConnection::default();
    let nid = NetId::from_str("nid").unwrap();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let channel_config = ChannelConfig {
        client_id: "client_id".to_string(),
        timeout_height: 100,
        counterparty_nid: NetId::from("nid".to_string()),
    };
    contract
        .store_ibc_config(deps.as_mut().storage,&nid.clone(), &IbcConfig::new(src.clone(), dst.clone()))
        .unwrap();

    contract.set_ibc_host(deps.as_mut().storage, Addr::unchecked(alice().as_str())).unwrap();

    contract
    .store_channel_config(deps.as_mut().storage, &src.channel_id, &channel_config)
    .unwrap();
    deps.querier.update_wasm(|r| match r {
        WasmQuery::Smart {
            contract_addr: _,
            msg: _,
        } => SystemResult::Ok(ContractResult::Ok(
            to_binary(&10).unwrap(),
        )),
        _ => todo!(),
    });

    contract.store_network_fees(deps.as_mut().storage, nid.clone(), &NetworkFees{
        send_packet_fee: 10,
        ack_fee: 10,
    }).unwrap();

    contract.store_config(deps.as_mut().storage, &Config { port_id: "our_port".to_owned(), denom: "abcd".to_owned() }).unwrap();
    
    (contract, deps, mock_env, nid)
}

#[test]
fn send_message_success_case() {
    let (mut contract, mut deps, mock_env, nid) = send_message_setup();
    let execute_msg = ExecuteMsg::SendMessage { to: nid, sn: 1, msg: vec![] };

    let mock_info = create_mock_info("alice", "abcd", 20);

    contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "InsufficientFunds")]
fn send_message_with_less_fund() {
    let (mut contract, mut deps, mock_env, nid) = send_message_setup();
    let execute_msg = ExecuteMsg::SendMessage { to: nid, sn: 1, msg: vec![] };

    let mock_info = create_mock_info("alice", "abcd", 10);

    contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_msg)
        .unwrap();
}

#[test]
fn send_message_with_negative_sn() {
    let (mut contract, mut deps, mock_env, nid) = send_message_setup();
    let execute_msg = ExecuteMsg::SendMessage { to: nid, sn: -1, msg: vec![] };
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 0,
    };
    let timeout = IbcTimeout::with_block(timeout_block);

    let packet = IbcPacket::new(vec![], src.clone(), dst.clone(), 0, timeout);

    contract.store_incoming_packet(deps.as_mut().storage, &src.channel_id, 1, packet).unwrap();

    let mock_info = create_mock_info("alice", "abcd", 20);

    contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_msg)
        .unwrap();
}
