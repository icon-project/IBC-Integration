use super::*;

/// The function creates a new event with attributes for a call execution and returns it.
///
/// Arguments:
///
/// * `request_id`: The `request_id` parameter is an unsigned 128-bit integer that represents the unique
/// identifier of a call execution event.
/// * `code`: The `code` parameter is an integer value representing the status code of the executed
/// call. It can be used to indicate whether the call was successful or encountered an error.
/// * `msg`: The `msg` parameter is a string that represents a message associated with the event. It can
/// be used to provide additional information or context about the event being triggered.
///
/// Returns:
///
/// This code returns an instance of the `Event` struct with attributes `request_id`, `code`, and `msg`
/// added to it. The event represents the execution of a call.
pub fn event_call_executed(request_id: u128, code: i8, msg: &str) -> Event {
    Event::new("callexecuted")
        .add_attribute("request_id", request_id.to_string())
        .add_attribute("code", code.to_string())
        .add_attribute("msg", msg.to_string())
}

/// This Rust function creates an event for a message sent through a cross-chain communication protocol.
///
/// Arguments:
///
/// * `sequence_no`: The sequence number of the message being sent. This is used to ensure that messages
/// are processed in the correct order.
/// * `from`: The sender of the cross-chain message. It is of type String.
/// * `req_id`: req_id is a unique identifier for the request being sent. It is of type u128, which
/// means it can hold a very large number. This identifier is used to track the request and match it
/// with the corresponding response.
/// * `data`: The `data` parameter is of type `&CallServiceMessage`, which is a reference to a struct
/// that contains information about a cross-chain message being sent. It likely includes details such as
/// the recipient chain, the message payload, and any required signatures or authentication.
///
/// Returns:
///
/// an instance of the `Event` struct.
pub fn event_xcall_message_sent(
    from: String,
    req_id: u128,
    data: &CallServiceMessage,
) -> Event {
    let event = Event::new("xcall_message_sent");

    event
        .add_attribute("from", from)
        .add_attribute("sq_no", req_id.to_string())
        .add_attribute("data", to_binary(data).unwrap().to_string())
}

/// The function creates an event object for a rollback execution with sequence number, code, and
/// message attributes.
///
/// Arguments:
///
/// * `sequence_no`: The sequence number of the rollback request that was executed.
/// * `code`: The `code` parameter is an integer value that represents the status code of the rollback
/// execution. It could be used to indicate whether the rollback was successful or not, or to provide
/// more specific information about the reason for the rollback.
/// * `msg`: The `msg` parameter is a string that represents a message or description of the event that
/// occurred during the rollback execution. It is used to provide additional information about the event
/// to the user or other parts of the system.
///
/// Returns:
///
/// A new `Event` object with attributes "request_id", "code", and "msg" added to it.
pub fn event_rollback_executed(sequence_no: u128, code: i8, msg: &str) -> Event {
    Event::new("rollback_executed")
        .add_attribute("request_id", sequence_no.to_string())
        .add_attribute("code", code.to_string())
        .add_attribute("msg", msg.to_string())
}

/// The function creates a new event with attributes for a call message in Rust.
///
/// Arguments:
///
/// * `from`: The `from` parameter is a `String` representing the sender of a message in an event.
/// * `to`: The "to" parameter in the function `event_call_message` represents the recipient of a call
/// message. It is a string type parameter that takes the value of the recipient's address or
/// identifier.
/// * `sequence_no`: The `sequence_no` parameter is an unsigned 128-bit integer that represents the
/// sequence number of a message being sent from one entity to another. It is used to ensure that
/// messages are processed in the correct order and to prevent duplicate messages.
/// * `request_id`: The `request_id` parameter is a unique identifier for a specific request being made
/// in the `call_message` event. It is of type `u128`, which means it can hold a very large integer
/// value. This identifier can be used to track the progress of the request and match it with the
///
/// Returns:
///
/// A function is being returned that creates an instance of the `Event` struct with the attributes
/// "call_message", "from", "to", "sequence_no", and "request_id".
pub fn event_call_message(from: String, to: String, sequence_no: u128, request_id: u128) -> Event {
    Event::new("call_message")
        .add_attribute("from", from)
        .add_attribute("to", to)
        .add_attribute("sequence_no", sequence_no.to_string())
        .add_attribute("request_id", request_id.to_string())
}

/// The function creates an event with a "rollback_message" type and a sequence number attribute.
///
/// Arguments:
///
/// * `sequence_no`: The `sequence_no` parameter is an unsigned 128-bit integer that represents the
/// sequence number of a rollback message event. It is used to uniquely identify the event and keep
/// track of the order in which events occur.
///
/// Returns:
///
/// A new `Event` object with the name "rollback_message" and an attribute "sequence_no" with the value
/// of `sequence_no` converted to a string.
pub fn event_rollback_message(sequence_no: u128) -> Event {
    Event::new("rollback_message").add_attribute("sequence_no", sequence_no.to_string())
}

/// This Rust function creates an event with attributes for a response message.
///
/// Arguments:
///
/// * `sequence_no`: The sequence number is a unique identifier for a particular event or message. It is
/// used to keep track of the order in which events or messages are sent and received.
/// * `response_code`: The `response_code` parameter is an integer value representing the response code
/// of an event. It is of type `i8`, which means it can hold values from -128 to 127. The response code
/// is typically used to indicate the status or outcome of an operation or request.
/// * `message`: The `message` parameter is a string that represents the message to be included in the
/// response event. It can be any text that provides additional information about the response.
///
/// Returns:
///
/// A new `Event` object with the attributes `sequence_no`, `response_code`, and `message` added to it.
pub fn event_response_message(sequence_no: u128, response_code: i8, message: &str) -> Event {
    Event::new("response_message")
        .add_attribute("sequence_no", sequence_no.to_string())
        .add_attribute("response_code", response_code.to_string())
        .add_attribute("message", message.to_string())
}
