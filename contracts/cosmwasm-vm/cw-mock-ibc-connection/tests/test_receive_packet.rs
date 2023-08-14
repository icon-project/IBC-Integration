mod account;
mod setup;

use account::*;

use cosmwasm_std::{
    testing::{mock_dependencies, mock_env},
    Addr, Binary, IbcEndpoint, IbcPacket, IbcPacketReceiveMsg, IbcTimeout, IbcTimeoutBlock,
};

use cw_xcall::types::{
    message::CallServiceMessage,
    request::CallServiceMessageRequest,
    response::CallServiceMessageResponse,
    rlp::{self},
};
use cw_mock_ibc_connection::{
    ibc::ibc_packet_receive, state::CwIbcConnection, types::message::Message,
};
use cw_xcall_lib::network_address::NetworkAddress;
use setup::*;

#[test]
fn test_receive_packet_for_call_message_request() {
    let mut mock_deps = mock_dependencies();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .set_xcall_host(mock_deps.as_mut().storage, Addr::unchecked("xcallhost"))
        .unwrap();

    let data = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked(alice().to_string()),
        1,
        false,
        vec![1, 2, 3],
        vec![],
    );

    let message: CallServiceMessage = data.try_into().unwrap();
    let message: Message = Message {
        sn: common::rlp::Nullable::new(Some(0)),
        fee: 0,
        data: rlp::encode(&message).to_vec(),
    };
    let message_data = Binary(common::rlp::encode(&message).to_vec());

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
    let packet = IbcPacket::new(message_data, src, dst, 0, timeout);
    let packet_message = IbcPacketReceiveMsg::new(packet, Addr::unchecked("relay"));

    let result = ibc_packet_receive(mock_deps.as_mut(), mock_env, packet_message);

    assert!(result.is_ok());
}

#[test]
fn test_receive_packet_for_call_message_response() {
    let mut mock_deps = mock_dependencies();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .set_xcall_host(mock_deps.as_mut().storage, Addr::unchecked("xcallhost"))
        .unwrap();
    let data = CallServiceMessageResponse::new(
        1,
        cw_xcall::types::response::CallServiceResponseType::CallServiceResponseSuccess,
    );

    let message: CallServiceMessage = data.try_into().unwrap();
    let message: Message = Message {
        sn: common::rlp::Nullable::new(Some(0)),
        fee: 0,
        data: rlp::encode(&message).to_vec(),
    };
    let message_data = Binary(common::rlp::encode(&message).to_vec());

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
    let packet = IbcPacket::new(message_data, src, dst, 0, timeout);

    let packet_message = IbcPacketReceiveMsg::new(packet, Addr::unchecked("relay"));

    let result = ibc_packet_receive(mock_deps.as_mut(), mock_env, packet_message);

    assert!(result.is_ok());
}
