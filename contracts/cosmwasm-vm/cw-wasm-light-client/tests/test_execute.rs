use crate::setup::TestContext;
use common::{
    icon::icon::types::v1::{BtpHeader, SignedHeader},
    traits::AnyTypes,
};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Coin,
};
use cw_common::raw_types::Any;
use cw_wasm_light_client::traits::IQueryHandler;
use cw_wasm_light_client::{
    constants::CLIENT_ID,
    contract::execute,
    msg::{ExecuteMsg, UpdateStateMsgRaw},
    query_handler::QueryHandler,
    utils::to_ibc_height,
};
use ics07_tendermint_cw::ics23::FakeInner;
use prost::Message;
use test_utils::{get_test_headers, get_test_signed_headers};
mod setup;

#[test]
pub fn test_update_success() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &vec![Coin::new(100, "test")]);
    let header = &get_test_headers()[0];
    let context = TestContext::for_instantiate();
    context.init(deps.as_mut().storage, header);

    let signed_header: &SignedHeader = &get_test_signed_headers()[1].clone();
    let header_any: Any = signed_header.to_any();
    let block_height = signed_header.header.clone().unwrap().main_height;
    let wasm_header = ics08_wasm::client_message::Header::<FakeInner> {
        inner: Box::new(FakeInner),
        data: header_any.encode_to_vec(),
        height: to_ibc_height(block_height),
    };

    let msg = ExecuteMsg::UpdateState(UpdateStateMsgRaw {
        client_message: cw_wasm_light_client::msg::ClientMessageRaw::Header(wasm_header),
    });
    let _result = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let updated_client_state =
        QueryHandler::get_client_state(deps.as_ref().storage, CLIENT_ID).unwrap();

    assert_eq!(updated_client_state.latest_height, block_height);

    let consensus_state= QueryHandler::get_consensus_state(deps.as_ref().storage, CLIENT_ID, block_height).unwrap();


    assert_eq!(
        consensus_state.message_root,
        signed_header.header.clone().unwrap().message_root
    )
}
