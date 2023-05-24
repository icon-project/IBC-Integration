use cosmwasm_std::{
    coins,
    testing::{mock_dependencies, mock_info, MockApi, MockQuerier, MockStorage},
    Empty, MessageInfo, OwnedDeps,
};

pub fn create_mock_info(creator: &str, denom: &str, amount: u128) -> MessageInfo {
    let funds = coins(amount, denom);
    mock_info(creator, &funds)
}

pub fn deps() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    mock_dependencies()
}
