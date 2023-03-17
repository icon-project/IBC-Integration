use cw_ibc_core::{
    context::CwIbcCoreContext,
    types::{ChannelId, PortId},
    ChannelEnd, Sequence,
};
use ibc::core::ics04_channel::{
    channel::{Counterparty, Order, State},
    Version,
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
