use cosmwasm_std::Event;
use cw_xcall::{
    rollback_message::{try_rollback_executed, RollbackMessage},
    types::address::Address,
    types::request::CallServiceMessageRequest,
};

#[test]
fn test_rollback_message() {
    let msg = RollbackMessage::new(123, vec![1, 2, 3], "message".to_owned());
    let actual_event = msg.rollback_message();
    let expected_event = Event::new("rollbackmessage").add_attribute("sequence_no", "123");
    assert_eq!(actual_event, expected_event)
}

#[test]
fn test_rollback_executed() {
    let msg = RollbackMessage::new(123, vec![1, 2, 3], "message".to_owned());
    let actual_event = msg.rollback_executed();
    let expected_event = Event::new("rollbackexecuted")
        .add_attribute("sequence_no", "123")
        .add_attribute("rollback", String::from_utf8(vec![1, 2, 3]).unwrap())
        .add_attribute("message", "message");
    assert_eq!(actual_event, expected_event);
}

#[test]
fn test_contains_rollback() {
    let msg = CallServiceMessageRequest::new(
        Address::from_str("88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4"),
        "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7".to_owned(),
        123,
        vec![1, 2, 3],
        vec![4, 5, 6],
    );
    let m = try_rollback_executed(msg);
    assert_eq!(m.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_not_contains_rollback() {
    let msg = CallServiceMessageRequest::new(
        Address::from_str("88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4"),
        "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7".to_owned(),
        123,
        vec![],
        vec![1, 2, 3],
    );
    let n = try_rollback_executed(msg);
    assert!(n.is_err());
}
