pub mod setup;
use common::rlp::{self, Nullable};
use cosmwasm_std::{
    testing::mock_env, to_binary, Addr, Binary, IbcAcknowledgement, IbcChannel,
    IbcChannelConnectMsg::OpenAck, IbcChannelOpenMsg::OpenInit, IbcChannelOpenMsg::OpenTry,
    IbcEndpoint, IbcPacket, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcTimeout, IbcTimeoutBlock,
};

use cw_common::from_binary_response;
use cw_common::types::Ack;

use cw_common::xcall_types::network_address::{NetId, NetworkAddress};
use cw_xcall_ibc_connection::msg::InstantiateMsg;
use cw_xcall_ibc_connection::types::channel_config::ChannelConfig;
use cw_xcall_ibc_connection::types::message::Message;
use cw_xcall_ibc_connection::{execute, instantiate, query};
use cw_xcall_multi::ack::{on_ack_failure, on_ack_sucess};
use cw_xcall_multi::types::response::CallServiceMessageResponse;
use setup::*;
pub mod account;
use account::admin_one;
use account::alice;

use cosmwasm_std::from_binary;
use cw_common::xcall_connection_msg::{ExecuteMsg, QueryMsg};

use cw_xcall_ibc_connection::state::CwIbcConnection;
use cw_xcall_multi::types::message::CallServiceMessage;
use cw_xcall_multi::types::request::CallServiceMessageRequest;

#[test]
#[cfg(not(feature = "native_ibc"))]
#[should_panic(expected = "OrderedChannel")]
fn fails_on_open_channel_open_init_ordered_channel() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcConnection::default();

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
        .set_ibc_host(deps.as_mut().storage, Addr::unchecked(alice().as_str()))
        .unwrap();

    contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_msg)
        .unwrap();
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn success_on_open_channel_open_init_unordered_channel() {
    use cw_common::xcall_connection_msg::ExecuteMsg;
    use cw_xcall_ibc_connection::state::CwIbcConnection;

    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcConnection::default();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let connection_id = "newconnection".to_string();

    let execute_msg = ExecuteMsg::IbcChannelOpen {
        msg: OpenInit {
            channel: IbcChannel::new(
                src,
                dst.clone(),
                cosmwasm_std::IbcOrder::Unordered,
                "ics20-1",
                &connection_id,
            ),
        },
    };

    contract
        .configure_connection(
            deps.as_mut().storage,
            connection_id,
            dst.port_id,
            NetId::from("nid".to_string()),
            "client-id".to_string(),
            100,
        )
        .unwrap();
    contract
        .set_ibc_host(deps.as_mut().storage, Addr::unchecked(alice().as_str()))
        .unwrap();

    let result = contract.execute(deps.as_mut(), mock_env, mock_info, execute_msg);
    println!("{:?}", result);

    assert!(result.is_ok())
}

#[test]
#[cfg(not(feature = "native_ibc"))]
#[should_panic(expected = " InvalidVersion { actual: \"xyz\", expected: \"ics20-1\" }")]
fn fails_on_open_channel_open_try_invalid_version() {
    use cw_common::xcall_connection_msg::ExecuteMsg;
    use cw_xcall_ibc_connection::state::CwIbcConnection;

    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcConnection::default();

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
        .set_ibc_host(deps.as_mut().storage, Addr::unchecked(alice().as_str()))
        .unwrap();

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
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcConnection::default();

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
                dst.clone(),
                cosmwasm_std::IbcOrder::Unordered,
                "ics20-1",
                "newconnection",
            ),
            counterparty_version: "ics20-1".to_owned(),
        },
    };
    contract
        .configure_connection(
            deps.as_mut().storage,
            "newconnection".to_string(),
            dst.port_id,
            NetId::from("nid".to_string()),
            "client-id".to_string(),
            100,
        )
        .unwrap();
    contract
        .set_ibc_host(deps.as_mut().storage, Addr::unchecked(alice().as_str()))
        .unwrap();

    let result = contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_message)
        .unwrap();

    let result_data: IbcEndpoint = from_binary(&result.data.unwrap()).unwrap();
    assert_eq!(src.channel_id, result_data.channel_id);

    assert_eq!("ics20-1", result.attributes[1].value)
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn sucess_on_ibc_channel_connect() {
    use std::str::FromStr;

    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcConnection::default();
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };
    let connection_id = "newconnection";
    let execute_message = ExecuteMsg::IbcChannelConnect {
        msg: OpenAck {
            channel: IbcChannel::new(
                src.clone(),
                dst.clone(),
                cosmwasm_std::IbcOrder::Unordered,
                "ics20-1",
                connection_id,
            ),
            counterparty_version: "ics20-1".to_owned(),
        },
    };
    contract
        .set_ibc_host(deps.as_mut().storage, Addr::unchecked(alice().as_str()))
        .unwrap();

    contract
        .configure_connection(
            deps.as_mut().storage,
            "newconnection".to_string(),
            dst.port_id,
            NetId::from("btp".to_string()),
            "client-id".to_string(),
            100,
        )
        .unwrap();

    let result = contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_message)
        .unwrap();

    assert_eq!("on_channel_connect", result.attributes[0].value);

    let ibc_config = contract
        .get_ibc_config(deps.as_ref().storage, &NetId::from_str("btp").unwrap())
        .unwrap();

    assert_eq!(ibc_config.src_endpoint().port_id, src.port_id.as_str())
}

#[test]
#[cfg(not(feature = "native_ibc"))]
#[should_panic(expected = "UnOrderedChannel")]
fn fails_on_ibc_channel_connect_unordered_channel() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcConnection::default();
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
                src,
                dst,
                cosmwasm_std::IbcOrder::Ordered,
                "xcall-1",
                "newconnection",
            ),
            counterparty_version: "xcall-1".to_owned(),
        },
    };
    contract
        .set_ibc_host(deps.as_mut().storage, Addr::unchecked(alice().as_str()))
        .unwrap();

    contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_message)
        .unwrap();
}

#[test]
#[cfg(not(feature = "native_ibc"))]
#[should_panic(expected = " InvalidVersion { actual: \"xyz-1\", expected: \"ics20-1\" }")]
fn fails_on_ibc_channel_connect_invalid_counterparty_version() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcConnection::default();
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    contract
        .set_ibc_host(deps.as_mut().storage, Addr::unchecked(alice().as_str()))
        .unwrap();

    let execute_message = ExecuteMsg::IbcChannelConnect {
        msg: OpenAck {
            channel: IbcChannel::new(
                src,
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
    use common::rlp::{self, Nullable};
    use cw_common::xcall_types::network_address::NetworkAddress;
    use cw_xcall_ibc_connection::types::message::Message;

    let mut mock_deps = deps();
    let mock_info = create_mock_info("ibchostaddress", "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    let data = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
    );

    let message: CallServiceMessage = data.try_into().unwrap();
    let message: Message = Message {
        sn: Nullable::new(Some(1)),
        fee: 0,
        data: rlp::encode(&message).to_vec(),
    };
    let message_data = Binary(rlp::encode(&message).to_vec());

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
    contract
        .set_xcall_host(
            mock_deps.as_mut().storage,
            Addr::unchecked(alice().as_str()),
        )
        .unwrap();

    let packet = IbcPacket::new(message_data, src, dst.clone(), 0, timeout);
    let packet_message = IbcPacketReceiveMsg::new(packet, Addr::unchecked("relay"));

    let execute_message = ExecuteMsg::IbcPacketReceive {
        msg: packet_message,
    };
    contract
        .set_ibc_host(
            mock_deps.as_mut().storage,
            Addr::unchecked("ibchostaddress"),
        )
        .unwrap();
    contract
        .configure_connection(
            mock_deps.as_mut().storage,
            "newconnection".to_string(),
            dst.port_id,
            NetId::from("cnid".to_string()),
            "client-id".to_string(),
            100,
        )
        .unwrap();

    let result = contract.execute(mock_deps.as_mut(), mock_env, mock_info, execute_message);

    assert!(result.is_ok());
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn sucess_on_ack_packet() {
    use cw_common::xcall_types::network_address::{NetId, NetworkAddress};

    let mut mock_deps = deps();
    let mock_info = create_mock_info("alice", "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwIbcConnection::default();
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
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
    );
    contract
        .set_ibc_host(
            mock_deps.as_mut().storage,
            Addr::unchecked(alice().as_str()),
        )
        .unwrap();
    let channel = src.channel_id.clone();
    contract
        .store_outgoing_packet_sn(mock_deps.as_mut().storage, &channel, 0, 1)
        .unwrap();
    contract
        .set_xcall_host(mock_deps.as_mut().storage, Addr::unchecked("xcall-host"))
        .unwrap();

    let channel_config = ChannelConfig {
        client_id: "client_id".to_string(),
        timeout_height: 100,
        counterparty_nid: NetId::from("nid".to_string()),
    };
    contract
        .store_channel_config(mock_deps.as_mut().storage, &channel, &channel_config)
        .unwrap();
    let message: CallServiceMessage = data.try_into().unwrap();

    let packet = IbcPacket::new(to_binary(&message).unwrap(), src, dst, 0, timeout);

    let ack_packet = IbcPacketAckMsg::new(ack, packet, Addr::unchecked("relayer"));

    let execute_message = ExecuteMsg::IbcPacketAck { msg: ack_packet };

    let result = contract.execute(mock_deps.as_mut(), mock_env, mock_info, execute_message);
    assert!(result.is_ok());
}

#[test]
fn test_entry_point() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info("owner", "uconst", 200000);
    let env = mock_env();

    let msg = cw_common::xcall_connection_msg::ExecuteMsg::UpdateAdmin {
        address: admin_one().to_string(),
    };

    instantiate(
        mock_deps.as_mut(),
        env.clone(),
        mock_info.clone(),
        InstantiateMsg {
            ibc_host: Addr::unchecked("hostaddress"),
            xcall_address: Addr::unchecked("xcalladdress"),
            denom: "arch".to_string(),
            port_id: "mock".to_string(),
        },
    )
    .unwrap();

    execute(mock_deps.as_mut(), env.clone(), mock_info, msg).unwrap();

    let query_message = QueryMsg::GetAdmin {};

    let response =
        from_binary_response::<String>(&query(mock_deps.as_ref(), env, query_message).unwrap())
            .unwrap();

    assert_eq!(response, admin_one().to_string())
}

#[test]
#[cfg(not(feature = "native_ibc"))]
#[should_panic(expected = "NotFound")]
fn fails_receive_packet_for_call_message_request() {
    let mut mock_deps = deps();
    let mock_info = create_mock_info("alice", "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    let data = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
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

    contract
        .execute(mock_deps.as_mut(), mock_env, mock_info, execute_message)
        .unwrap();
}

#[test]
#[should_panic(expected = "NotFound")]
#[cfg(not(feature = "native_ibc"))]
fn fails_on_open_channel_open_init_unauthorized() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcConnection::default();

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
    contract
        .execute(deps.as_mut(), mock_env, mock_info, execute_msg)
        .unwrap();
}

#[test]
fn success_on_setting_timeout_height() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let contract = CwIbcConnection::default();

    let init_message = InstantiateMsg {
        ibc_host: Addr::unchecked("ibchostaddress"),
        xcall_address: Addr::unchecked("xcalladdress"),
        denom: "arch".to_string(),
        port_id: "mock".to_string(),
    };

    contract
        .instantiate(deps.as_mut(), mock_env.clone(), mock_info, init_message)
        .unwrap();

    contract
        .store_channel_config(
            deps.as_mut().storage,
            "channel",
            &ChannelConfig {
                client_id: "client_id".to_owned(),
                counterparty_nid: NetId::from("nid".to_string()),
                timeout_height: 100,
            },
        )
        .unwrap();

    let response: u64 = from_binary(
        &contract
            .query(
                deps.as_ref(),
                mock_env,
                QueryMsg::GetTimeoutHeight {
                    channel_id: "channel".to_string(),
                },
            )
            .unwrap(),
    )
    .unwrap();

    assert_eq!(response, 100)
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn fails_on_configure_connection_unauthorized() {
    let mut deps = deps();

    let mock_env = mock_env();
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcConnection::default();

    let init_message = InstantiateMsg {
        ibc_host: Addr::unchecked("ibchostaddress"),
        xcall_address: Addr::unchecked("xcalladdress"),
        denom: "arch".to_string(),
        port_id: "mock".to_string(),
    };

    contract
        .instantiate(deps.as_mut(), mock_env.clone(), mock_info, init_message)
        .unwrap();

    let exec_message = ExecuteMsg::ConfigureConnection {
        connection_id: "connection-1".to_string(),
        counterparty_port_id: "mock".to_string(),
        counterparty_nid: NetId::from("cnid".to_string()),

        client_id: "client_id".to_string(),
        timeout_height: 1000,
    };

    let mock_info = create_mock_info("bob", "umlg", 2000);
    contract
        .execute(deps.as_mut(), mock_env, mock_info, exec_message)
        .unwrap();
}

#[test]
fn test_ack_success_on_call_request() {
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let data = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
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

    let message = to_binary(&message).unwrap();

    let packet = IbcPacket::new(message, src, dst, 0, timeout);

    let ack = on_ack_sucess(packet);

    assert!(ack.is_ok())
}

#[test]
#[should_panic(expected = "ParseErr")]
fn test_ack_on_fails() {
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let data = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
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

    on_ack_sucess(packet).unwrap();
}

#[test]
fn test_ack_success_on_call_response() {
    let data = cw_xcall_multi::types::response::CallServiceMessageResponse::new(
        0,
        cw_xcall_multi::types::response::CallServiceResponseType::CallServiceResponseSuccess,
        "Success",
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

    let message = to_binary(&message).unwrap();

    let packet = IbcPacket::new(message, src, dst, 0, timeout);

    let ack = on_ack_sucess(packet);

    assert!(ack.is_ok())
}

#[test]
fn test_ack_failure_on_call_request() {
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let data = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
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

    let message = to_binary(&message).unwrap();

    let packet = IbcPacket::new(message, src, dst, 0, timeout);

    let ack = on_ack_failure(packet, "Failed to Execute");

    assert!(ack.is_ok())
}

#[test]
#[should_panic(expected = "ParseErr")]
fn fails_on_ack_failure_for_call_request() {
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let data = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
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

    on_ack_failure(packet, "Failed to Execute").unwrap();
}

#[test]
fn test_ack_failure_on_call_response() {
    let data = CallServiceMessageResponse::new(
        0,
        cw_xcall_multi::types::response::CallServiceResponseType::CallServiceResponseSuccess,
        "Success",
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

    let message = to_binary(&message).unwrap();

    let packet = IbcPacket::new(message, src, dst, 0, timeout);

    let ack = on_ack_failure(packet, "Failed to Execute");

    assert!(ack.is_ok())
}

#[test]
fn test_handle_response() {
    let mut mock_deps = deps();
    let mock_info = create_mock_info("alice", "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwIbcConnection::default();
    contract
        .set_ibc_host(
            mock_deps.as_mut().storage,
            Addr::unchecked(alice().as_str()),
        )
        .unwrap();
    contract
        .set_xcall_host(mock_deps.as_mut().storage, Addr::unchecked("xcallhost"))
        .unwrap();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    let data = CallServiceMessageResponse::new(
        0,
        cw_xcall_multi::types::response::CallServiceResponseType::CallServiceResponseSuccess,
        "Success",
    );

    let message: CallServiceMessage = data.try_into().unwrap();
    let message = Message {
        sn: Nullable::new(Some(0)),
        fee: 0,
        data: rlp::encode(&message).to_vec(),
    };
    let message_data = Binary(rlp::encode(&message).to_vec());

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

    let res = contract.execute(
        mock_deps.as_mut(),
        mock_env,
        mock_info,
        ExecuteMsg::IbcPacketReceive {
            msg: packet_message,
        },
    );

    assert!(res.is_ok())
}

#[test]
fn test_for_call_service_request_from_rlp_bytes() {
    let hex_decode_rlp_data = hex::decode("ed93736f6d65636f6e74726163746164647265737393736f6d65636f6e7472616374616464726573730100f800c0").unwrap();

    let cs_message_request = CallServiceMessageRequest::try_from(&hex_decode_rlp_data).unwrap();

    let expected_data = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", "somecontractaddress"),
        Addr::unchecked("somecontractaddress"),
        1,
        false,
        vec![],
        vec![],
    );

    assert_eq!(expected_data, cs_message_request)
}

#[test]
fn test_for_call_service_response_from_rlp_bytes() {
    let hex_decode_rlp_data = hex::decode("c90181fe8568656c6c6f").unwrap();
    let cs_response_message = CallServiceMessageResponse::try_from(&hex_decode_rlp_data).unwrap();

    let expected_data = CallServiceMessageResponse::new(
        1,
        cw_xcall_multi::types::response::CallServiceResponseType::CallServiceError,
        "hello",
    );

    assert_eq!(expected_data, cs_response_message)
}
#[test]
fn test_for_call_message_data_from_rlp_bytes() {
    let hex_decode = hex::decode("f001aeed93736f6d65636f6e74726163746164647265737393736f6d65636f6e7472616374616464726573730100f800c0").unwrap();

    let cs_message = CallServiceMessage::try_from(hex_decode).unwrap();

    let cs_message_request = CallServiceMessageRequest::try_from(cs_message.payload()).unwrap();

    let expected_data = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", "somecontractaddress"),
        Addr::unchecked("somecontractaddress"),
        1,
        false,
        vec![],
        vec![],
    );

    assert_eq!(expected_data, cs_message_request)
}

#[test]
fn test_call_message_from_raw_message() {
    let data=hex::decode("f001aeed93736f6d65636f6e74726163746164647265737393736f6d65636f6e7472616374616464726573730100f800c0").unwrap();

    let cs_message = CallServiceMessage::try_from(data).unwrap();

    let cs_message_request = CallServiceMessageRequest::try_from(cs_message.payload()).unwrap();

    let expected_data = CallServiceMessageRequest::new(
        NetworkAddress::new("nid", "somecontractaddress"),
        Addr::unchecked("somecontractaddress"),
        1,
        false,
        vec![],
        vec![],
    );

    assert_eq!(expected_data, cs_message_request)
}
