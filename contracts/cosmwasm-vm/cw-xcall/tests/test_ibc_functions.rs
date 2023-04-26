pub mod setup;
use cosmwasm_std::{
    testing::mock_env, to_binary, Addr, Binary, IbcAcknowledgement, IbcChannel,
    IbcChannelConnectMsg::OpenAck, IbcChannelOpenMsg::OpenInit, IbcChannelOpenMsg::OpenTry,
    IbcEndpoint, IbcPacket, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcTimeout, IbcTimeoutBlock,
};
use cw_common::types::{Ack, Address};
use cw_common::xcall_msg::ExecuteMsg;
use cw_xcall::{
    state::CwCallService,
    types::{message::CallServiceMessage, request::CallServiceMessageRequest},
};
use setup::*;
pub mod account;
use account::alice;

#[test]
#[should_panic(expected = "OrderedChannel")]
#[cfg(not(feature = "native_ibc"))]
fn fails_on_open_channel_open_init_ordered_channel() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let mut contract = CwCallService::default();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let execute_msg = ExecuteMsg::IbcChannelOpen {
        msg: OpenInit {
            channel: IbcChannel::new(
                src,
                dst,
                cosmwasm_std::IbcOrder::Ordered,
                "xcall-1",
                "newconnection",
            ),
        },
    };

    contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_msg)
        .unwrap();
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn success_on_open_channel_open_init_unordered_channel() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let mut contract = CwCallService::default();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let execute_msg = ExecuteMsg::IbcChannelOpen {
        msg: OpenInit {
            channel: IbcChannel::new(
                src,
                dst,
                cosmwasm_std::IbcOrder::Unordered,
                "xcall-1",
                "newconnection",
            ),
        },
    };

    let result = contract.execute(deps.as_mut(), mock_env, mock_info, execute_msg);

    assert!(result.is_ok())
}

#[test]
#[cfg(not(feature = "native_ibc"))]
#[should_panic(expected = " InvalidVersion { actual: \"xyz\", expected: \"xcall-1\" }")]
fn fails_on_open_channel_open_try_invalid_version() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let mut contract = CwCallService::default();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let execute_msg = ExecuteMsg::IbcChannelOpen {
        msg: OpenTry {
            channel: IbcChannel::new(
                src,
                dst,
                cosmwasm_std::IbcOrder::Unordered,
                "xcall-1",
                "newconnection",
            ),
            counterparty_version: "xyz".to_owned(),
        },
    };

    contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_msg)
        .unwrap();
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn sucess_on_open_channel_open_try_valid_version() {
    use cosmwasm_std::from_binary;

    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let mut contract = CwCallService::default();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let execute_message = ExecuteMsg::IbcChannelOpen {
        msg: OpenTry {
            channel: IbcChannel::new(
                src.clone(),
                dst,
                cosmwasm_std::IbcOrder::Unordered,
                "xcall-1",
                "newconnection",
            ),
            counterparty_version: "xcall-1".to_owned(),
        },
    };

    let result = contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_message)
        .unwrap();

    let result_data: IbcEndpoint = from_binary(&result.data.unwrap()).unwrap();
    assert_eq!(src.channel_id, result_data.channel_id);

    assert_eq!("xcall-1", result.attributes[1].value)
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn sucess_on_ibc_channel_connect() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let mut contract = CwCallService::default();
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let execute_message = ExecuteMsg::IbcChannelConnect {
        msg: OpenAck {
            channel: IbcChannel::new(
                src.clone(),
                dst,
                cosmwasm_std::IbcOrder::Unordered,
                "xcall-1",
                "newconnection",
            ),
            counterparty_version: "xcall-1".to_owned(),
        },
    };

    let result = contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_message)
        .unwrap();

    assert_eq!("on_channel_connect", result.attributes[0].value);

    let ibc_config = contract.ibc_config().load(deps.as_ref().storage).unwrap();

    assert_eq!(ibc_config.src_endpoint().port_id, src.port_id.as_str())
}

#[test]
#[cfg(not(feature = "native_ibc"))]
#[should_panic(expected = "OrderedChannel")]
fn fails_on_ibc_channel_connect_ordered_channel() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let mut contract = CwCallService::default();
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let execute_message = ExecuteMsg::IbcChannelConnect {
        msg: OpenAck {
            channel: IbcChannel::new(
                src.clone(),
                dst,
                cosmwasm_std::IbcOrder::Ordered,
                "xcall-1",
                "newconnection",
            ),
            counterparty_version: "xcall-1".to_owned(),
        },
    };

    contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_message)
        .unwrap();
}

#[test]
#[cfg(not(feature = "native_ibc"))]
#[should_panic(expected = " InvalidVersion { actual: \"xyz-1\", expected: \"xcall-1\" }")]
fn fails_on_ibc_channel_connect_invalid_counterparty_version() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let mut contract = CwCallService::default();
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let execute_message = ExecuteMsg::IbcChannelConnect {
        msg: OpenAck {
            channel: IbcChannel::new(
                src.clone(),
                dst,
                cosmwasm_std::IbcOrder::Unordered,
                "xcall-1",
                "newconnection",
            ),
            counterparty_version: "xyz-1".to_owned(),
        },
    };

    contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_message)
        .unwrap();
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn sucess_receive_packet_for_call_message_request() {
    let mut mock_deps = deps();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwCallService::default();

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

    let execute_message = ExecuteMsg::IbcPacketReceive {
        msg: packet_message,
    };

    let result = contract.execute(mock_deps.as_mut(), mock_env, mock_info, execute_message);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.events[0].ty, "call_message".to_string())
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn sucess_on_ack_packet() {
    let mut mock_deps = deps();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwCallService::default();
    let ack = IbcAcknowledgement::new(
        to_binary(&Ack::Result(
            Binary::from_base64("aGVsbG8gd29ybGQ=").unwrap(),
        ))
        .unwrap(),
    );

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

    let data = CallServiceMessageRequest::new(
        Address::from(mock_info.sender.as_str()),
        alice().to_string(),
        1,
        vec![],
        vec![1, 2, 3],
    );

    let message: CallServiceMessage = data.try_into().unwrap();

    let packet = IbcPacket::new(to_binary(&message).unwrap(), src, dst, 0, timeout);

    let ack_packet = IbcPacketAckMsg::new(ack, packet, Addr::unchecked("relayer"));

    let execute_message = ExecuteMsg::IbcPacketAck { msg: ack_packet };

    let result = contract
        .execute(mock_deps.as_mut(), mock_env, mock_info, execute_message)
        .unwrap();

    assert_eq!("call_service_request", result.attributes[2].value)
}
