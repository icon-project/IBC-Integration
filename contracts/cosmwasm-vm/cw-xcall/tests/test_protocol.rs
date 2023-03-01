use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    to_binary, Coin, IbcEndpoint, IbcMsg, StdError,
};
use cw_xcall::{
    state::{CwCallservice, IbcConfig},
    types::address::Address,
};
pub mod account;
use account::*;
#[test]
fn test_valid_input() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("user", &[Coin::new(1000, "ucosm")]);
    let address = Address::from("xyz");

    let contract = CwCallservice::new();
    contract
        .add_owner(
            deps.as_mut().storage,
            Address::from(&info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(deps.as_mut().storage, info.clone(), admin_one())
        .unwrap();
    contract
        .fee_handler()
        .save(&mut deps.storage, &address)
        .unwrap();
    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "ucosm")]);
    let response = contract
        .setprotocol_feehandler(deps.as_mut(), env, info, address)
        .unwrap();
    assert_eq!(response.attributes.len(), 2);
}

#[test]
#[should_panic(expected = "OnlyAdmin")]
fn test_invalid_input() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("user", &[Coin::new(1000, "ucosm")]);
    let address = Address::from("xyz");
    let cw_callservice = CwCallservice::new();
    cw_callservice
        .add_owner(
            deps.as_mut().storage,
            Address::from(&info.sender.to_string()),
        )
        .unwrap();

    cw_callservice
        .add_admin(deps.as_mut().storage, info.clone(), admin_one())
        .unwrap();

    cw_callservice
        .setprotocol_feehandler(deps.as_mut(), env, info, address.clone())
        .unwrap();
}

#[test]
fn test_send_packet_return_ibc_msg() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let cw_callservice = CwCallservice::new();
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let ibc_config = IbcConfig::new(src, dst);
    cw_callservice
        .ibc_config()
        .save(deps.as_mut().storage, &ibc_config)
        .unwrap();

    let data = to_binary(&"xyz").unwrap();
    let accured_fees = Coin {
        denom: "xyz".to_owned(),
        amount: 100u128.into(),
    };

    let ibc_msg = cw_callservice.create_packet(deps.as_ref(), env, data, accured_fees.clone());

    match ibc_msg {
        IbcMsg::Transfer { amount, .. } => {
            assert_eq!(amount, accured_fees);
        }
        _ => panic!("Expected Transfer message"),
    }
}

#[test]
fn test_get_balance_with_invalid_address_should_return_error() {
    let deps = mock_dependencies();
    let cw_callservice = CwCallservice::new();

    let result = cw_callservice.get_balance(deps.as_ref(), &Address::from("invalid"));

    match result {
        Err(StdError::NotFound { .. }) => {}
        _ => panic!("error"),
    }
}

#[test]
fn test_get_protocol_fee_handler() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("user", &[Coin::new(1000, "ucosm")]);
    let address = Address::from("xyz");

    let contract = CwCallservice::new();
    contract
        .add_owner(
            deps.as_mut().storage,
            Address::from(&info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(deps.as_mut().storage, info.clone(), admin_one())
        .unwrap();
    contract
        .fee_handler()
        .save(&mut deps.storage, &address)
        .unwrap();
    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "ucosm")]);
    contract
        .setprotocol_feehandler(deps.as_mut(), env, info, address)
        .unwrap();
    let result = contract.get_protocolfeehandler(deps.as_ref()).unwrap();
    assert_eq!("xyz", result.to_string());
}
