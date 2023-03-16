use std::time::Duration;

use cosmwasm_std::testing::mock_dependencies;
use cosmwasm_std::testing::MockStorage;
use cosmwasm_std::Response;
use cw_ibc_core::state::CwIbcStore;
use cw_ibc_core::types::ClientId;
use cw_ibc_core::types::ConnectionId;
use cw_ibc_core::ConnectionEnd;
use cw_ibc_core::IbcClientId;
use ibc::core::ics03_connection::connection::Counterparty;
use ibc::core::ics03_connection::connection::State;
use ibc::core::ics03_connection::version::Version;

#[test]
fn test_set_connection() {
    let mut deps = mock_dependencies();
    let conn_end = ConnectionEnd::default();
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcStore::new();
    let actual_response = contract
        .set_connection(deps.as_mut(), conn_end, conn_id)
        .unwrap();
    let expected_response = Response::new().add_attribute("method", "set_connection");
    assert_eq!(actual_response, expected_response);
}

#[test]
fn test_get_connection() {
    let mut s = MockStorage::default();
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
    println!("{:?}", conn_id);
    let contract = CwIbcStore::new();
    let _result = contract.add_connection(&mut s, conn_end, conn_id.clone());
    let response = contract.query_connection(&mut s, conn_id);
    println!("{:?}", response);
}

#[test]
fn test_connection_sequence() {
    let mut store = MockStorage::default();
    let contract = CwIbcStore::new();
    contract
        .connection_next_sequence_init(&mut store, u128::default())
        .unwrap();
    contract.query_next_sequence(&mut store).unwrap();
    let result = contract.get_next_connection_sequence(&mut store, 1);
    assert_eq!(1, result);
    let increment_sequence = contract.increment_connection_sequence(&mut store);
    assert_eq!(2, increment_sequence);
}

#[test]
fn test_client_connection() {
    let mut store = MockStorage::default();
    let client_id = ClientId::default();
    let conn_id = ConnectionId::new(5);
    let contract = CwIbcStore::new();
    contract
        .client_connections()
        .save(&mut store, client_id.clone(), &conn_id)
        .unwrap();
    let actual_result = contract
        .store_connection_to_client(&mut store, client_id, conn_id)
        .unwrap();
    let expected_result = Response::new().add_attribute("method", "store_connection_to_client");
    assert_eq!(actual_result, expected_result);
}
