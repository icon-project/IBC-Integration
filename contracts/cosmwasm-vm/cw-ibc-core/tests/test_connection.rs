use std::time::Duration;

pub mod setup;
use cosmwasm_std::testing::MockStorage;
use setup::*;

use cw_ibc_core::context::CwIbcCoreContext;
use cw_ibc_core::types::ClientId;
use cw_ibc_core::types::ConnectionId;
use cw_ibc_core::ConnectionEnd;
use cw_ibc_core::IbcClientId;
use ibc::core::ics03_connection::connection::Counterparty;
use ibc::core::ics03_connection::connection::State;
use ibc::core::ics03_connection::version::Version;

#[test]
fn test_set_connection() {
    let mut deps = deps();
    let conn_end = ConnectionEnd::default();
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
        .unwrap();
    let result = contract
        .connection_end(deps.as_ref().storage, conn_id)
        .unwrap();

    assert_eq!(conn_end, result)
}

#[test]
fn test_get_connection() {
    let mut deps = deps();
    let ss = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );
    let counter_party = Counterparty::new(IbcClientId::default(), None, ss.unwrap());
    let conn_end = ConnectionEnd::new(
        State::Open,
        IbcClientId::default(),
        counter_party,
        vec![Version::default()],
        Duration::default(),
    );
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();
    contract
        .store_connection(deps.as_mut().storage, conn_id.clone(), conn_end.clone())
        .unwrap();
    let result = contract
        .connection_end(deps.as_ref().storage, conn_id)
        .unwrap();

    assert_eq!(conn_end, result)
}

#[test]
fn test_connection_sequence() {
    let mut store = MockStorage::default();
    let contract = CwIbcCoreContext::new();
    contract
        .connection_next_sequence_init(&mut store, u128::default())
        .unwrap();
    let result = contract.connection_counter(&mut store).unwrap();

    assert_eq!(0, result);

    let increment_sequence = contract.increase_connection_counter(&mut store).unwrap();
    assert_eq!(1, increment_sequence);
}

#[test]
fn test_client_connection() {
    let mut deps = deps();
    let client_id = ClientId::default();
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();

    contract
        .store_connection_to_client(deps.as_mut().storage, client_id.clone(), conn_id.clone())
        .unwrap();

    let result = contract
        .client_connection(deps.as_ref().storage, client_id)
        .unwrap();

    assert_eq!(conn_id, result)
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<u8>\" })")]
fn test_get_connection_fail() {
    let deps = deps();

    let conn_id = ConnectionId::new(5);
    let contract = CwIbcCoreContext::new();

    contract
        .connection_end(deps.as_ref().storage, conn_id)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"cw_ibc_core::types::ConnectionId\" })")]
fn test_client_connection_fail() {
    let deps = deps();
    let client_id = ClientId::default();

    let contract = CwIbcCoreContext::new();

    contract
        .client_connection(deps.as_ref().storage, client_id)
        .unwrap();
}
