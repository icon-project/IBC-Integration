use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info},
    Addr, Coin, CosmosMsg, Reply, SubMsgResponse, SubMsgResult, WasmMsg,
};

use cw_common::xcall_types::network_address::NetworkAddress;
use cw_xcall_multi::{
    state::{CwCallService, EXECUTE_CALL_ID, EXECUTE_ROLLBACK_ID},
    types::{call_request::CallRequest, request::CallServiceMessageRequest},
};
mod account;
mod setup;
use crate::account::alice;

use schemars::_serde_json::to_string;
use setup::test::*;

#[test]
#[should_panic(expected = "InvalidRequestId")]
fn test_execute_call_invalid_request_id() {
    let cw_callservice = CwCallService::new();

    let deps = mock_dependencies();

    cw_callservice
        .contains_proxy_request(&deps.storage, 123456)
        .unwrap();
}

#[test]
fn test_execute_call_having_request_id_without_rollback() {
    let mut deps = mock_dependencies();

    let info = mock_info("user1", &[Coin::new(1000, "ucosm")]);
    let cw_callservice = CwCallService::default();

    let request_id = 123456;
    let proxy_requests = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", "mockaddress"),
        Addr::unchecked("88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7"),
        123,
        false,
        vec![104, 101, 108, 108, 111],
        vec![],
    );
    cw_callservice
        .store_proxy_request(deps.as_mut().storage, request_id, &proxy_requests)
        .unwrap();

    let res = cw_callservice
        .execute_call(deps.as_mut(), info, request_id)
        .unwrap();
    match &res.messages[0].msg {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr,
            msg,
            funds: _,
        }) => {
            assert_eq!(
                contract_addr,
                "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7"
            );

            assert_eq!(
                "\"eyJ4X2NhbGxfbWVzc2FnZSI6eyJkYXRhIjpbMTA0LDEwMSwxMDgsMTA4LDExMV19fQ==\"",
                to_string(msg).unwrap()
            )
        }
        _ => {}
    }
}

#[test]
fn test_successful_reply_message() {
    let mut mock_deps = deps();

    let env = mock_env();

    let msg = Reply {
        id: EXECUTE_CALL_ID,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: None,
        }),
    };

    let contract = CwCallService::default();

    let request_id = 123456;
    let proxy_requests = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", "mockaddress"),
        Addr::unchecked("88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7"),
        123,
        false,
        vec![],
        vec![],
    );
    contract
        .store_proxy_request(mock_deps.as_mut().storage, request_id, &proxy_requests)
        .unwrap();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &request_id)
        .unwrap();

    let response = contract.reply(mock_deps.as_mut(), env, msg).unwrap();

    assert_eq!(response.events[0].attributes[1].value, 0.to_string());
}

#[test]
fn test_failed_reply_message() {
    let mut mock_deps = deps();

    let env = mock_env();

    let msg = Reply {
        id: EXECUTE_CALL_ID,
        result: SubMsgResult::Err("error message".into()),
    };

    let contract = CwCallService::default();

    let request_id = 123456;
    let proxy_requests = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", "mockaddress"),
        Addr::unchecked("88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7"),
        123,
        false,
        vec![],
        vec![],
    );
    contract
        .store_proxy_request(mock_deps.as_mut().storage, request_id, &proxy_requests)
        .unwrap();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &request_id)
        .unwrap();

    let response = contract.reply(mock_deps.as_mut(), env, msg).unwrap();

    assert_eq!(response.events[0].attributes[1].value, "-1".to_string());
}

#[test]
fn check_for_rollback_in_response() {
    let mut mock_deps = deps();

    let env = mock_env();

    let msg = Reply {
        id: EXECUTE_ROLLBACK_ID,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: None,
        }),
    };

    let contract = CwCallService::default();

    let seq_id = 123456;

    let request = CallRequest::new(
        Addr::unchecked("88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4"),
        NetworkAddress::new("nid", "mockaddress"),
        vec![],
        vec![1, 2, 3],
        true,
    );

    contract
        .store_call_request(mock_deps.as_mut().storage, seq_id, &request)
        .unwrap();

    contract
        .store_execute_rollback_id(mock_deps.as_mut().storage, seq_id)
        .unwrap();

    let response = contract.reply(mock_deps.as_mut(), env, msg).unwrap();

    assert_eq!("0", response.events[0].attributes[1].value)
}

#[test]
#[should_panic(expected = "InvalidSequenceId { id: 123456 }")]
fn test_invalid_sequence_no() {
    let deps = mock_dependencies();
    let contract = CwCallService::new();
    contract
        .get_proxy_request(deps.as_ref().storage, 123456)
        .unwrap();
}

#[test]
fn check_for_rollback_response_failure() {
    let mut mock_deps = deps();

    let env = mock_env();

    let msg = Reply {
        id: EXECUTE_ROLLBACK_ID,
        result: SubMsgResult::Err("error message".into()),
    };

    let contract = CwCallService::default();

    let seq_id = 123456;

    let request = CallRequest::new(
        Addr::unchecked("88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4"),
        NetworkAddress::new("nid", "mockaddress"),
        vec![],
        vec![],
        false,
    );

    contract
        .store_call_request(mock_deps.as_mut().storage, seq_id, &request)
        .unwrap();

    contract
        .store_execute_rollback_id(mock_deps.as_mut().storage, seq_id)
        .unwrap();

    let response = contract.reply(mock_deps.as_mut(), env, msg).unwrap();

    assert_eq!("-1", response.events[0].attributes[1].value)
}

#[test]
fn execute_rollback_success() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    let seq_id = 123456;

    let request = CallRequest::new(
        Addr::unchecked("88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4"),
        NetworkAddress::new("nid", "mockaddress"),
        vec![],
        vec![1, 2, 3],
        true,
    );

    contract
        .store_call_request(mock_deps.as_mut().storage, seq_id, &request)
        .unwrap();

    contract
        .store_execute_rollback_id(mock_deps.as_mut().storage, seq_id)
        .unwrap();

    let response = contract
        .execute_rollback(mock_deps.as_mut(), mock_env(), mock_info, seq_id)
        .unwrap();

    match response.messages[0].msg.clone() {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: _,
            msg,
            funds: _,
        }) => {
            let data = String::from_utf8(msg.0).unwrap();
            assert_eq!("{\"x_call_message\":{\"data\":[1,2,3]}}", data)
        }
        _ => todo!(),
    }
}

#[test]
#[should_panic(expected = "RollbackNotEnabled")]
fn execute_rollback_failure() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    let seq_id = 123456;

    let request = CallRequest::new(
        Addr::unchecked("88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4"),
        NetworkAddress::new("nid", "mockaddress"),
        vec![],
        vec![],
        false,
    );

    contract
        .store_call_request(mock_deps.as_mut().storage, seq_id, &request)
        .unwrap();

    contract
        .store_execute_rollback_id(mock_deps.as_mut().storage, seq_id)
        .unwrap();

    let response = contract
        .execute_rollback(mock_deps.as_mut(), mock_env(), mock_info, seq_id)
        .unwrap();

    match response.messages[0].msg.clone() {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: _,
            msg,
            funds: _,
        }) => {
            let r: Vec<u64> = from_binary(&msg).unwrap();

            assert_eq!(vec![1, 2, 3], r)
        }
        _ => todo!(),
    }
}
