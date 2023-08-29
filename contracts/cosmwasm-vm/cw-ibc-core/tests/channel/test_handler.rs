use std::collections::HashMap;

use cw_ibc_core::conversions::to_ibc_height;

use super::*;

#[test]
#[should_panic(expected = "UndefinedConnectionCounterparty")]
fn test_validate_open_try_channel_fail_missing_counterparty() {
    let mut deps = deps();
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let raw = get_dummy_raw_msg_chan_open_try(10);
    let mut test_context = TestContext::for_channel_open_try(env.clone(), &raw);
    let mut connection_end = test_context.connection_end();
    let mut counter_party = connection_end.counterparty().clone();
    counter_party.connection_id = None;
    connection_end.set_counterparty(counter_party);
    test_context.connection_end = Some(connection_end);
    test_context.init_channel_open_try(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);
    contract
        .validate_channel_open_try(deps.as_mut(), info, &raw)
        .unwrap();
}
