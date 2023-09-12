mod account;
mod setup;
use account::*;
use cosmwasm_std::{testing::mock_env, Addr};
use cw_common::xcall_connection_msg::{ExecuteMsg, QueryMsg};
use cw_xcall_ibc_connection::{execute, state::CwIbcConnection};
use setup::*;

#[test]
fn add_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    let response = contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info,
            Addr::unchecked(Addr::unchecked(admin_one().to_string())),
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "add_admin");

    assert_eq!(
        response.attributes[1].value,
        Addr::unchecked(admin_one().to_string())
    );

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, Addr::unchecked(admin_one().to_string()))
}

#[test]
#[should_panic(expected = "OnlyAdmin")]
fn update_admin_unauthorzied() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info,
            Addr::unchecked(Addr::unchecked(admin_one().to_string())),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, Addr::unchecked(admin_one().to_string()));

    let mock_info = create_mock_info(&bob().to_string(), "umlg", 2000);

    let execute_msg = ExecuteMsg::SetAdmin {
        address: admin_one().to_string(),
    };

    execute(mock_deps.as_mut(), mock_env(), mock_info, execute_msg).unwrap();
}

#[test]
fn update_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&admin_one().to_string(), "umlg", 2000);

    let contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, Addr::unchecked(admin_one().to_string()));

    let mock_info = create_mock_info(&admin_one().to_string(), "umlg", 2000);

    let execute_msg = ExecuteMsg::SetAdmin {
        address: admin_two().to_string(),
    };

    execute(mock_deps.as_mut(), mock_env(), mock_info, execute_msg).unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_two().to_string());
}

#[test]
fn query_admin() {
    let mock_deps = deps();

    let mock_env = mock_env();

    let contract = CwIbcConnection::default();

    let result = contract.query(mock_deps.as_ref(), mock_env, QueryMsg::GetAdmin {});

    assert!(result.is_err())
}

#[test]
#[should_panic(expected = "InvalidAddress { address: \"*************\"")]
fn add_invalid_char_as_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info,
            cw_common::xcall_connection_msg::ExecuteMsg::SetAdmin {
                address: "*************".into(),
            },
        )
        .unwrap();
}

#[test]
#[should_panic(
    expected = "Std(GenericErr { msg: \"Invalid input: human address too short for this mock implementation (must be >= 3).\" })"
)]
fn validate_address_add_admin_size_lessthan_3() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info,
            cw_common::xcall_connection_msg::ExecuteMsg::SetAdmin {
                address: "sm".into(),
            },
        )
        .unwrap();
}

#[test]
#[should_panic(
    expected = "Std(GenericErr { msg: \"Invalid input: human address too long for this mock implementation (must be <= 90).\" })"
)]
fn validate_address_add_admin_size_more_than_45() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info,
            cw_common::xcall_connection_msg::ExecuteMsg::SetAdmin {
                address: "eddiuo6lbp05golmz3rb5n7hbi4c5hhyh0rb1w6cslyjt5mhwd0chn3x254lyorpx4dzvrvsc9h2em44be2rj193dwe".into(),
            },
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidAddress { address: \"new_addmin!@234\" }")]
fn update_admin_fails() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwIbcConnection::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.clone().sender)
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, Addr::unchecked(admin_one().to_string()));

    let execute_msg = ExecuteMsg::SetAdmin {
        address: "new_addmin!@234".into(),
    };

    execute(mock_deps.as_mut(), mock_env(), mock_info, execute_msg).unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_two().to_string());
}
