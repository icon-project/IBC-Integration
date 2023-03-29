use std::str::FromStr;

use cw_ibc_core::{
    context::CwIbcCoreContext,
    ics04_channel::{
        create_ack_packet_event, create_channel_id_generated_event,
        create_close_confirm_channel_event, create_close_init_channel_event,
        create_open_ack_channel_event, create_open_confirm_channel_event,
        create_open_init_channel_event, create_open_try_channel_event, create_packet_timeout_event,
        create_send_packet_event, create_write_ack_event, MsgChannelCloseConfirm,
        MsgChannelCloseInit, MsgChannelOpenAck, MsgChannelOpenConfirm, MsgChannelOpenInit,
        MsgChannelOpenTry,
    },
    types::{ChannelId, PortId},
    ChannelEnd, IbcConnectionId, Sequence,
};
use ibc::{
    core::ics04_channel::{
        channel::{Counterparty, Order, State},
        packet::Packet,
        Version,
    },
    events::IbcEventType,
    signer::Signer,
};
use ibc_proto::ibc::core::{
    channel::v1::{
        MsgChannelCloseConfirm as RawMsgChannelCloseConfirm,
        MsgChannelCloseInit as RawMsgChannelCloseInit, MsgChannelOpenAck as RawMsgChannelOpenAck,
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
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let channel_end = ChannelEnd::new(
        State::Init,
        Order::None,
        Counterparty::default(),
        Vec::default(),
        Version::from("ics-20".to_string()),
    );
    let mut mock_deps = deps();

    let _storing = ctx.store_channel_end(
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
    let _store = ctx.init_channel_counter(mock_deps.as_mut().storage, u128::default());
    let result = ctx.channel_counter(mock_deps.as_ref().storage);

    assert_eq!(0, result.unwrap());

    let incremented_result = ctx.increase_channel_sequence(mock_deps.as_mut().storage);
    assert_eq!(1, incremented_result.unwrap());
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u128\" })")]
fn test_channel_sequence_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    ctx.increase_channel_sequence(mock_deps.as_mut().storage)
        .unwrap();
}

#[test]
fn test_channel_sequence_send() {
    let ctx = CwIbcCoreContext::new();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let sequene = Sequence::from(6);
    let mut mock_deps = deps();

    let _store = ctx.store_next_sequence_send(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        sequene,
    );
    let result = ctx.get_next_sequence_send(mock_deps.as_ref().storage, port_id, channel_id);

    assert_eq!(sequene, result.unwrap())
}

#[test]
fn test_channel_sequence_send_increment() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let sequence = Sequence::default();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let _store = ctx.store_next_sequence_send(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        sequence,
    );
    let result = ctx.get_next_sequence_send(
        mock_deps.as_ref().storage,
        port_id.clone(),
        channel_id.clone(),
    );

    assert_eq!(sequence, result.unwrap());

    let incremented_result = ctx.increase_next_sequence_send(
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
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let _store = ctx.store_next_sequence_recv(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        sequence,
    );
    let result = ctx.get_next_sequence_recv(
        mock_deps.as_ref().storage,
        port_id.clone(),
        channel_id.clone(),
    );

    assert_eq!(sequence, result.unwrap());

    let incremented_result = ctx.increase_next_sequence_recv(
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
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let _store = ctx.store_next_sequence_ack(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        sequence,
    );
    let result = ctx.get_next_sequence_ack(
        mock_deps.as_ref().storage,
        port_id.clone(),
        channel_id.clone(),
    );

    assert_eq!(sequence, result.unwrap());

    let incremented_result = ctx.increase_next_sequence_ack(
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
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    ctx.increase_next_sequence_ack(
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
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    ctx.increase_next_sequence_send(
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
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    ctx.increase_next_sequence_recv(
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

#[test]
fn channel_open_confirm_from_raw_valid_channel_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let default_raw_confirm_msg = RawMsgChannelOpenConfirm {
        channel_id: "channel-34".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg.clone());

    let expected = MsgChannelOpenConfirm {
        port_id_on_b: PortId::default().ibc_port_id().clone(),
        chan_id_on_b: ChannelId::new(34).ibc_channel_id().clone(),
        proof_chan_end_on_a:
            ibc::core::ics23_commitment::commitment::CommitmentProofBytes::try_from(
                get_dummy_proof(),
            )
            .unwrap(),
        proof_height_on_a: ibc::core::ics02_client::height::Height::new(0, proof_height).unwrap(),
        signer: Signer::from_str("cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng").unwrap(),
    };
    assert_eq!(res_msg.unwrap(), expected);
}

#[test]
fn channel_open_try_from_raw_good_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(proof_height);
    let res_msg = MsgChannelOpenTry::try_from(default_raw_msg.clone());
    assert_eq!(res_msg.is_ok(), true)
}
#[test]
#[should_panic(expected = "Identifier(ContainSeparator { id: \"p34/\" })")]
fn channel_open_try_from_raw_incorrect_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(proof_height);
    let default_raw_try_msg = RawMsgChannelOpenTry {
        port_id: "p34/".to_string(),
        ..default_raw_msg.clone()
    };
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_open_try_from_raw_missing_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(proof_height);
    let default_raw_try_msg = RawMsgChannelOpenTry {
        proof_height: None,
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_open_try_from_raw_missing_proof_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(proof_height);
    let default_raw_try_msg = RawMsgChannelOpenTry {
        proof_height: Some(Height {
            revision_number: 0,
            revision_height: 0,
        }),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "InvalidProof")]
fn channel_open_try_from_raw_missing_proof_init_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(proof_height);
    let default_raw_try_msg = RawMsgChannelOpenTry {
        proof_init: Vec::new(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "InvalidLength")]
fn channel_open_try_from_raw_invalid_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(proof_height);
    let default_raw_try_msg = RawMsgChannelOpenTry {
        port_id: "abcdefghijasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadgasgasdfasdfaabcdefghijasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadgasgasdfasdfa".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "InvalidLength")]
fn channel_open_try_from_raw_bad_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(proof_height);
    let default_raw_try_msg = RawMsgChannelOpenTry {
        port_id: "p".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg.clone());
    res_msg.unwrap();
}

#[test]
fn channel_open_ack_from_raw_good_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let res_msg = MsgChannelOpenAck::try_from(default_raw_msg.clone());
    assert_eq!(res_msg.is_ok(), true)
}
#[test]
#[should_panic(expected = "Identifier(ContainSeparator { id: \"p34/\" })")]
fn channel_open_ack_from_raw_incorrect_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let default_raw_ack_msg = RawMsgChannelOpenAck {
        port_id: "p34/".to_string(),
        ..default_raw_msg.clone()
    };
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_open_ack_from_raw_missing_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let default_raw_ack_msg = RawMsgChannelOpenAck {
        proof_height: None,
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_open_ack_from_raw_missing_proof_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let default_raw_ack_msg = RawMsgChannelOpenAck {
        proof_height: Some(Height {
            revision_number: 0,
            revision_height: 0,
        }),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "InvalidProof")]
fn channel_open_ack_from_raw_missing_proof_try_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let default_raw_ack_msg = RawMsgChannelOpenAck {
        proof_try: Vec::new(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "InvalidLength")]
fn channel_open_ack_from_raw_invalid_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let default_raw_ack_msg = RawMsgChannelOpenAck {
        port_id: "abcdefghijasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadgasgasdfasdfaabcdefghijasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadgasgasdfasdfa".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "InvalidLength")]
fn channel_open_ack_from_raw_bad_channel_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let default_raw_ack_msg = RawMsgChannelOpenAck {
        channel_id: "chshort".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}

#[test]
fn create_channel_id_event_test() {
    let client_id = ChannelId::new(10);
    let event = create_channel_id_generated_event(client_id);

    assert_eq!("channel_id_created", event.ty);
    assert_eq!("channel-10", event.attributes[0].value);
    assert_eq!("channel_id", event.attributes[0].key)
}
#[test]
pub fn test_to_and_from_channel_close_init() {
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw.clone()).unwrap();
    let raw_back = RawMsgChannelCloseInit::from(msg.clone());
    let msg_back = MsgChannelCloseInit::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
}

#[test]
fn channel_close_innit_from_raw_valid_channel_id_parameter() {
    let default_raw_msg = get_dummy_raw_msg_chan_close_init();
    let default_raw_confirm_msg = RawMsgChannelCloseInit {
        channel_id: "channel-34".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelCloseInit::try_from(default_raw_confirm_msg.clone());

    let expected = MsgChannelCloseInit {
        port_id_on_a: PortId::default().ibc_port_id().clone(),
        chan_id_on_a: ChannelId::new(34).ibc_channel_id().clone(),
        signer: Signer::from_str("cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng").unwrap(),
    };
    assert_eq!(res_msg.unwrap(), expected);
}

#[test]
#[should_panic(expected = "InvalidLength")]
fn channel_close_init_from_raw_bad_channel_id_parameter() {
    let default_raw_msg = get_dummy_raw_msg_chan_close_init();
    let default_raw_ack_msg = RawMsgChannelCloseInit {
        channel_id: "chshort".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelCloseInit::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}

#[test]
#[should_panic(expected = "InvalidLength")]
fn channel_close_init_from_raw_bad_port_id_parameter() {
    let default_raw_msg = get_dummy_raw_msg_chan_close_init();
    let default_raw_ack_msg = RawMsgChannelCloseInit {
        port_id: "abcdefsdfasdfasdfasdfasdfasdfadsfasdgafsgadfasdfasdfasdfsdfasdfaghijklmnopqrstuabcdefsdfasdfasdfasdfasdfasdfadsfasdgafsgadfasdfasdfasdfsdfasdfaghijklmnopqrstu".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelCloseInit::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}

#[test]
pub fn test_to_and_from_channel_close_confirm() {
    let proof_height = 10;
    let raw = get_dummy_raw_msg_chan_close_confirm(proof_height);
    let msg = MsgChannelCloseConfirm::try_from(raw.clone()).unwrap();
    let raw_back = RawMsgChannelCloseConfirm::from(msg.clone());
    let msg_back = MsgChannelCloseConfirm::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
}

#[test]
#[should_panic(expected = "InvalidLength")]
fn channel_close_confirm_from_raw_bad_channel_id_parameter_too_long() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_close_confirm(proof_height);
    let default_raw_ack_msg = RawMsgChannelCloseConfirm {
        channel_id: "channel-128391283791827398127398791283912837918273981273987912839".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelCloseConfirm::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}

#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_close_confirm_from_raw_missing_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_close_confirm(proof_height);
    let default_raw_ack_msg = RawMsgChannelCloseConfirm {
        proof_height: Some(Height {
            revision_number: 0,
            revision_height: 0,
        }),
        ..default_raw_msg
    };
    let res_msg = MsgChannelCloseConfirm::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}

#[test]
#[should_panic(expected = "InvalidLength")]
fn channel_close_confirm_from_raw_bad_channel_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_close_confirm(proof_height);
    let default_raw_ack_msg = RawMsgChannelCloseConfirm {
        channel_id: "chshort".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelCloseConfirm::try_from(default_raw_ack_msg.clone());
    res_msg.unwrap();
}

#[test]
fn channel_close_confirm_from_raw() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_close_confirm(proof_height);
    let default_raw_confirm_msg = RawMsgChannelCloseConfirm {
        channel_id: "channel-34".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelCloseConfirm::try_from(default_raw_confirm_msg.clone());

    let expected = MsgChannelCloseConfirm {
        port_id_on_b: PortId::default().ibc_port_id().clone(),
        chan_id_on_b: ChannelId::new(34).ibc_channel_id().clone(),
        proof_chan_end_on_a:
            ibc::core::ics23_commitment::commitment::CommitmentProofBytes::try_from(
                get_dummy_proof(),
            )
            .unwrap(),
        proof_height_on_a: ibc::core::ics02_client::height::Height::new(0, proof_height).unwrap(),
        signer: Signer::from_str("cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng").unwrap(),
    };
    assert_eq!(res_msg.unwrap(), expected);
}

#[test]
fn create_open_ack_channel_event_test() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let message = MsgChannelOpenAck::try_from(default_raw_msg.clone()).unwrap();
    let event = create_open_ack_channel_event(&message);

    assert_eq!(IbcEventType::OpenAckChannel.as_str(), event.ty);
    assert_eq!("channel-0", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
    assert_eq!("channel_id", event.attributes[1].key);
}

#[test]
fn create_open_confirm_channel_event_test() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let message = MsgChannelOpenConfirm::try_from(default_raw_msg.clone()).unwrap();
    let event = create_open_confirm_channel_event(&message);

    assert_eq!(IbcEventType::OpenConfirmChannel.as_str(), event.ty);
    assert_eq!("channel-0", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
    assert_eq!("port_id", event.attributes[0].key);
}

#[test]
fn create_open_init_channel_event_test() {
    let default_raw_msg = get_dummy_raw_msg_chan_open_init(Some(10));
    let message = MsgChannelOpenInit::try_from(default_raw_msg.clone()).unwrap();
    let channel_id = ChannelId::new(10);
    let event = create_open_init_channel_event(&channel_id, &message);

    assert_eq!(IbcEventType::OpenInitChannel.as_str(), event.ty);
    assert_eq!("channel-10", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
    assert_eq!("version", event.attributes[4].key);
}

#[test]
fn create_open_try_channel_event_test() {
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(10);
    let message = MsgChannelOpenTry::try_from(default_raw_msg.clone()).unwrap();
    let channel_id = ChannelId::new(11);
    let event = create_open_try_channel_event(&channel_id, &message);

    assert_eq!(IbcEventType::OpenTryChannel.as_str(), event.ty);
    assert_eq!("counterparty_port_id", event.attributes[2].key);
    assert_eq!("channel-11", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
}

#[test]
fn test_create_send_packet_event() {
    let raw = get_dummy_raw_packet(15, 0);
    let msg = Packet::try_from(raw.clone()).unwrap();
    let raw_back = RawPacket::from(msg.clone());
    let msg_back = Packet::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
    let event = create_send_packet_event(msg_back, &Order::Ordered, &IbcConnectionId::default());
    assert_eq!(IbcEventType::SendPacket.as_str(), event.unwrap().ty)
}

#[test]
#[should_panic(expected = "NonUtf8PacketData")]
fn test_create_send_packet_event_fail() {
    let raw = get_dummy_raw_packet(15, 0);

    let raw = RawPacket {
        data: vec![u8::MAX],
        ..raw.clone()
    };
    let msg = Packet::try_from(raw.clone()).unwrap();
    let _event =
        create_send_packet_event(msg, &Order::Ordered, &IbcConnectionId::default()).unwrap();
}

#[test]
fn test_create_write_ack_packet_event() {
    let raw = get_dummy_raw_packet(15, 0);
    let msg = Packet::try_from(raw.clone()).unwrap();
    let raw_back = RawPacket::from(msg.clone());
    let msg_back = Packet::try_from(raw_back.clone()).unwrap();
    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
    let event = create_write_ack_event(msg_back, vec![0], &IbcConnectionId::default());
    assert_eq!(IbcEventType::WriteAck.as_str(), event.unwrap().ty)
}

#[test]
#[should_panic(expected = "NonUtf8PacketData")]
fn test_create_write_ack_packet_event_fail() {
    let raw = get_dummy_raw_packet(15, 0);

    let raw = RawPacket {
        data: vec![u8::MAX],
        ..raw.clone()
    };
    let msg = Packet::try_from(raw.clone()).unwrap();
    let _event =
        create_send_packet_event(msg, &Order::Ordered, &IbcConnectionId::default()).unwrap();
}

#[test]
fn test_create_ack_packet_event() {
    let raw = get_dummy_raw_packet(15, 0);
    let packet = Packet::try_from(raw.clone()).unwrap();
    let event = create_ack_packet_event(packet, &Order::Ordered, &IbcConnectionId::default());
    assert_eq!("acknowledge_packet", event.ty)
}

#[test]
fn test_create_timout_packet_event() {
    let raw = get_dummy_raw_packet(15, 0);
    let packet = Packet::try_from(raw.clone()).unwrap();
    let event = create_packet_timeout_event(packet, &Order::Ordered);
    assert_eq!("timeout_packet", event.ty)
}

#[test]
pub fn test_create_close_init_channel_event() {
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw.clone()).unwrap();
    let event = create_close_init_channel_event(&msg);

    assert_eq!(event.ty, IbcEventType::CloseInitChannel.as_str())
}

#[test]
pub fn test_create_close_confirm_channel_event() {
    let proof_height = 10;
    let raw = get_dummy_raw_msg_chan_close_confirm(proof_height);
    let msg = MsgChannelCloseConfirm::try_from(raw.clone()).unwrap();
    let event = create_close_confirm_channel_event(&msg);

    assert_eq!(event.ty, IbcEventType::CloseConfirmChannel.as_str())
}
