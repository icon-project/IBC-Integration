mod account;
mod setup;
use account::*;

use cosmwasm_std::Addr;
use cw_xcall_ibc_connection::state::CwIbcConnection;
use setup::*;
#[test]
fn add_owner() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    let result = contract.query_owner(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, mock_info.sender.to_string())
}

#[test]
#[should_panic(expected = "OwnerAlreadyExist")]
fn add_existing_owner() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    let result = contract.query_owner(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, mock_info.sender.to_string());

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();
}
