mod account;
mod setup;

use account::*;
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env},
    Addr, IbcEndpoint, IbcPacket, IbcPacketReceiveMsg, IbcTimeout, IbcTimeoutBlock,
};
use cw_xcall::{
    ibc::ibc_packet_receive,
    state::CwCallService,
    types::{
        address::Address, call_request::CallRequest, message::CallServiceMessage,
        request::CallServiceMessageRequest, response::CallServiceMessageReponse,
    },
};
use setup::*;

#[test]
fn test_receive_packet_for_call_message_request() {
    let mut mock_deps = mock_dependencies();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    let data = CallServiceMessageRequest::new(
        Address::from(mock_info.sender.as_str()),
        alice().to_string(),
        1,
        vec![],
        vec![1, 2, 3],
    );

    let message: CallServiceMessage = data.try_into().unwrap();

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 0,
    };
    let timeout = IbcTimeout::with_block(timeout_block);
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };
    let packet = IbcPacket::new(message, src, dst, 0, timeout);
    let packet_message = IbcPacketReceiveMsg::new(packet, Addr::unchecked("relay"));

    let result = ibc_packet_receive(mock_deps.as_mut(), mock_env, packet_message);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.events[0].ty, "call_message".to_string())
}

#[test]
fn test_receive_packet_for_call_message_response() {
    let mut mock_deps = mock_dependencies();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    let data = CallServiceMessageReponse::new(
        1,
        cw_xcall::types::response::CallServiceResponseType::CallServiceResponseSucess,
        "",
    );

    let call_request = CallRequest::new(alice(), bob().to_string(), vec![1, 2, 3].into(), true);

    contract
        .set_call_request(mock_deps.as_mut().storage, 1, call_request)
        .unwrap();

    let message: CallServiceMessage = data.try_into().unwrap();

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 0,
    };
    let timeout = IbcTimeout::with_block(timeout_block);
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };
    let packet = IbcPacket::new(message, src, dst, 0, timeout);

    let packet_message = IbcPacketReceiveMsg::new(packet, Addr::unchecked("relay"));

    let result = ibc_packet_receive(mock_deps.as_mut(), mock_env, packet_message);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.events[0].ty, "response_message".to_string())
}

#[test]
fn receive_packet_for_call_message_response_invalid_sequence_id() {
    let mut mock_deps = mock_dependencies();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    let data = CallServiceMessageReponse::new(
        1,
        cw_xcall::types::response::CallServiceResponseType::CallServiceResponseSucess,
        "",
    );

    let message: CallServiceMessage = data.try_into().unwrap();

    let call_request = CallRequest::new(alice(), bob().to_string(), vec![1, 2, 3].into(), true);

    contract
        .set_call_request(mock_deps.as_mut().storage, 2, call_request)
        .unwrap();

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 0,
    };
    let timeout = IbcTimeout::with_block(timeout_block);
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };
    let packet = IbcPacket::new(message, src, dst, 0, timeout);

    let packet_message = IbcPacketReceiveMsg::new(packet, Addr::unchecked("relay"));

    let result = ibc_packet_receive(mock_deps.as_mut(), mock_env, packet_message).unwrap();

    assert_eq!(result.attributes[1].value, "InvalidSequenceId 1")
}

#[test]
fn handle_response_emit_rollback_event() {
    let mut mock_deps = mock_dependencies();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();
    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    let data = CallServiceMessageReponse::new(
        1,
        cw_xcall::types::response::CallServiceResponseType::CallServiceResponseFailure,
        "",
    );

    let call_request = CallRequest::new(alice(), bob().to_string(), vec![1, 2, 3].into(), false);

    contract
        .set_call_request(mock_deps.as_mut().storage, 1, call_request)
        .unwrap();

    let message: CallServiceMessage = data.try_into().unwrap();

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 0,
    };
    let timeout = IbcTimeout::with_block(timeout_block);
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };
    let packet = IbcPacket::new(message, src, dst, 0, timeout);

    let packet_message = IbcPacketReceiveMsg::new(packet, Addr::unchecked("relay"));

    let result = ibc_packet_receive(mock_deps.as_mut(), mock_env, packet_message);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.events[0].ty, "rollback_message".to_string());

    let call_request = contract
        .query_request(mock_deps.as_mut().storage, 1)
        .unwrap();

    assert_eq!(call_request.enabled(), true)
}
