use super::*;

/// This function creates an event with attributes related to an open initialization connection.
///
/// Arguments:
///
/// * `connection_id`: A string representing the unique identifier of the connection being opened.
/// * `client_id`: The `client_id` parameter is a string that represents the unique identifier of the
/// client that is creating the connection. In the context of the Inter-Blockchain Communication (IBC)
/// protocol, a client is a module that is responsible for verifying the validity of the state of a
/// remote blockchain.
/// * `counterparty_client_id`: `counterparty_client_id` is a string parameter that represents the
/// identifier of the client on the counterparty chain that is associated with the connection being
/// opened. In the context of the Inter-Blockchain Communication (IBC) protocol, a connection is a
/// communication channel between two chains, and each chain has its
///
/// Returns:
///
/// A new `Event` object with attributes representing an `OpenInitConnection` event in the
/// Inter-Blockchain Communication (IBC) protocol.
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

/// This function creates an event for an open try connection.
///
/// Arguments:
///
/// * `connection_id`: A unique identifier for the connection being opened.
/// * `client_id`: The `client_id` parameter is a unique identifier for the client associated with the
/// connection being created.
/// * `counterparty_connection_id`: The `counterparty_connection_id` parameter is the identifier of the
/// connection on the counterparty chain that is being attempted to be opened.
/// * `counterparty_client_id`: The `counterparty_client_id` parameter is a unique identifier for the
/// client on the counterparty chain that is attempting to establish a connection with the local chain.
///
/// Returns:
///
/// an `Event` object.
pub fn create_open_try_event(
    connection_id: ConnectionId,
    client_id: ClientId,
    counterparty_connection_id: Option<ConnectionId>,
    counterparty_client_id: ClientId,
) -> Event {
    Event::new(IbcEventType::OpenTryConnection.as_str())
        .add_attribute(CONN_ID_ATTRIBUTE_KEY, connection_id.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
        .add_attribute(
            COUNTERPARTY_CONN_ID_ATTRIBUTE_KEY,
            counterparty_connection_id
                .map(|c| c.to_string())
                .unwrap_or("".to_owned()),
        )
        .add_attribute(
            COUNTERPARTY_CLIENT_ID_ATTRIBUTE_KEY,
            counterparty_client_id.as_str(),
        )
}

/// This function creates an event for acknowledging the opening of a connection.
///
/// Arguments:
///
/// * `connection_id`: A unique identifier for the connection between two parties in an Inter-Blockchain
/// Communication (IBC) protocol.
/// * `client_id`: The `client_id` parameter is a unique identifier for the client associated with the
/// connection being acknowledged.
/// * `counterparty_connection_id`: The `counterparty_connection_id` parameter is the identifier of the
/// connection on the counterparty chain that is being acknowledged in response to an open connection
/// attempt.
/// * `counterparty_client_id`: The `counterparty_client_id` parameter is a unique identifier for the
/// client on the counterparty chain that is associated with the connection being acknowledged.
///
/// Returns:
///
/// an `Event` object.
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

/// This function creates an event for confirming the opening of a connection between two clients.
///
/// Arguments:
///
/// * `connection_id_on_b`: The connection ID on the receiving end (B) of the connection open
/// confirmation event.
/// * `client_id_on_b`: `client_id_on_b` is the identifier of the client on chain B that is associated
/// with the connection being confirmed.
/// * `counterparty_connection_id_on_a`: The `counterparty_connection_id_on_a` parameter is the
/// identifier of the connection on the counterparty chain that is being confirmed as open.
/// * `counterparty_client_id_on_a`: `counterparty_client_id_on_a` is the client identifier of the
/// counterparty on chain A, which is the other end of the connection being confirmed.
///
/// Returns:
///
/// an `Event` object.
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
