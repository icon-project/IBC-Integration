use cosmwasm_std::{
    testing::{mock_dependencies, mock_info},
    Addr, Coin,
};
use cw_xcall::state::CwCallService;
pub mod account;
use account::*;

#[test]
fn set_protocol_fee_handler() {
    let mut deps = mock_dependencies();
    let address = "xyz".to_string();

    let contract = CwCallService::new();

    contract
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    contract
        .fee_handler()
        .save(&mut deps.storage, &address)
        .unwrap();

    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "uconst")]);

    contract
        .set_protocol_feehandler(deps.as_mut(), &info, address.clone())
        .unwrap();

    let result = contract.get_protocol_feehandler(deps.as_ref());
    assert_eq!(address, result);
}

#[test]
#[should_panic(expected = "OnlyAdmin")]
fn test_invalid_input() {
    let mut deps = mock_dependencies();
    let info = mock_info("user", &[Coin::new(1000, "ucosm")]);
    let address = "xyz".to_string();
    let cw_callservice = CwCallService::new();

    cw_callservice
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    let result = cw_callservice.query_admin(deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    cw_callservice
        .set_protocol_feehandler(deps.as_mut(), &info, address)
        .unwrap();
}

#[test]
fn get_protocol_fee_handler() {
    let mut deps = mock_dependencies();
    let address = "xyz".to_string();

    let contract = CwCallService::new();

    contract
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();
    contract
        .fee_handler()
        .save(&mut deps.storage, &address)
        .unwrap();
    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "ucosm")]);
    contract
        .set_protocol_feehandler(deps.as_mut(), &info, address)
        .unwrap();
    let result = contract.get_protocol_feehandler(deps.as_ref());
    assert_eq!("xyz", result);
}

#[test]
fn set_protocol_fee() {
    let mut deps = mock_dependencies();
    let value = 123;
    let contract = CwCallService::new();

    contract
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "uconst")]);
    let result = contract.set_protocol_fee(deps.as_mut(), info, value);
    assert_eq!(result.unwrap().attributes.len(), 1)
}

#[test]
fn get_protocol_fee() {
    let mut deps = mock_dependencies();
    let value = 123;
    let contract = CwCallService::new();

    contract
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();
    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "ucosm")]);
    contract
        .set_protocol_fee(deps.as_mut(), info, value)
        .unwrap();
    let result = contract.get_protocol_fee(deps.as_ref().storage);
    assert_eq!("123", result.to_string());
}
