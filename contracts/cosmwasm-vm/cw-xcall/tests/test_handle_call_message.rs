use cosmwasm_std::{
    coin,
    testing::{mock_dependencies, mock_env, mock_info},
    to_binary, Binary, Coin, CosmosMsg, Event, Reply, SubMsgResponse, SubMsgResult, WasmMsg,
};
use cw_xcall::{
    error::ContractError,
    handle_callmessage::reply,
    state::CwCallservice,
    types::{address::Address, request::CallServiceMessageRequest},
};

const EXECUTE_CALL: u64 = 0;
#[test]
#[should_panic(expected = "InvalidRequestId")]
fn test_execute_call_invalid_request_id() {
    let cw_callservice = CwCallservice::new();
    let env = mock_env();
    let info = mock_info("user", &[Coin::new(1000, "ucosm")]);
    let mut deps = mock_dependencies();

    cw_callservice
        .contains_request(&deps.storage, 123456)
        .unwrap();
}

#[test]
fn test_execute_call_having_request_id_without_rollback() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("user1", &[Coin::new(1000, "ucosm")]);
    let cw_callservice = CwCallservice::default();

    let request_id = 123456;
    let proxy_reqs = CallServiceMessageRequest::new(
        Address::from(" 88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4"),
        "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7".to_owned(),
        123,
        Binary::from(vec![]),
        Binary::from(vec![]),
    );
    cw_callservice
        .insert_request(deps.as_mut().storage, request_id, proxy_reqs)
        .unwrap();

    let res = cw_callservice.execute_call(env.clone(), deps.as_mut(), info.clone(), request_id);

    assert!(res.is_ok());
    let response = res.unwrap();
    assert_eq!(response.messages.len(), 1);
    match &response.messages[0].msg {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr,
            msg: _,
            funds: _,
        }) => {
            assert_eq!(
                contract_addr,
                "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7"
            );
        }
        _ => panic!("Unexpected message type"),
    }
    assert_eq!(response.attributes.len(), 1);
    assert_eq!(response.attributes[0].key, "call_message");
    assert_eq!(response.attributes[0].value, "execute_call");
}

#[test]
fn test_successful_reply_message() {
    let event = Event::new("callexecuted")
        .add_attribute("request_id", 2.to_string())
        .add_attribute("code", "code".to_string())
        .add_attribute("msg", "msg".to_string());
    let msg = Reply {
        id: EXECUTE_CALL,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: None,
        }),
    };
    let env = mock_env();
    let mut deps = mock_dependencies();
    let res = reply(deps.as_mut(), env, msg).unwrap();

    assert_eq!(1, res.attributes.len());
    assert_eq!("call_message", res.attributes[0].key);
    assert_eq!(1, res.events.len());
}

#[test]
fn test_failed_reply_message() {
    let msg = Reply {
        id: EXECUTE_CALL,
        result: SubMsgResult::Err("error message".into()),
    };
    let env = mock_env();
    let mut deps = mock_dependencies();
    let res = reply(deps.as_mut(), env, msg);

    assert!(res.is_err());
    assert_eq!(
        ContractError::ReplyError {
            code: 1,
            msg: "error message".into()
        }
        .to_string(),
        res.unwrap_err().to_string()
    );
}
