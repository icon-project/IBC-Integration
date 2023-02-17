use cw_xcall::{
    rollback_message::{try_rollbackexecuted, RollbackMessage},
    types::address::Address,
    types::request::CallServiceMessageRequest,
};

#[test]
fn test_rollbackmessage() {
    let msg = RollbackMessage::new(123, vec![1, 2, 3], "message".to_owned());
    let event = msg.rollbackmessage();
    assert_eq!(event.ty, "rollbackmessage");
    assert_eq!(event.attributes[0].key, "sn");
    assert_eq!(event.attributes[0].value, "123");
}

#[test]
fn test_rollbackexecuted() {
    let msg = RollbackMessage::new(123, vec![1, 2, 3], "message".to_owned());
    let event = msg.rollbackexecuted();
    assert_eq!(event.ty, "rollbackexecuted");
    assert_eq!(event.attributes.len(), 3);
    assert_eq!(event.attributes[0].key, "sn");
    assert_eq!(event.attributes[0].value, "123");
    assert_eq!(event.attributes[1].key, "rollback");
    assert_eq!(event.attributes[1].value, "\u{1}\u{2}\u{3}");
    assert_eq!(event.attributes[2].key, "message");
    assert_eq!(event.attributes[2].value, "message");
}

#[test]
fn test_contains_rollback() {
    let msg = CallServiceMessageRequest::new(
        Address::from_string(
            "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4".to_string(),
        ),
        "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7".to_owned(),
        123,
        vec![1, 2, 3],
        vec![4, 5, 6],
    );
    let m = try_rollbackexecuted(msg);
    assert_eq!(m.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_not_contains_rollback() {
    let msg = CallServiceMessageRequest::new(
        Address::from_string(
            "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4".to_string(),
        ),
        "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f123t7".to_owned(),
        123,
        vec![1, 2, 3],
        vec![4, 5, 6],
    );
    let n = try_rollbackexecuted(msg);
    assert!(n.is_err());
}
