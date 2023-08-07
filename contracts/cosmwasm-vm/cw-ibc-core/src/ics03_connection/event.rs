use super::*;
pub const CONN_ID_ATTRIBUTE_KEY: &str = "connection_id";
pub const CLIENT_ID_ATTRIBUTE_KEY: &str = "client_id";
pub const COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY: &str = "counterparty_connection_id";
pub const COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY: &str = "counterparty_client_id";

pub fn create_connection_event(
    event_type: IbcEventType,
    connection_id: &ConnectionId,
    client_id: &ClientId,
    counterparty_client_id: &ClientId,
    counterparty_connection_id: Option<ConnectionId>,
) -> Result<Event, ContractError> {
    let mut event = Event::new(event_type.as_str());
    event = event
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_id.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
        .add_attribute(
            COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
            counterparty_client_id.as_str(),
        );

    match event_type {
        IbcEventType::OpenInitConnection => Ok(event),
        IbcEventType::OpenTryConnection
        | IbcEventType::OpenAckConnection
        | IbcEventType::OpenConfirmConnection => {
            event = event.add_attribute(
                COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY,
                counterparty_connection_id
                    .map(|c| c.to_string())
                    .unwrap_or("".to_owned()),
            );
            Ok(event)
        }
        _ => Err(ContractError::InvalidEventType {
            event: "Connection Event".to_string(),
            event_type: event_type.as_str().to_string(),
        }),
    }
}
