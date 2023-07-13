use cosmwasm_std::{
    coin,
    testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR},
    Coin,
};
use cw_xcall::state::CwCallService;
pub mod account;
use account::*;

#[test]
fn set_protocol_fee_handler() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    let info = mock_info("user", &[Coin::new(1000, "uconst")]);
    let address = "xyz".to_string();

    let contract = CwCallService::new();
    contract
        .add_owner(deps.as_mut().storage, info.sender.to_string())
        .unwrap();

    contract
        .add_admin(deps.as_mut().storage, &info, admin_one().to_string())
        .unwrap();

    contract
        .fee_handler()
        .save(&mut deps.storage, &address)
        .unwrap();

    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "uconst")]);

    let balance = vec![coin(123, "uconst"), coin(777, "ucosm")];
    deps.querier
        .update_balance(MOCK_CONTRACT_ADDR.to_string(), balance);

    let response = contract
        .set_protocol_feehandler(deps.as_mut(), &env, &info, address.clone())
        .unwrap();
    match response.messages[0].msg.clone() {
        cosmwasm_std::CosmosMsg::Bank(bank_msg) => match bank_msg {
            cosmwasm_std::BankMsg::Send { to_address, amount } => {
                assert_eq!(to_address, address.to_string());
                assert_eq!(amount[0].amount.u128(), 123)
            }
            _ => todo!(),
        },
        _ => todo!(),
    };
}

#[test]
#[should_panic(expected = "OnlyAdmin")]
fn test_invalid_input() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("user", &[Coin::new(1000, "ucosm")]);
    let address = "xyz".to_string();
    let cw_callservice = CwCallService::new();
    cw_callservice
        .add_owner(deps.as_mut().storage, info.sender.to_string())
        .unwrap();

    cw_callservice
        .add_admin(deps.as_mut().storage, &info, admin_one().to_string())
        .unwrap();

    cw_callservice
        .set_protocol_feehandler(deps.as_mut(), &env, &info, address)
        .unwrap();
}

#[test]
fn get_protocol_fee_handler() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("user", &[Coin::new(1000, "ucosm")]);
    let address = "xyz".to_string();

    let contract = CwCallService::new();
    contract
        .add_owner(deps.as_mut().storage, info.sender.to_string())
        .unwrap();

    contract
        .add_admin(deps.as_mut().storage, &info, admin_one().to_string())
        .unwrap();
    contract
        .fee_handler()
        .save(&mut deps.storage, &address)
        .unwrap();
    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "ucosm")]);
    contract
        .set_protocol_feehandler(deps.as_mut(), &env, &info, address)
        .unwrap();
    let result = contract.get_protocol_feehandler(deps.as_ref());
    assert_eq!("xyz", result);
}

#[test]
fn set_protocol_fee() {
    let mut deps = mock_dependencies();
    let value = 123;
    let info = mock_info("user", &[Coin::new(1000, "uconst")]);
    let contract = CwCallService::new();

    contract
        .add_owner(deps.as_mut().storage, info.sender.to_string())
        .unwrap();

    contract
        .add_admin(deps.as_mut().storage, &info, admin_one().to_string())
        .unwrap();

    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "uconst")]);
    let result = contract.set_protocol_fee(deps.as_mut(), info, value);
    assert_eq!(result.unwrap().attributes.len(), 1)
}

#[test]
fn get_protocol_fee() {
    let mut deps = mock_dependencies();
    let info = mock_info("user", &[Coin::new(1000, "ucosm")]);
    let value = 123;

    let contract = CwCallService::new();
    contract
        .add_owner(deps.as_mut().storage, info.sender.to_string())
        .unwrap();

    contract
        .add_admin(deps.as_mut().storage, &info, admin_one().to_string())
        .unwrap();
    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "ucosm")]);
    contract
        .set_protocol_fee(deps.as_mut(), info, value)
        .unwrap();
    let result = contract.get_protocol_fee(deps.as_ref().storage);
    assert_eq!("123", result.to_string());
}
