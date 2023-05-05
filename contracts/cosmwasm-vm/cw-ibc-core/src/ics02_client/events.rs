use super::*;

/// creates a client event with specified attributes.
/// 
/// Arguments:
/// 
/// * `client_id`: The client identifier is a unique identifier assigned to an IBC client. It is used to
/// differentiate between different clients on the same chain.
/// * `client_type`: The `client_type` parameter is a string that represents the type of client being
/// created. In the context of the Inter-Blockchain Communication (IBC) protocol, a client is a software
/// module that connects to a blockchain network and verifies the validity of the data received from
/// that network. The client type can
/// * `consensus_height`: Consensus height refers to the height of the consensus state of a blockchain
/// network. It is a unique identifier for a specific state of the network and is used in the IBC
/// protocol to synchronize state between different chains.
/// 
/// Returns:
/// 
/// A new `Event` object with attributes `CLIENT_ID_ATTRIBUTE_KEY`, `CLIENT_TYPE_ATTRIBUTE_KEY`, and
/// `CONSENSUS_HEIGHT_ATTRIBUTE_KEY` added to it. The event type is `CreateClient`.
pub fn create_client_event(client_id: &str, client_type: &str, consensus_height: &str) -> Event {
    Event::new(IbcEventType::CreateClient.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id)
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type)
        .add_attribute(CONSENSUS_HEIGHT_ATTRIBUTE_KEY, consensus_height)
}

/// This function creates an event for updating a client with the given client type, consensus height,
/// consensus heights, and client ID.
/// 
/// Arguments:
/// 
/// * `client_type`: The type of the IBC client being updated (e.g. Tendermint, Solo Machine, etc.).
/// * `consensus_height`: The height of the consensus state being updated for the client.
/// * `consensus_heights`: A vector of `Height` values representing the consensus heights of the client.
/// * `client_id`: The ID of the client being updated.
/// 
/// Returns:
/// 
/// an `Event` object.
pub fn update_client_event(
    client_type: IbcClientType,
    consensus_height: Height,
    consensus_heights: Vec<Height>,
    client_id: &IbcClientId,
) -> Event {
    let consensus_heights: Vec<String> = consensus_heights
        .into_iter()
        .map(|consensus_height| consensus_height.to_string())
        .collect();

    Event::new(IbcEventType::UpdateClient.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type.as_str())
        .add_attribute(CONSENSUS_HEIGHT_ATTRIBUTE_KEY, consensus_height)
        .add_attribute(CONSENSUS_HEIGHTS_ATTRIBUTE_KEY, consensus_heights.join(","))
}

/// This function creates an event for upgrading a client.
/// 
/// Arguments:
/// 
/// * `client_type`: The type of the IBC client being upgraded (e.g. Tendermint, Solo Machine, etc.).
/// * `consensus_height`: `consensus_height` is a parameter of type `Height` which represents the height
/// of the consensus state of the client being upgraded. This height is used to verify that the client
/// being upgraded is at the correct height before the upgrade can proceed.
/// * `client_id`: The ID of the client being upgraded.
/// 
/// Returns:
/// 
/// an `Event` object.
pub fn upgrade_client_event(
    client_type: IbcClientType,
    consensus_height: Height,
    client_id: IbcClientId,
) -> Event {
    Event::new(IbcEventType::UpgradeClient.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type.as_str())
        .add_attribute(CONSENSUS_HEIGHT_ATTRIBUTE_KEY, consensus_height)
}

/// This function creates a new event for client misbehavior.
/// 
/// Arguments:
/// 
/// * `client_id`: The `client_id` parameter is a string that represents the unique identifier of a
/// client in the Inter-Blockchain Communication (IBC) protocol. Clients are entities that maintain a
/// connection to a particular blockchain network and can send and receive messages to and from other
/// clients on different networks.
/// * `client_type`: The `client_type` parameter is a string that represents the type of client that has
/// misbehaved. It could be a solo machine client, a light client, or any other type of client that is
/// supported by the Inter-Blockchain Communication (IBC) protocol.
/// 
/// Returns:
/// 
/// A new `Event` object with the type `IbcEventType::ClientMisbehaviour` and two attributes:
/// `CLIENT_ID_ATTRIBUTE_KEY` with the value of `client_id` and `CLIENT_TYPE_ATTRIBUTE_KEY` with the
/// value of `client_type`.
pub fn client_misbehaviour_event(client_id: &str, client_type: &str) -> Event {
    Event::new(IbcEventType::ClientMisbehaviour.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id)
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type)
}

/// The function generates an event with a specific attribute for a given client ID.
/// 
/// Arguments:
/// 
/// * `client_id`: The `client_id` parameter is of type `IbcClientId`, which is likely a custom data
/// type representing an identifier for an Inter-Blockchain Communication (IBC) client. This function
/// generates an event with the name "client_id_created" and adds an attribute with the key
/// `CLIENT_ID_ATTRIBUTE_KEY
/// 
/// Returns:
/// 
/// A function is being returned that takes an `IbcClientId` as input and returns an `Event`. The
/// `Event` represents a client ID creation event and contains an attribute with the key
/// `CLIENT_ID_ATTRIBUTE_KEY` and the value of the `IbcClientId` converted to a string.
pub fn generated_client_id_event(client_id: IbcClientId) -> Event {
    Event::new("client_id_created").add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
}
