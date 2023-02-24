use cosmwasm_std::{to_binary, Event};

use crate::types::message::CallServiceMessage;

pub fn event_call_executed(request_id: u128, code: i8, msg: &str) -> Event {
    Event::new("callexecuted")
        .add_attribute("request_id", request_id.to_string())
        .add_attribute("code", code.to_string())
        .add_attribute("msg", msg.to_string())
}
pub fn event_xcall_message_sent(
    sequence_no: u128,
    from: String,
    req_id: u128,
    data: &CallServiceMessage,
) -> Event {
    let event = Event::new("xcall_message_sent");

    event
        .add_attribute("from", from)
        .add_attribute("sequence_no", sequence_no.to_string())
        .add_attribute("req_id", req_id.to_string())
        .add_attribute("data", to_binary(data).unwrap().to_string())
}
