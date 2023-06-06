use std::{str::FromStr, time::Duration};

use common::traits::AnyTypes;
use cosmwasm_std::{
    to_binary, Addr, Event, IbcEndpoint, IbcPacket, IbcPacketReceiveMsg, IbcTimeout,
    IbcTimeoutBlock, Reply, SubMsgResponse, SubMsgResult,
};

use common::icon::icon::lightclient::v1::{ClientState, ConsensusState};

use common::ibc::core::ics24_host::identifier::{ChannelId, ConnectionId, PortId};
use common::ibc::{
    core::ics04_channel::{
        channel::{Counterparty, Order, State},
        packet::Packet,
        Version,
    },
    events::IbcEventType,
    signer::Signer,
};
use cw_common::ibc_types::{IbcClientId, IbcConnectionId, IbcPortId};
use cw_common::raw_types::channel::{
    RawMsgChannelCloseConfirm, RawMsgChannelCloseInit, RawMsgChannelOpenAck,
    RawMsgChannelOpenConfirm, RawMsgChannelOpenInit, RawMsgChannelOpenTry, RawPacket,
};
use cw_common::raw_types::RawHeight;

use cw_ibc_core::ics04_channel::open_init::{
    create_channel_submesssage, on_chan_open_init_submessage,
};
use cw_ibc_core::ics04_channel::open_try::on_chan_open_try_submessage;
use cw_ibc_core::ics04_channel::{EXECUTE_ON_CHANNEL_OPEN_INIT, EXECUTE_ON_CHANNEL_OPEN_TRY};
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
    ChannelEnd, ConnectionEnd, Sequence,
};
use cw_ibc_core::{traits::*, IbcClientType};

pub mod channel;
pub mod setup;
use prost::Message;
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
    let _store = ctx.init_channel_counter(mock_deps.as_mut().storage, u64::default());
    let result = ctx.channel_counter(mock_deps.as_ref().storage);

    assert_eq!(0, result.unwrap());

    let incremented_result = ctx.increase_channel_sequence(mock_deps.as_mut().storage);
    assert_eq!(1, incremented_result.unwrap());
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u64\" })")]
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
    let sequence = Sequence::from(6);
    let mut mock_deps = deps();

    let _store = ctx.store_next_sequence_send(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        sequence,
    );
    let result = ctx.get_next_sequence_send(mock_deps.as_ref().storage, port_id, channel_id);

    assert_eq!(sequence, result.unwrap())
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

    let incremented_result =
        ctx.increase_next_sequence_send(mock_deps.as_mut().storage, port_id, channel_id);
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

    let incremented_result =
        ctx.increase_next_sequence_recv(mock_deps.as_mut().storage, port_id, channel_id);
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

    let incremented_result =
        ctx.increase_next_sequence_ack(mock_deps.as_mut().storage, port_id, channel_id);
    assert_eq!(Sequence::from(1), incremented_result.unwrap());
}

#[test]
#[should_panic(expected = "MissingNextAckSeq")]
fn test_channel_sequence_ack_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    ctx.increase_next_sequence_ack(mock_deps.as_mut().storage, port_id, channel_id)
        .unwrap();
}

#[test]
#[should_panic(expected = "IbcPacketError")]
fn test_channel_sequence_send_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    ctx.increase_next_sequence_send(mock_deps.as_mut().storage, port_id, channel_id)
        .unwrap();
}

#[test]
#[should_panic(expected = "IbcPacketError")]
fn test_channel_sequence_recv_fail() {
    let ctx = CwIbcCoreContext::new();
    let mut mock_deps = deps();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    ctx.increase_next_sequence_recv(mock_deps.as_mut().storage, port_id, channel_id)
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
    let res_msg = MsgChannelOpenInit::try_from(default_raw_init_msg);
    assert_eq!(res_msg.is_ok(), true)
}
#[test]
#[should_panic(expected = "Identifier(ContainSeparator { id: \"p34/\" })")]
fn channel_open_init_from_raw_incorrect_port_id_parameter() {
    let default_raw_init_msg = get_dummy_raw_msg_chan_open_init(None);
    let default_raw_init_msg = RawMsgChannelOpenInit {
        port_id: "p34/".to_string(),
        ..default_raw_init_msg
    };
    let res_msg = MsgChannelOpenInit::try_from(default_raw_init_msg);
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
    let res_msg = MsgChannelOpenInit::try_from(default_raw_init_msg);
    res_msg.unwrap();
}
#[test]
fn channel_open_confirm_from_raw_good_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_msg);
    assert_eq!(res_msg.is_ok(), true)
}
#[test]
#[should_panic(expected = "Identifier(ContainSeparator { id: \"p34/\" })")]
fn channel_open_confirm_from_raw_incorrect_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let default_raw_confirm_msg = RawMsgChannelOpenConfirm {
        port_id: "p34/".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg);
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
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg);
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_open_confirm_from_raw_missing_proof_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let default_raw_confirm_msg = RawMsgChannelOpenConfirm {
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: 0,
        }),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg);
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
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg);
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
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg);
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
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg);
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
    let res_msg = MsgChannelOpenConfirm::try_from(default_raw_confirm_msg);

    let expected = MsgChannelOpenConfirm {
        port_id_on_b: PortId::default(),
        chan_id_on_b: ChannelId::new(34),
        proof_chan_end_on_a:
            common::ibc::core::ics23_commitment::commitment::CommitmentProofBytes::try_from(
                get_dummy_proof(),
            )
            .unwrap(),
        proof_height_on_a: common::ibc::core::ics02_client::height::Height::new(0, proof_height)
            .unwrap(),
        signer: Signer::from_str("cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng").unwrap(),
    };
    assert_eq!(res_msg.unwrap(), expected);
}

#[test]
fn channel_open_try_from_raw_good_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(proof_height);
    let res_msg = MsgChannelOpenTry::try_from(default_raw_msg);
    assert_eq!(res_msg.is_ok(), true)
}
#[test]
#[should_panic(expected = "Identifier(ContainSeparator { id: \"p34/\" })")]
fn channel_open_try_from_raw_incorrect_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(proof_height);
    let default_raw_try_msg = RawMsgChannelOpenTry {
        port_id: "p34/".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg);
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
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg);
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_open_try_from_raw_missing_proof_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(proof_height);
    let default_raw_try_msg = RawMsgChannelOpenTry {
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: 0,
        }),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg);
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
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg);
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
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg);
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
    let res_msg = MsgChannelOpenTry::try_from(default_raw_try_msg);
    res_msg.unwrap();
}

#[test]
fn channel_open_ack_from_raw_good_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let res_msg = MsgChannelOpenAck::try_from(default_raw_msg);
    assert_eq!(res_msg.is_ok(), true)
}
#[test]
#[should_panic(expected = "Identifier(ContainSeparator { id: \"p34/\" })")]
fn channel_open_ack_from_raw_incorrect_port_id_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let default_raw_ack_msg = RawMsgChannelOpenAck {
        port_id: "p34/".to_string(),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg);
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
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg);
    res_msg.unwrap();
}
#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_open_ack_from_raw_missing_proof_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let default_raw_ack_msg = RawMsgChannelOpenAck {
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: 0,
        }),
        ..default_raw_msg
    };
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg);
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
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg);
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
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg);
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
    let res_msg = MsgChannelOpenAck::try_from(default_raw_ack_msg);
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
    let res_msg = MsgChannelCloseInit::try_from(default_raw_confirm_msg);

    let expected = MsgChannelCloseInit {
        port_id_on_a: PortId::default(),
        chan_id_on_a: ChannelId::new(34),
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
    let res_msg = MsgChannelCloseInit::try_from(default_raw_ack_msg);
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
    let res_msg = MsgChannelCloseInit::try_from(default_raw_ack_msg);
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
    let res_msg = MsgChannelCloseConfirm::try_from(default_raw_ack_msg);
    res_msg.unwrap();
}

#[test]
#[should_panic(expected = "MissingHeight")]
fn channel_close_confirm_from_raw_missing_height_parameter() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_close_confirm(proof_height);
    let default_raw_ack_msg = RawMsgChannelCloseConfirm {
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: 0,
        }),
        ..default_raw_msg
    };
    let res_msg = MsgChannelCloseConfirm::try_from(default_raw_ack_msg);
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
    let res_msg = MsgChannelCloseConfirm::try_from(default_raw_ack_msg);
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
    let res_msg = MsgChannelCloseConfirm::try_from(default_raw_confirm_msg);

    let expected = MsgChannelCloseConfirm {
        port_id_on_b: PortId::default(),
        chan_id_on_b: ChannelId::new(34),
        proof_chan_end_on_a:
            common::ibc::core::ics23_commitment::commitment::CommitmentProofBytes::try_from(
                get_dummy_proof(),
            )
            .unwrap(),
        proof_height_on_a: common::ibc::core::ics02_client::height::Height::new(0, proof_height)
            .unwrap(),
        signer: Signer::from_str("cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng").unwrap(),
    };
    assert_eq!(res_msg.unwrap(), expected);
}

#[test]
fn create_open_ack_channel_event_test() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_ack(proof_height);
    let message = MsgChannelOpenAck::try_from(default_raw_msg).unwrap();
    let event = create_open_ack_channel_event(
        message.port_id_on_a.as_str(),
        message.chan_id_on_a.as_str(),
        IbcPortId::default().as_str(),
        message.chan_id_on_b.as_str(),
        ConnectionId::default().as_str(),
    );

    assert_eq!(IbcEventType::OpenAckChannel.as_str(), event.ty);
    assert_eq!("channel-0", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
    assert_eq!("channel_id", event.attributes[1].key);
}

#[test]
fn create_open_confirm_channel_event_test() {
    let proof_height = 10;
    let default_raw_msg = get_dummy_raw_msg_chan_open_confirm(proof_height);
    let message = MsgChannelOpenConfirm::try_from(default_raw_msg).unwrap();
    let event = create_open_confirm_channel_event(
        message.port_id_on_b.as_str(),
        message.chan_id_on_b.as_str(),
        PortId::default().as_str(),
        ChannelId::default().as_str(),
        ConnectionId::default().as_str(),
    );

    assert_eq!(IbcEventType::OpenConfirmChannel.as_str(), event.ty);
    assert_eq!("channel-0", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
    assert_eq!("port_id", event.attributes[0].key);
}

#[test]
fn create_open_init_channel_event_test() {
    let default_raw_msg = get_dummy_raw_msg_chan_open_init(Some(10));
    let message = MsgChannelOpenInit::try_from(default_raw_msg).unwrap();
    let channel_id = ChannelId::new(10);
    let event = create_open_init_channel_event(
        &channel_id,
        &message.port_id_on_a,
        &message.port_id_on_a,
        &message.connection_hops_on_a[0],
        &message.version_proposal,
    );

    assert_eq!(IbcEventType::OpenInitChannel.as_str(), event.ty);
    assert_eq!("channel-10", event.attributes[1].value);
    assert_eq!("defaultPort", event.attributes[0].value);
    assert_eq!("version", event.attributes[4].key);
}

#[test]
fn create_open_try_channel_event_test() {
    let default_raw_msg = get_dummy_raw_msg_chan_open_try(10);
    let message = MsgChannelOpenTry::try_from(default_raw_msg).unwrap();
    let channel_id = ChannelId::new(11);
    let event = create_open_try_channel_event(
        channel_id.as_str(),
        message.port_id_on_b.as_str(),
        message.port_id_on_a.as_str(),
        message.chan_id_on_a.as_str(),
        message.connection_hops_on_b[0].as_str(),
        message.version_supported_on_a.as_str(),
    );

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
        ..raw
    };
    let msg = Packet::try_from(raw).unwrap();
    let _event =
        create_send_packet_event(msg, &Order::Ordered, &IbcConnectionId::default()).unwrap();
}

#[test]
fn test_create_write_ack_packet_event() {
    let raw = get_dummy_raw_packet(15, 0);
    let msg = Packet::try_from(raw.clone()).unwrap();
    let raw_back = RawPacket::from(msg.clone());
    let msg_back = Packet::try_from(raw_back.clone()).unwrap();
    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 10,
    };
    let timeout = IbcTimeout::with_both(timeout_block, cosmwasm_std::Timestamp::from_nanos(100));
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let packet = IbcPacket::new(vec![0, 1, 2, 3], src, dst, 1, timeout);
    let ibc_packet_recv_message = IbcPacketReceiveMsg::new(packet, Addr::unchecked("relayer"));

    assert_eq!(raw, raw_back);
    assert_eq!(msg, msg_back);
    let event = create_write_ack_event(
        ibc_packet_recv_message.packet,
        Order::Unordered.as_str(),
        IbcConnectionId::default().as_str(),
        &vec![],
    );
    assert_eq!(IbcEventType::WriteAck.as_str(), event.unwrap().ty)
}

#[test]
#[should_panic(expected = "NonUtf8PacketData")]
fn test_create_write_ack_packet_event_fail() {
    let raw = get_dummy_raw_packet(15, 0);

    let raw = RawPacket {
        data: vec![u8::MAX],
        ..raw
    };
    let msg = Packet::try_from(raw).unwrap();
    let _event =
        create_send_packet_event(msg, &Order::Ordered, &IbcConnectionId::default()).unwrap();
}

#[test]
fn test_create_ack_packet_event() {
    let raw = get_dummy_raw_packet(15, 0);
    let packet = Packet::try_from(raw).unwrap();
    let event = create_ack_packet_event(
        packet.port_id_on_a.as_str(),
        packet.chan_id_on_a.as_str(),
        &packet.sequence.to_string(),
        packet.port_id_on_b.as_str(),
        packet.chan_id_on_b.as_str(),
        &packet.timeout_height_on_b.to_string(),
        &packet.timeout_timestamp_on_b.to_string(),
        Order::Ordered.as_str(),
        IbcConnectionId::default().as_str(),
    );
    assert_eq!("acknowledge_packet", event.ty)
}

#[test]
fn test_create_timout_packet_event() {
    let raw = get_dummy_raw_packet(15, 0);
    let packet = Packet::try_from(raw).unwrap();
    let event = create_packet_timeout_event(packet, &Order::Ordered);
    assert_eq!("timeout_packet", event.ty)
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_validate_open_init_channel_fail_missing_connection_end() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_init(None);
    let msg = MsgChannelOpenInit::try_from(raw).unwrap();

    contract
        .validate_channel_open_init(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
pub fn test_create_close_init_channel_event() {
    let raw = get_dummy_raw_msg_chan_close_init();
    let msg = MsgChannelCloseInit::try_from(raw).unwrap();
    let event =
        create_close_init_channel_event(msg.port_id_on_a.as_str(), msg.chan_id_on_a.as_str());

    assert_eq!(event.ty, IbcEventType::CloseInitChannel.as_str())
}

#[test]
pub fn test_create_close_confirm_channel_event() {
    let proof_height = 10;
    let raw = get_dummy_raw_msg_chan_close_confirm(proof_height);
    let msg = MsgChannelCloseConfirm::try_from(raw).unwrap();
    let event =
        create_close_confirm_channel_event(msg.port_id_on_b.as_str(), msg.chan_id_on_b.as_str());

    assert_eq!(event.ty, IbcEventType::CloseConfirmChannel.as_str())
}

#[test]
fn test_validate_open_init_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_init(None);
    let mut msg = MsgChannelOpenInit::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_a.clone();
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();

    let module = Addr::unchecked("contractaddress");
    let cx_module_id = module_id;
    contract
        .add_route(&mut deps.storage, cx_module_id, &module)
        .unwrap();

    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    msg.connection_hops_on_a = vec![conn_id.clone()];
    msg.version_proposal = Version::from_str("xcall-1").unwrap();
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end)
        .unwrap();

    let res = contract.validate_channel_open_init(deps.as_mut(), info.clone(), &msg);

    let channel_id_expect = ChannelId::new(0);
    let expected = on_chan_open_init_submessage(&msg, &channel_id_expect, &conn_id);
    let data = cw_common::xcall_msg::ExecuteMsg::IbcChannelOpen { msg: expected };
    let data = to_binary(&data).unwrap();
    let on_chan_open_init = create_channel_submesssage(
        "contractaddress".to_string(),
        data,
        info.funds,
        EXECUTE_ON_CHANNEL_OPEN_INIT,
    );

    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap().messages[0], on_chan_open_init)
}

#[test]
#[should_panic(expected = "error: UnknownPort { port_id: PortId(\"defaultPort\")")]
fn test_validate_open_init_channel_fail_missing_module_id() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_init(None);
    let mut msg = MsgChannelOpenInit::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    msg.connection_hops_on_a = vec![conn_id.clone()];
    msg.version_proposal = Version::from_str("xcall-1").unwrap();
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id, conn_end)
        .unwrap();

    contract
        .validate_channel_open_init(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_validate_open_try_channel_fail_missing_connection_end() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let msg = MsgChannelOpenTry::try_from(raw).unwrap();

    contract
        .validate_channel_open_try(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
fn test_validate_open_try_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 20000000);
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let mut msg = MsgChannelOpenTry::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let module_id = common::ibc::core::ics26_routing::context::ModuleId::from_str("xcall").unwrap();
    let port_id = msg.port_id_on_a.clone();
    contract
        .store_module_by_port(&mut deps.storage, port_id, module_id.clone())
        .unwrap();

    let module = Addr::unchecked("contractaddress");
    let cx_module_id = module_id;
    contract
        .add_route(&mut deps.storage, cx_module_id, &module)
        .unwrap();

    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let connection_id = IbcConnectionId::new(5);
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        Some(connection_id),
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    msg.connection_hops_on_b = vec![conn_id.clone()];
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id, conn_end)
        .unwrap();

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let client = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(&mut deps.storage, &IbcClientId::default(), client)
        .unwrap();
    let client_type = IbcClientType::new("iconclient".to_string());

    contract
        .store_client_into_registry(
            &mut deps.storage,
            client_type,
            "contractaddress".to_string(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = msg.proof_height_on_a;
    let consenus_state = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state,
        )
        .unwrap();

    let res = contract.validate_channel_open_try(deps.as_mut(), info, &msg);

    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap().messages[0].id, 421)
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_validate_open_try_channel_fail_missing_client_state() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let mut msg = MsgChannelOpenTry::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());
    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    msg.connection_hops_on_b = vec![conn_id.clone()];
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id, conn_end)
        .unwrap();

    contract
        .validate_channel_open_try(deps.as_mut(), info, &msg)
        .unwrap();
}

#[test]
fn test_execute_open_try_channel() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let mut msg = MsgChannelOpenTry::try_from(raw).unwrap();
    let _store = contract.init_channel_counter(deps.as_mut().storage, u64::default());

    let ss = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = common::ibc::core::ics03_connection::connection::Counterparty::new(
        IbcClientId::default(),
        None,
        ss.unwrap(),
    );
    let conn_end = ConnectionEnd::new(
        common::ibc::core::ics03_connection::connection::State::Open,
        IbcClientId::default(),
        counter_party,
        vec![common::ibc::core::ics03_connection::version::Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    msg.connection_hops_on_b = vec![conn_id.clone()];
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id, conn_end)
        .unwrap();

    let counter_party = Counterparty::new(msg.port_id_on_a.clone(), Some(msg.chan_id_on_a.clone()));
    let channel_id_on_b = ChannelId::new(0); // creating new channel_id
    let channel_end = ChannelEnd::new(
        State::Uninitialized,
        msg.ordering,
        counter_party,
        msg.connection_hops_on_b.clone(),
        msg.version_supported_on_a.clone(),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            msg.port_id_on_b.clone(),
            channel_id_on_b.clone(),
            channel_end,
        )
        .unwrap();

    let expected_data = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_b.to_string(),
        channel_id: channel_id_on_b.to_string(),
    };

    let response = SubMsgResponse {
        data: Some(to_binary(&expected_data).unwrap()),
        events: vec![Event::new("action").add_attribute("action", "channel open try execution")],
    };
    let result: SubMsgResult = SubMsgResult::Ok(response);
    let reply = Reply {
        id: EXECUTE_ON_CHANNEL_OPEN_TRY,
        result,
    };

    let result = contract.execute_channel_open_try(deps.as_mut(), reply);
    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap().events[0].ty, "channel_id_created");
    assert_eq!(result.unwrap().events[1].ty, "channel_open_try")
}

#[test]
fn test_get_channel() {
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
    ctx.store_channel_end(
        mock_deps.as_mut().storage,
        port_id.clone(),
        channel_id.clone(),
        channel_end.clone(),
    )
    .unwrap();
    let retrived_channel_end = ctx.get_channel_end(mock_deps.as_ref().storage, port_id, channel_id);

    assert_eq!(channel_end, retrived_channel_end.unwrap())
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_get_channel_fail() {
    let ctx = CwIbcCoreContext::new();
    let port_id = PortId::default();
    let channel_id = ChannelId::default();
    let mock_deps = deps();
    ctx.get_channel_end(mock_deps.as_ref().storage, port_id, channel_id)
        .unwrap();
}
