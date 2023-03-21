use cw_ibc_core::{
    context::CwIbcCoreContext,
    ics04_channel::{
        MsgChannelOpenAck, MsgChannelOpenConfirm, MsgChannelOpenInit, MsgChannelOpenTry,
    },
    types::{ChannelId, PortId},
    ChannelEnd, Sequence,
};
use ibc::core::ics04_channel::{
    channel::{Counterparty, Order, State},
    Version,
};
use ibc_proto::ibc::core::{
    channel::v1::{
        MsgChannelOpenAck as RawMsgChannelOpenAck,
        MsgChannelOpenConfirm as RawMsgChannelOpenConfirm,
        MsgChannelOpenInit as RawMsgChannelOpenInit, MsgChannelOpenTry as RawMsgChannelOpenTry,
    },
    client::v1::Height,
};

pub mod setup;
use setup::*;

#[test]
fn test_add_channel() {
    let ctx = CwIbcCoreContext::new();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let channel_end = ChannelEnd::new(
        State::Init,
        Order::None,
        Counterparty::default(),
        Vec::default(),
        Version::from("ics-20".to_string()),
    );
    let mut mock_deps = deps();

    let _storing = ctx.add_channel_end(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        channel_end.clone(),
    );

    let retrived_channel_end = ctx.get_channel_end(mock_deps.as_ref().storage, port_id, channel_id);

    assert_eq!(channel_end, retrived_channel_end.unwrap())
}

#[test]
fn test_channel_sequence_initialisation() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let _store = ctx.init_next_channel_sequence(mock_deps.as_mut().storage, u128::default());
    let result = ctx.query_channel_sequence(mock_deps.as_ref().storage);

    assert_eq!(0, result.unwrap());

    let incremented_result = ctx.increment_channel_sequence(mock_deps.as_mut().storage);
    assert_eq!(1, incremented_result.unwrap());
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u128\" })")]
fn test_channel_sequence_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    ctx.increment_channel_sequence(mock_deps.as_mut().storage)
        .unwrap();
}

#[test]
fn test_channel_sequence_send() {
    let ctx = CwIbcCoreContext::new();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let sequene = Sequence::from(6);
    let mut mock_deps = deps();

    let _store = ctx.store_next_sequence_send(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        sequene,
    );
    let result = ctx.query_next_sequence_send(mock_deps.as_ref().storage, port_id, channel_id);

    assert_eq!(sequene, result.unwrap())
}

#[test]
fn test_channel_sequence_send_increment() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let sequence = Sequence::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let _store = ctx.store_next_sequence_send(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        sequence,
    );
    let result = ctx.query_next_sequence_send(
        mock_deps.as_ref().storage,
        port_id.clone(),
        channel_id.clone(),
    );

    assert_eq!(sequence, result.unwrap());

    let incremented_result = ctx.increment_next_sequence_send(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
    );
    assert_eq!(Sequence::from(1), incremented_result.unwrap());
}

#[test]
fn test_channel_sequence_recv_increment() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let sequence = Sequence::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let _store = ctx.store_next_sequence_recv(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        sequence,
    );
    let result = ctx.query_next_sequence_recv(
        mock_deps.as_ref().storage,
        port_id.clone(),
        channel_id.clone(),
    );

    assert_eq!(sequence, result.unwrap());

    let incremented_result = ctx.increment_next_sequence_recv(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
    );
    assert_eq!(Sequence::from(1), incremented_result.unwrap());
}

#[test]
fn test_channel_sequence_ack_increment() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let sequence = Sequence::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let _store = ctx.store_next_sequence_ack(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        sequence,
    );
    let result = ctx.query_next_sequence_ack(
        mock_deps.as_ref().storage,
        port_id.clone(),
        channel_id.clone(),
    );

    assert_eq!(sequence, result.unwrap());

    let incremented_result = ctx.increment_next_sequence_ack(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
    );
    assert_eq!(Sequence::from(1), incremented_result.unwrap());
}

#[test]
#[should_panic(expected = "MissingNextAckSeq")]
fn test_channel_sequence_ack_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    ctx.increment_next_sequence_ack(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "MissingNextSendSeq")]
fn test_channel_sequence_send_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    ctx.increment_next_sequence_send(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "MissingNextRecvSeq")]
fn test_channel_sequence_recv_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    ctx.increment_next_sequence_recv(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
    )
    .unwrap();
}

#[test]
pub fn test_to_and_from_channel_open_init() {
    let raw = get_dummy_raw_msg_chan_open_init(None);
    let msg = MsgChannelOpenInit::try_from(raw.clone()).unwrap();
    let raw_back = RawMsgChannelOpenInit::from(msg.clone());
    let msg_back = MsgChannelOpenInit::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
}

#[test]
pub fn test_to_and_from_channel_open_ack() {
    let raw = get_dummy_raw_msg_chan_open_ack(100);
    let msg = MsgChannelOpenAck::try_from(raw.clone()).unwrap();
    let raw_back = RawMsgChannelOpenAck::from(msg.clone());
    let msg_back = MsgChannelOpenAck::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
}
#[test]
pub fn test_to_and_from_channel_open_confirm() {
    let raw = get_dummy_raw_msg_chan_open_confirm(19);
    let msg = MsgChannelOpenConfirm::try_from(raw.clone()).unwrap();
    let raw_back = RawMsgChannelOpenConfirm::from(msg.clone());
    let msg_back = MsgChannelOpenConfirm::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
}
#[test]
pub fn test_to_and_from_channel_open_try() {
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let msg = MsgChannelOpenTry::try_from(raw.clone()).unwrap();
    let raw_back = RawMsgChannelOpenTry::from(msg.clone());
    let msg_back = MsgChannelOpenTry::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
}

#[test]
fn channel_open_init_from_raw_good_parameter() {
    let default_raw_init_msg = get_dummy_raw_msg_chan_open_init(None);
    let res_msg = MsgChannelOpenInit::try_from(default_raw_init_msg.clone());
    assert_eq!(res_msg.is_ok(), true)
}
#[test]
#[should_panic(expected = "Identifier(ContainSeparator { id: \"p34/\" })")]
fn channel_open_init_from_raw_incorrect_port_id_parameter() {
    let default_raw_init_msg = get_dummy_raw_msg_chan_open_init(None);
    let default_raw_init_msg = RawMsgChannelOpenInit {
        port_id: "p34/".to_string(),
        ..default_raw_init_msg.clone()
    };
    let res_msg = MsgChannelOpenInit::try_from(default_raw_init_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "MissingChannel")]
fn channel_open_init_from_raw_missing_channel_parameter() {
    let default_raw_init_msg = get_dummy_raw_msg_chan_open_init(None);
    let default_raw_init_msg = RawMsgChannelOpenInit {
        channel: None,
        ..default_raw_init_msg
    };
    let res_msg = MsgChannelOpenInit::try_from(default_raw_init_msg.clone());
    res_msg.unwrap();
}

#[test]
fn channel_open_confirm_from_raw_good_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_msg.clone());
    assert_eq!(res_msg.is_ok(), true)
}
#[test]
#[should_panic(expected = "Identifier(ContainSeparator { id: \"p34/\" })")]
fn channel_open_confirm_from_raw_incorrect_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let default_raw_confirm_msg = RawMsgChannelOpenConfirm {
        port_id: "p34/".to_string(),
        ..default_raw_msg.clone()
    };
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_open_confirm_from_raw_missing_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let default_raw_confirm_msg = RawMsgChannelOpenConfirm {
        proof_height: None,
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_open_confirm_from_raw_missing_proof_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let default_raw_confirm_msg = RawMsgChannelOpenConfirm {
        proof_height: Some(Height {
            revision_number: 0,
            revision_height: 0,
        }),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "InvalidProof")]
fn channel_open_confirm_from_raw_missing_proof_try_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let default_raw_confirm_msg = RawMsgChannelOpenConfirm {
        proof_ack: Vec::new(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "InvalidLength")]
fn channel_open_confirm_from_raw_invalid_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let default_raw_confirm_msg = RawMsgChannelOpenConfirm {
        port_id: "abcdefghijasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadgasgasdfasdfaabcdefghijasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadgasgasdfasdfa".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "InvalidLength")]
fn channel_open_confirm_from_raw_bad_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let default_raw_confirm_msg = RawMsgChannelOpenConfirm {
        port_id: "p".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg.clone());
    res_msg.unwrap();
}
