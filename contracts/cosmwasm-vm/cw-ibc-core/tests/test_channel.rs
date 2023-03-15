use cosmwasm_std::{testing::MockStorage, StdError};
use cw_ibc_core::{
    state::CwIbcStore,
    types::{ChannelId, PortId},
    ChannelEnd, ContractError, Sequence,
};
use ibc::core::ics04_channel::{
    channel::{Counterparty, Order, State},
    Version,
};

#[test]
fn test_add_channel() {
    let ctx = CwIbcStore::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let channel_end = ChannelEnd::new(
        State::Init,
        Order::None,
        Counterparty::default(),
        Vec::default(),
        Version::from("ics-20".to_string()),
    );
    let mut storage = MockStorage::default();

    let _storing = ctx.add_channel_end(
        &mut storage,
        port_id.clone(),
        channel_id.clone(),
        channel_end.clone(),
    );

    let retrived_channel_end = ctx.get_channel_end(&mut storage, port_id, channel_id);

    assert_eq!(channel_end, retrived_channel_end.unwrap())
}

#[test]
fn test_channel_sequence_initialisation() {
    let ctx = CwIbcStore::default();
    let mut store = MockStorage::default();
    let _store = ctx.init_next_channel_sequence(&mut store, u128::default());
    let result = ctx.query_channel_sequence(&mut store);

    assert_eq!(0, result.unwrap());

    let incremented_result = ctx.increment_channel_sequence(&mut store);
    assert_eq!(1, incremented_result.unwrap());
}

#[test]
fn test_channel_sequence_fail() {
    let ctx = CwIbcStore::default();
    let mut store = MockStorage::default();
    let result = ctx.increment_channel_sequence(&mut store);

    assert_eq!(
        result,
        Err(ContractError::from(StdError::NotFound {
            kind: "u128".to_string()
        }))
    )
}

#[test]
fn test_channel_sequence_send() {
    let ctx = CwIbcStore::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let sequene = Sequence::from(6);
    let mut store = MockStorage::default();

    let _store =
        ctx.store_next_sequence_send(&mut store, port_id.clone(), channel_id.clone(), sequene);
    let result = ctx.query_next_sequence_send(&mut store, port_id, channel_id);

    assert_eq!(sequene, result.unwrap())
}

#[test]
fn test_channel_sequence_send_increment() {
    let ctx = CwIbcStore::default();
    let mut store = MockStorage::default();
    let sequence = Sequence::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let _store =
        ctx.store_next_sequence_send(&mut store, port_id.clone(), channel_id.clone(), sequence);
    let result = ctx.query_next_sequence_send(&mut store, port_id.clone(), channel_id.clone());

    assert_eq!(sequence, result.unwrap());

    let incremented_result =
        ctx.increment_next_sequence_send(&mut store, port_id.clone(), channel_id.clone());
    assert_eq!(Sequence::from(1), incremented_result.unwrap());
}

#[test]
fn test_channel_sequence_recv_increment() {
    let ctx = CwIbcStore::default();
    let mut store = MockStorage::default();
    let sequence = Sequence::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let _store =
        ctx.store_next_sequence_recv(&mut store, port_id.clone(), channel_id.clone(), sequence);
    let result = ctx.query_next_sequence_recv(&mut store, port_id.clone(), channel_id.clone());

    assert_eq!(sequence, result.unwrap());

    let incremented_result =
        ctx.increment_next_sequence_recv(&mut store, port_id.clone(), channel_id.clone());
    assert_eq!(Sequence::from(1), incremented_result.unwrap());
}

#[test]
fn test_channel_sequence_ack_increment() {
    let ctx = CwIbcStore::default();
    let mut store = MockStorage::default();
    let sequence = Sequence::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let _store =
        ctx.store_next_sequence_ack(&mut store, port_id.clone(), channel_id.clone(), sequence);
    let result = ctx.query_next_sequence_ack(&mut store, port_id.clone(), channel_id.clone());

    assert_eq!(sequence, result.unwrap());

    let incremented_result =
        ctx.increment_next_sequence_ack(&mut store, port_id.clone(), channel_id.clone());
    assert_eq!(Sequence::from(1), incremented_result.unwrap());
}

#[test]
fn test_channel_sequence_ack_fail() {
    let ctx = CwIbcStore::default();
    let mut store = MockStorage::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let result = ctx.increment_next_sequence_ack(&mut store, port_id.clone(), channel_id.clone());

    assert_eq!(
        result,
        Err(ContractError::MissingNextAckSeq {
            port_id,
            channel_id
        })
    )
}

#[test]
fn test_channel_sequence_send_fail() {
    let ctx = CwIbcStore::default();
    let mut store = MockStorage::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let result = ctx.increment_next_sequence_send(&mut store, port_id.clone(), channel_id.clone());

    assert_eq!(
        result,
        Err(ContractError::MissingNextSendSeq {
            port_id,
            channel_id
        })
    )
}

#[test]
fn test_channel_sequence_recv_fail() {
    let ctx = CwIbcStore::default();
    let mut store = MockStorage::default();
    let port_id = PortId::dafault();
    let channel_id = ChannelId::default();
    let result = ctx.increment_next_sequence_recv(&mut store, port_id.clone(), channel_id.clone());

    assert_eq!(
        result,
        Err(ContractError::MissingNextRecvSeq {
            port_id,
            channel_id
        })
    )
}
