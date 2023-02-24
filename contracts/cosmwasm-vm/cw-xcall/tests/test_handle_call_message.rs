use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info},
    Binary, Coin, CosmosMsg, IbcEndpoint, IbcMsg, Reply, SubMsgResponse, SubMsgResult, WasmMsg,
};
use cw_xcall::{
    state::{CwCallservice, IbcConfig, EXECUTE_CALL, EXECUTE_ROLLBACK},
    types::{
        address::Address,
        message::{CallServiceMessage, CallServiceMessageType},
        request::CallServiceMessageRequest,
        response::{CallServiceMessageReponse, CallServiceResponseType},
    },
};
mod account;
mod setup;

use schemars::_serde_json::to_string;
use setup::*;

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
        Binary::from(vec![104, 101, 108, 108, 111]),
    );
    cw_callservice
        .insert_request(deps.as_mut().storage, request_id, proxy_reqs)
        .unwrap();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let ibc_config = IbcConfig::new(src, dst);

    cw_callservice
        .ibc_config()
        .save(deps.as_mut().storage, &ibc_config)
        .unwrap();

    let res = cw_callservice
        .execute_call( deps.as_mut(), info.clone(), request_id)
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

            assert_eq!("\"aGVsbG8=\"", to_string(msg).unwrap())
        }
        _ => {}
    }
}

#[test]
fn test_successful_reply_message() {
    let mut mock_deps = deps();

    let env = mock_env();

    let msg = Reply {
        id: EXECUTE_CALL,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: None,
        }),
    };

    let contract = CwCallservice::default();

    let request_id = 123456;
    let proxy_reqs = CallServiceMessageRequest::new(
        Address::from(" 88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4"),
        "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7".to_owned(),
        123,
        Binary::from(vec![]),
        Binary::from(vec![]),
    );
    contract
        .insert_request(mock_deps.as_mut().storage, request_id, proxy_reqs)
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
        id: EXECUTE_CALL,
        result: SubMsgResult::Err("error message".into()),
    };

    let contract = CwCallservice::default();

    let request_id = 123456;
    let proxy_reqs = CallServiceMessageRequest::new(
        Address::from(" 88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4"),
        "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7".to_owned(),
        123,
        Binary::from(vec![]),
        Binary::from(vec![]),
    );
    contract
        .insert_request(mock_deps.as_mut().storage, request_id, proxy_reqs)
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
        id: EXECUTE_CALL,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: None,
        }),
    };

    let contract = CwCallservice::default();

    let request_id = 123456;
    let proxy_reqs = CallServiceMessageRequest::new(
        Address::from(" 88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4"),
        "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7".to_owned(),
        123,
        Binary::from(vec![1, 2, 3]),
        Binary::from(vec![]),
    );
    contract
        .insert_request(mock_deps.as_mut().storage, request_id, proxy_reqs)
        .unwrap();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &request_id)
        .unwrap();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let ibc_config = IbcConfig::new(src, dst);

    contract
        .ibc_config()
        .save(mock_deps.as_mut().storage, &ibc_config)
        .unwrap();

    let response = contract.reply(mock_deps.as_mut(), env, msg).unwrap();

    match response.messages[0].msg.clone() {
        CosmosMsg::Ibc(IbcMsg::SendPacket {
            channel_id: _,
            data,
            timeout: _,
        }) => {
            let result: CallServiceMessage = from_binary(&data).unwrap();
            assert_eq!(
                result.message_type().to_owned(),
                CallServiceMessageType::CallServiceResponse
            );

            let payload: CallServiceMessageReponse =
                from_binary(&Binary::from(result.payload().to_vec())).unwrap();

            assert_eq!(
                payload.response_code().to_owned(),
                CallServiceResponseType::CallServiceResponseSucess
            )
        }

        _ => todo!(),
    };
}
