pub mod setup;

use cosmwasm_std::{
    to_json_binary as to_binary, Addr, Binary, Ibc3ChannelOpenResponse, IbcAcknowledgement,
    IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcPacketAckMsg,
};

use cw_common::types::Ack;
use cw_xcall_ibc_connection::ibc::*;
use cw_xcall_ibc_connection::state::CwIbcConnection;

use setup::*;

#[test]
fn test_ibc_channel_open() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::default();

    ctx.init_channel_open(deps.as_mut(), &contract);

    let msg = IbcChannelOpenMsg::OpenInit {
        channel: ctx.channel,
    };
    let res = ibc_channel_open(deps.as_mut(), ctx.env, msg).unwrap();
    let expected_res = Some(Ibc3ChannelOpenResponse {
        version: IBC_VERSION.to_string(),
    });

    assert_eq!(res, expected_res);
}

#[test]
fn test_ibc_channel_connect() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::default();

    ctx.init_channel_connect(deps.as_mut(), &contract);

    let msg = IbcChannelConnectMsg::OpenAck {
        channel: ctx.channel,
        counterparty_version: IBC_VERSION.to_string(),
    };
    let res = ibc_channel_connect(deps.as_mut(), ctx.env, msg).unwrap();
    assert_eq!(res.attributes[0].value, "on_channel_connect");
}

#[test]
fn test_channel_close_confirm() {
    let ctx = TestContext::default();
    let mut deps = deps();
    let contract = CwIbcConnection::default();

    ctx.init_context(deps.as_mut().storage, &contract);

    let msg = IbcChannelCloseMsg::CloseConfirm {
        channel: ctx.channel,
    };
    let res = ibc_channel_close(deps.as_mut(), ctx.env, msg).unwrap();
    assert_eq!(res.attributes[0].value, "ibc_channel_close")
}

#[test]
fn test_ibc_packet_ack() {
    let packet = get_dummy_ibc_packet();
    let ctx = TestContext::for_packet_ack(&packet);

    let mut deps = deps();
    let contract = CwIbcConnection::default();

    ctx.init_context(deps.as_mut().storage, &contract);

    let ack = IbcAcknowledgement::new(
        to_binary(&Ack::Result(
            Binary::from_base64("aGVsbG8gd29ybGQ=").unwrap(),
        ))
        .unwrap(),
    );

    let msg = IbcPacketAckMsg::new(ack, packet, Addr::unchecked("relayer"));
    let res = ibc_packet_ack(deps.as_mut(), ctx.env, msg);
    assert!(res.is_ok());
}
