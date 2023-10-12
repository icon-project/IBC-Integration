use cw_ibc_core::VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE;

use super::*;

#[test]
fn test_timeout_on_close_packet_validate_to_light_client() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let height = 2;
    let timeout_timestamp = 5;
    let msg = get_dummy_raw_msg_timeout_on_close(height, timeout_timestamp);
    let mut test_context = TestContext::for_packet_timeout_on_close(env.clone(), &msg);
    test_context.init_timeout_packet_on_close(deps.as_mut().storage, &contract);

    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res =
        contract.timeout_on_close_packet_validate_to_light_client(deps.as_mut(), info, env, msg);
    print!("{res:?}");
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE
    )
}
