pub mod setup;
use std::str::FromStr;

use common::rlp::{self, Nullable};
use cosmwasm_std::{
    testing::mock_env, to_binary, Addr, Binary, IbcAcknowledgement, IbcChannel,
    IbcChannelConnectMsg::OpenAck, IbcChannelOpenMsg::OpenInit, IbcChannelOpenMsg::OpenTry,
    IbcEndpoint, IbcPacket, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcTimeout, IbcTimeoutBlock,
};

use cw_common::from_binary_response;
use cw_common::types::Ack;
use cw_xcall_ibc_connection::ack::{make_ack_success, on_ack_failure, on_ack_success};
use cw_xcall_ibc_connection::types::config::Config;
use cw_xcall_lib::network_address::{NetId, NetworkAddress};

use cw_xcall::types::response::CSMessageResponse;
use cw_xcall_ibc_connection::msg::InstantiateMsg;
use cw_xcall_ibc_connection::types::channel_config::ChannelConfig;
use cw_xcall_ibc_connection::types::message::Message;
use cw_xcall_ibc_connection::{execute, instantiate, query};
use setup::*;
pub mod account;
use account::admin_one;
use account::alice;

use cosmwasm_std::{from_binary, IbcChannelCloseMsg, IbcPacketTimeoutMsg, Reply, SubMsgResult};
use cw_common::xcall_connection_msg::{ExecuteMsg, QueryMsg};
use cw_xcall::types::message::CSMessage;
use cw_xcall::types::request::CSMessageRequest;
use cw_xcall_ibc_connection::state::{
    CwIbcConnection, ACK_FAILURE_ID, HOST_SEND_MESSAGE_REPLY_ID,
    HOST_WRITE_ACKNOWLEDGEMENT_REPLY_ID, XCALL_HANDLE_ERROR_REPLY_ID,
    XCALL_HANDLE_MESSAGE_REPLY_ID,
};

use cosmwasm_std::IbcOrder;

#[test]
#[cfg(not(feature = "native_ibc"))]
#[should_panic(expected = "OrderedChannel")]
fn fails_on_open_channel_open_init_ordered_channel() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let mut contract = CwIbcConnection::default();
    let info = create_mock_info("ibc_host", "umlg", 2000);

    ctx.init_context(deps.as_mut().storage, &contract);

    let mut channel = get_dummy_channel();
    channel.order = IbcOrder::Ordered;

    let execute_msg = ExecuteMsg::IbcChannelOpen {
        msg: OpenInit { channel },
    };
    contract
        .execute(deps.as_mut(), ctx.env, info, execute_msg)
        .unwrap();
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn success_on_open_channel_open_init_unordered_channel() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let mut contract = CwIbcConnection::default();
    let info = create_mock_info("ibc_host", "umlg", 2000);

    ctx.init_channel_open(deps.as_mut(), &contract);

    let execute_msg = ExecuteMsg::IbcChannelOpen {
        msg: OpenInit {
            channel: ctx.channel,
        },
    };
    let result = contract.execute(deps.as_mut(), ctx.env, info, execute_msg);
    assert!(result.is_ok())
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn test_query_get_ibc_config() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::default();

    ctx.init_context(deps.as_mut().storage, &contract);

    let query = QueryMsg::GetIbcConfig {
        nid: ctx.network_id.clone(),
    };
    let response = contract.query(deps.as_ref(), ctx.env, query);
    assert!(response.is_ok());

    let ibc_config = contract
        .get_ibc_config(deps.as_ref().storage, &ctx.network_id)
        .unwrap();
    assert_eq!(ibc_config.sequence(), 0);
    assert_eq!(ibc_config.next_sequence(), Some(1))
}

#[test]
fn test_execute_set_xcall_host() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::default();

    ctx.init_context(deps.as_mut().storage, &contract);

    let address = "xcall".to_string();
    let msg = ExecuteMsg::SetXCallHost {
        address: address.clone(),
    };
    let res = execute(deps.as_mut(), ctx.env, ctx.info, msg);
    assert!(res.is_ok());

    let xcall_host = contract.get_xcall_host(deps.as_ref().storage).unwrap();
    assert_eq!(xcall_host.to_string(), address)
}

#[test]
fn test_execute_configure_connection() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let mut contract = CwIbcConnection::default();

    ctx.init_context(deps.as_mut().storage, &contract);

    let msg = ExecuteMsg::ConfigureConnection {
        connection_id: ctx.connection_id,
        counterparty_port_id: "port_dst".to_owned(),
        counterparty_nid: ctx.network_id,
        client_id: ctx.client_id,
        timeout_height: 100,
    };
    let res = contract.execute(deps.as_mut(), ctx.env, ctx.info, msg);
    assert!(res.is_ok())
}

#[test]
fn test_execute_override_connection() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let mut contract = CwIbcConnection::default();

    ctx.init_channel_open(deps.as_mut(), &contract);

    let msg = ExecuteMsg::OverrideConnection {
        connection_id: ctx.connection_id.clone(),
        counterparty_port_id: "counterparty_port".to_owned(),
        counterparty_nid: NetId::from_str("new_nid").unwrap(),
        client_id: ctx.client_id,
        timeout_height: 200,
    };
    let res = contract.execute(deps.as_mut(), ctx.env, ctx.info, msg);
    assert!(res.is_ok());

    let counterparty_nid = contract
        .get_counterparty_nid(
            deps.as_ref().storage,
            &ctx.connection_id,
            "counterparty_port",
        )
        .unwrap();
    assert_eq!(counterparty_nid.as_str(), "new_nid")
}

#[test]
fn test_execute_set_fee() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::default();

    ctx.init_channel_open(deps.as_mut(), &contract);

    let msg = ExecuteMsg::SetFees {
        nid: ctx.network_id,
        packet_fee: 10,
        ack_fee: 10,
    };
    let res = execute(deps.as_mut(), ctx.env, ctx.info, msg);
    assert!(res.is_ok())
}

#[test]
fn test_query_get_fee() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::default();

    ctx.init_channel_open(deps.as_mut(), &contract);

    let msg = QueryMsg::GetFee {
        nid: ctx.network_id,
        response: true,
    };
    let res = query(deps.as_ref(), ctx.env, msg).unwrap();
    let fee: u128 = from_binary(&res).unwrap();
    assert_eq!(fee, 0)
}

#[test]
fn test_query_get_unclaimed_fee() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::default();

    ctx.init_channel_open(deps.as_mut(), &contract);

    let msg = QueryMsg::GetUnclaimedFee {
        nid: ctx.network_id,
        relayer: "crly".to_owned(),
    };
    let res = query(deps.as_ref(), ctx.env, msg).unwrap();
    let fee: u128 = from_binary(&res).unwrap();
    assert_eq!(fee, 0)
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn success_on_open_channel_open_try_valid_version() {
    use cosmwasm_std::from_binary;
    use cw_xcall_lib::network_address::NetId;

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
            deps.as_mut(),
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
    contract
        .store_config(
            deps.as_mut().storage,
            &Config {
                port_id: "our-port".to_string(),
                denom: "arch".to_string(),
            },
        )
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
fn success_on_ibc_channel_connect() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let mut contract = CwIbcConnection::default();
    let info = create_mock_info("ibc_host", "umlg", 2000);

    ctx.init_channel_connect(deps.as_mut(), &contract);

    let msg = ExecuteMsg::IbcChannelConnect {
        msg: OpenAck {
            channel: ctx.channel.clone(),
            counterparty_version: "ics20-1".to_owned(),
        },
    };

    let res = contract.execute(deps.as_mut(), ctx.env, info, msg).unwrap();
    assert_eq!("on_channel_connect", res.attributes[0].value);

    let ibc_config = contract
        .get_ibc_config(deps.as_ref().storage, &ctx.network_id)
        .unwrap();
    assert_eq!(
        ibc_config.src_endpoint().port_id,
        ctx.channel.endpoint.port_id.as_str()
    )
}

#[test]
fn test_execute_channel_close_init() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::default();

    ctx.init_context(deps.as_mut().storage, &contract);

    let msg = ExecuteMsg::IbcChannelClose {
        msg: IbcChannelCloseMsg::CloseInit {
            channel: ctx.channel,
        },
    };
    let res = execute(deps.as_mut(), ctx.env, ctx.info, msg).unwrap();
    assert_eq!(res.attributes[0].value, "ibc_channel_close")
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn reconfigure_non_open_channel() {
    use cosmwasm_std::{ContractResult, SystemResult, WasmQuery};
    use cw_common::{raw_types::channel::RawChannel, ProstMessage};
    use cw_xcall_ibc_connection::state::IbcConfig;
    use cw_xcall_lib::network_address::NetId;

    let mut deps = deps();
    let contract = CwIbcConnection::default();
    contract
        .set_ibc_host(deps.as_mut().storage, Addr::unchecked("ibc"))
        .unwrap();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };
    deps.querier.update_wasm(|r| match r {
        WasmQuery::Smart {
            contract_addr: _,
            msg: _,
        } => {
            let channel = RawChannel {
                state: 1,
                ordering: 1,
                counterparty: None,
                connection_hops: vec![],
                version: "".to_string(),
            };
            SystemResult::Ok(ContractResult::Ok(
                to_binary(&hex::encode(channel.encode_to_vec())).unwrap(),
            ))
        }
        _ => todo!(),
    });

    let nid = NetId::from("nid".to_string());
    let cfg = IbcConfig::new(src, dst);
    let res = contract.store_ibc_config(deps.as_mut().storage, &nid, &cfg);
    assert!(res.is_ok());

    let res = contract.configure_connection(
        deps.as_mut(),
        "newconnection".to_string(),
        "port".to_string(),
        nid,
        "client-id".to_string(),
        100,
    );

    assert!(res.is_ok());
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn reconfigure_open_channel() {
    use cosmwasm_std::{ContractResult, SystemResult, WasmQuery};
    use cw_common::{raw_types::channel::RawChannel, ProstMessage};
    use cw_xcall_ibc_connection::state::IbcConfig;
    use cw_xcall_lib::network_address::NetId;

    let mut deps = deps();

    let contract = CwIbcConnection::default();
    contract
        .set_ibc_host(deps.as_mut().storage, Addr::unchecked("ibc"))
        .unwrap();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };
    deps.querier.update_wasm(|r| match r {
        WasmQuery::Smart {
            contract_addr: _,
            msg: _,
        } => {
            let channel = RawChannel {
                state: 3,
                ordering: 1,
                counterparty: None,
                connection_hops: vec![],
                version: "".to_string(),
            };
            SystemResult::Ok(ContractResult::Ok(
                to_binary(&hex::encode(channel.encode_to_vec())).unwrap(),
            ))
        }
        _ => todo!(),
    });

    let nid = NetId::from("nid".to_string());
    let cfg = IbcConfig::new(src, dst);
    let res = contract.store_ibc_config(deps.as_mut().storage, &nid, &cfg);
    assert!(res.is_ok());

    let res = contract.configure_connection(
        deps.as_mut(),
        "newconnection".to_string(),
        "port".to_string(),
        nid,
        "client-id".to_string(),
        100,
    );
    assert!(res.is_err());
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn override_open_channel() {
    use cw_xcall_ibc_connection::state::IbcConfig;
    use cw_xcall_lib::network_address::NetId;

    let mut deps = deps();

    let contract = CwIbcConnection::default();
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };
    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let nid = NetId::from("nid".to_string());
    let cfg = IbcConfig::new(src, dst);
    let res = contract.store_ibc_config(deps.as_mut().storage, &nid, &cfg);
    assert!(res.is_ok());

    let connection_id = "newconnection".to_string();
    let client_id = "client-id".to_string();
    let res = contract.override_connection(
        deps.as_mut().storage,
        connection_id.clone(),
        "port".to_string(),
        nid,
        client_id.clone(),
        100,
    );

    assert!(res.is_ok());
    let cfg = contract
        .get_connection_config(deps.as_mut().storage, connection_id.as_str())
        .unwrap();
    assert!(cfg.client_id == client_id);
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
fn success_receive_packet_for_call_message_request() {
    use common::rlp::{self, Nullable};

    use cw_xcall_ibc_connection::types::message::Message;
    use cw_xcall_lib::network_address::{NetId, NetworkAddress};

    let mut mock_deps = deps();
    let mock_info = create_mock_info("ibchostaddress", "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    let data = CSMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
    );

    let message: CSMessage = data.try_into().unwrap();
    let message: Message = Message {
        sn: Nullable::new(Some(1)),
        fee: 0,
        data: cw_xcall::types::rlp::encode(&message).to_vec(),
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
            mock_deps.as_mut(),
            "newconnection".to_string(),
            dst.port_id,
            NetId::from("cnid".to_string()),
            "client-id".to_string(),
            100,
        )
        .unwrap();
    contract
        .store_config(
            mock_deps.as_mut().storage,
            &Config {
                port_id: "our-port".to_string(),
                denom: "arch".to_string(),
            },
        )
        .unwrap();
    let channel_config = ChannelConfig {
        client_id: "client_id".to_string(),
        timeout_height: 100,
        counterparty_nid: NetId::from("nid".to_string()),
    };

    contract
        .store_channel_config(mock_deps.as_mut().storage, &dst.channel_id, &channel_config)
        .unwrap();

    let result = contract.execute(mock_deps.as_mut(), mock_env, mock_info, execute_message);
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[test]
#[cfg(not(feature = "native_ibc"))]
fn on_ack_packet() {
    let packet = get_dummy_ibc_packet();
    let ctx = TestContext::for_packet_ack(&packet);

    let mut deps = deps();
    let info = create_mock_info("ibc_host", "umlg", 2000);
    let mut contract = CwIbcConnection::default();

    ctx.init_context(deps.as_mut().storage, &contract);

    let ack = IbcAcknowledgement::new(
        to_binary(&Ack::Result(
            Binary::from_base64("aGVsbG8gd29ybGQ=").unwrap(),
        ))
        .unwrap(),
    );

    let ack_packet = IbcPacketAckMsg::new(ack, packet, Addr::unchecked("relayer"));

    let msg = ExecuteMsg::IbcPacketAck { msg: ack_packet };
    let result = contract.execute(deps.as_mut(), ctx.env, info, msg);
    assert!(result.is_ok());
}

#[test]
fn test_entry_point() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info("owner", "uconst", 200000);
    let env = mock_env();

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
    let msg = cw_common::xcall_connection_msg::ExecuteMsg::SetAdmin {
        address: admin_one().to_string(),
    };

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
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    let data = CSMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
    );

    let message: CSMessage = data.try_into().unwrap();

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
#[should_panic(expected = "OnlyAdmin")]
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

    let data = CSMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
    );

    let message: CSMessage = data.try_into().unwrap();

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

    let ack = on_ack_success(packet);

    assert!(ack.is_ok())
}

#[test]
fn test_ack_success_on_call_response() {
    let data = cw_xcall::types::response::CSMessageResponse::new(
        0,
        cw_xcall::types::response::CallServiceResponseType::CallServiceResponseSuccess,
    );

    let message: CSMessage = data.try_into().unwrap();

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

    let ack = on_ack_success(packet);

    assert!(ack.is_ok())
}

#[test]
fn test_ack_failure_on_call_request() {
    let mock_info = create_mock_info("alice", "umlg", 2000);

    let data = CSMessageRequest::new(
        NetworkAddress::new("nid", mock_info.sender.as_str()),
        Addr::unchecked("alice"),
        1,
        false,
        vec![1, 2, 3],
        vec![],
    );

    let message: CSMessage = data.try_into().unwrap();

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
fn test_ack_failure_on_call_response() {
    let data = CSMessageResponse::new(
        0,
        cw_xcall::types::response::CallServiceResponseType::CallServiceResponseSuccess,
    );

    let message: CSMessage = data.try_into().unwrap();

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
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    let data = CSMessageResponse::new(
        0,
        cw_xcall::types::response::CallServiceResponseType::CallServiceResponseSuccess,
    );

    let message: CSMessage = data.try_into().unwrap();
    let message = Message {
        sn: Nullable::new(Some(0)),
        fee: 0,
        data: cw_xcall::types::rlp::encode(&message).to_vec(),
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
        .store_channel_config(
            &mut mock_deps.storage,
            &dst.channel_id,
            &ChannelConfig {
                client_id: "client".to_string(),
                timeout_height: 1000,
                counterparty_nid: NetId::from_str("nid").unwrap(),
            },
        )
        .unwrap();
    contract
        .store_config(
            mock_deps.as_mut().storage,
            &Config {
                port_id: "our-port".to_string(),
                denom: "arch".to_string(),
            },
        )
        .unwrap();
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

    println!("{:?}", res);

    assert!(res.is_ok())
}

#[test]
fn test_for_call_service_request_from_rlp_bytes() {
    let hex_decode_rlp_data = hex::decode("f1976e69642f736f6d65636f6e74726163746164647265737393736f6d65636f6e7472616374616464726573730100f800c0").unwrap();

    let cs_message_request = CSMessageRequest::try_from(&hex_decode_rlp_data).unwrap();

    let expected_data = CSMessageRequest::new(
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
    let hex_decode_rlp_data = hex::decode("c20100").unwrap();
    let cs_response_message = CSMessageResponse::try_from(&hex_decode_rlp_data).unwrap();

    let expected_data = CSMessageResponse::new(
        1,
        cw_xcall::types::response::CallServiceResponseType::CallServiceResponseFailure,
    );

    assert_eq!(expected_data, cs_response_message)
}
#[test]
fn test_for_call_message_data_from_rlp_bytes() {
    let hex_decode = hex::decode("f401b2f1976e69642f736f6d65636f6e74726163746164647265737393736f6d65636f6e7472616374616464726573730100f800c0").unwrap();

    let cs_message = CSMessage::try_from(hex_decode).unwrap();

    let cs_message_request = CSMessageRequest::try_from(cs_message.payload()).unwrap();

    let expected_data = CSMessageRequest::new(
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
    let data=hex::decode("f401b2f1976e69642f736f6d65636f6e74726163746164647265737393736f6d65636f6e7472616374616464726573730100f800c0").unwrap();

    let cs_message = CSMessage::try_from(data).unwrap();

    let cs_message_request = CSMessageRequest::try_from(cs_message.payload()).unwrap();

    let expected_data = CSMessageRequest::new(
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
fn test_make_ack_success() {
    let res = make_ack_success();

    let res_bytes: Vec<u8> = res.into();
    let res_str = String::from_utf8(res_bytes).unwrap();

    assert_eq!(res_str, "{\"result\":\"MQ==\"}")
}

#[test]
#[should_panic(expected = "ReplyError { code: 1, msg: \"Unknown\" }")]
fn test_xcall_handle_message_reply_fail() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::new();

    let msg = Reply {
        id: XCALL_HANDLE_MESSAGE_REPLY_ID,
        result: SubMsgResult::Err("Unknown".to_string()),
    };

    contract.reply(deps.as_mut(), ctx.env, msg).unwrap();
}

#[test]
fn test_xcall_handle_error_reply() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::new();

    let sub_msg_reply = get_dummy_sub_msg_res();
    let msg = Reply {
        id: XCALL_HANDLE_ERROR_REPLY_ID,
        result: SubMsgResult::Ok(sub_msg_reply),
    };

    let res = contract.reply(deps.as_mut(), ctx.env, msg).unwrap();
    assert_eq!(res.attributes[0].value, "call_message");
    assert_eq!(res.attributes[1].value, "xcall_handle_error_reply")
}

#[test]
#[should_panic(expected = "ReplyError { code: 2, msg: \"Unknown\" }")]
fn test_xcall_handle_error_reply_fail() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::new();

    let msg = Reply {
        id: XCALL_HANDLE_ERROR_REPLY_ID,
        result: SubMsgResult::Err("Unknown".to_string()),
    };

    contract.reply(deps.as_mut(), ctx.env, msg).unwrap();
}

#[test]
fn test_host_send_message_reply() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::new();

    let sub_msg_reply = get_dummy_sub_msg_res();
    let msg = Reply {
        id: HOST_SEND_MESSAGE_REPLY_ID,
        result: SubMsgResult::Ok(sub_msg_reply),
    };

    let res = contract.reply(deps.as_mut(), ctx.env, msg).unwrap();
    assert_eq!(res.attributes[0].value, "call_message");
    assert_eq!(res.attributes[1].value, "reply_forward_host")
}

#[test]
#[should_panic(expected = "ReplyError { code: 4, msg: \"Unknown\" }")]
fn test_host_send_message_reply_fail() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::new();

    let msg = Reply {
        id: HOST_SEND_MESSAGE_REPLY_ID,
        result: SubMsgResult::Err("Unknown".to_string()),
    };

    contract.reply(deps.as_mut(), ctx.env, msg).unwrap();
}

#[test]
fn test_host_write_acknowledgement_reply() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::new();

    let sub_msg_reply = get_dummy_sub_msg_res();
    let msg = Reply {
        id: HOST_WRITE_ACKNOWLEDGEMENT_REPLY_ID,
        result: SubMsgResult::Ok(sub_msg_reply),
    };

    let res = contract.reply(deps.as_mut(), ctx.env, msg).unwrap();
    assert_eq!(res.attributes[0].value, "call_message");
    assert_eq!(res.attributes[1].value, "reply_write_acknowledgement")
}

#[test]
#[should_panic(expected = "ReplyError { code: 3, msg: \"Unknown\" }")]
fn test_host_write_acknowledgement_reply_fail() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::new();

    let msg = Reply {
        id: HOST_WRITE_ACKNOWLEDGEMENT_REPLY_ID,
        result: SubMsgResult::Err("Unknown".to_string()),
    };

    contract.reply(deps.as_mut(), ctx.env, msg).unwrap();
}

#[test]
fn test_reply_ack_on_error() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::new();

    let sub_msg_reply = get_dummy_sub_msg_res();
    let msg = Reply {
        id: ACK_FAILURE_ID,
        result: SubMsgResult::Ok(sub_msg_reply),
    };

    let res = contract.reply(deps.as_mut(), ctx.env, msg);
    assert!(res.is_ok())
}

#[test]
fn test_reply_ack_on_error_fail() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::new();

    let msg = Reply {
        id: ACK_FAILURE_ID,
        result: SubMsgResult::Err("Unknown".to_string()),
    };

    let res = contract.reply(deps.as_mut(), ctx.env, msg);
    assert!(res.is_ok())
}

#[test]
#[should_panic(expected = "ReplyError { code: 9, msg: \"Unknown\" }")]
fn test_reply_fail_for_invalid_id() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::new();

    let invalid_reply_id = 9;
    let msg = Reply {
        id: invalid_reply_id,
        result: SubMsgResult::Err("Unknown".to_string()),
    };

    contract.reply(deps.as_mut(), ctx.env, msg).unwrap();
}
