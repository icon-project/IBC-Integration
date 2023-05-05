use super::*;

pub fn create_client_event(client_id: &str, client_type: &str, consensus_height: &str) -> Event {
    Event::new(IbcEventType::CreateClient.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id)
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type)
        .add_attribute(CONSENSUS_HEIGHT_ATTRIBUTE_KEY, consensus_height)
}

pub fn update_client_event(
    client_type: IbcClientType,
    consensus_height: IbcHeight,
    consensus_heights: Vec<IbcHeight>,
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

pub fn upgrade_client_event(
    client_type: IbcClientType,
    consensus_height: IbcHeight,
    client_id: IbcClientId,
) -> Event {
    Event::new(IbcEventType::UpgradeClient.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type.as_str())
        .add_attribute(CONSENSUS_HEIGHT_ATTRIBUTE_KEY, consensus_height)
}

pub fn client_misbehaviour_event(client_id: &str, client_type: &str) -> Event {
    Event::new(IbcEventType::ClientMisbehaviour.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id)
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type)
}

pub fn generated_client_id_event(client_id: IbcClientId) -> Event {
    Event::new("client_id_created").add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
}
