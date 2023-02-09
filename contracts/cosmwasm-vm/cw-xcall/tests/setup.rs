use cosmwasm::traits::{Api, Storage};
use cosmwasm_std::{
    coins,
    testing::{mock_dependencies, mock_env, mock_info},
};
use cw_xcall::{contract, msg::InstantiateMsg};

fn test_setup() {
    let mut deps = mock_dependencies();

    let creator = String::from("creator");
    let init_amount = coins(1000, "earth");
    let init_info = mock_info(&creator, &init_amount);
    contract::instantiate(deps.as_mut(), mock_env(), init_info, InstantiateMsg {}).unwrap();
}
