mod account;
mod setup;
use cosmwasm_std::{
    testing::{mock_env, MOCK_CONTRACT_ADDR},
    Addr,
};

use cw_xcall_ibc_connection::{instantiate, msg::InstantiateMsg, state::CwIbcConnection};
use setup::*;

#[test]

fn proper_instantiate() {
    let mut mock_deps = deps();
    let mock_info = create_mock_info(MOCK_CONTRACT_ADDR, "umlg", 2000);
    let env = mock_env();
    let store = CwIbcConnection::default();

    let res = instantiate(
        mock_deps.as_mut(),
        env,
        mock_info,
        InstantiateMsg {
            timeout_height: 10,
            ibc_host: Addr::unchecked("someaddress"),
            protocol_fee: 0,
        },
    )
    .unwrap();

    assert_eq!(res.messages.len(), 0);

    let owner = store.query_owner(mock_deps.as_ref().storage).unwrap();

    assert_eq!(MOCK_CONTRACT_ADDR, owner.to_string())
}
