use super::*;

pub fn create_open_init_event(
    connection_id: &str,
    client_id: &str,
    counterparty_client_id: &str,
) -> Event {
    Event::new(IbcEventType::OpenInitConnection.as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_id)
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id)
        .add_attribute(COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY, counterparty_client_id)
}

pub fn create_open_try_event(
    connection_id: ConnectionId,
    client_id: ClientId,
    counterparty_connection_id: ConnectionId,
    counterparty_client_id: ClientId,
) -> Event {
    Event::new(IbcEventType::OpenTryConnection.as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_id.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
        .add_attribute(
            COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY,
            counterparty_connection_id.as_str(),
        )
        .add_attribute(
            COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
            counterparty_client_id.as_str(),
        )
}

pub fn create_open_ack_event(
    connection_id: ConnectionId,
    client_id: ClientId,
    counterparty_connection_id: ConnectionId,
    counterparty_client_id: ClientId,
) -> Event {
    Event::new(IbcEventType::OpenAckConnection.as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_id.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
        .add_attribute(
            COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY,
            counterparty_connection_id.as_str(),
        )
        .add_attribute(
            COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
            counterparty_client_id.as_str(),
        )
}

pub fn create_open_confirm_event(
    connection_id_on_b: ConnectionId,
    client_id_on_b: ClientId,
    counterparty_connection_id_on_a: ConnectionId,
    counterparty_client_id_on_a: ClientId,
) -> Event {
    Event::new(IbcEventType::OpenConfirmConnection.as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_id_on_b.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id_on_b.as_str())
        .add_attribute(
            COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY,
            counterparty_connection_id_on_a.as_str(),
        )
        .add_attribute(
            COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
            counterparty_client_id_on_a.as_str(),
        )
}
