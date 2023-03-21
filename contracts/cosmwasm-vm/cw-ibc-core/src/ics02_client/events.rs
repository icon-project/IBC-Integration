use super::*;

pub fn create_client_event(
    client_id: IbcClientId,
    client_type: IbcClientType,
    consensus_height: Height,
) -> Event {
    Event::new(IbcEventType::CreateClient.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type.as_str())
        .add_attribute(CONSENSUS_HEIGHT_ATTRIBUTE_KEY, consensus_height)
}

pub fn update_client_event(
    client_type: IbcClientType,
    consensus_height: Height,
    consensus_heights: Vec<Height>,
    message: MsgUpdateClient,
) -> Event {
    let consensus_heights: Vec<String> = consensus_heights
        .into_iter()
        .map(|consensus_height| consensus_height.to_string())
        .collect();

    Event::new(IbcEventType::UpdateClient.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, message.client_id.as_str())
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type.as_str())
        .add_attribute(CONSENSUS_HEIGHT_ATTRIBUTE_KEY, consensus_height)
        .add_attribute(CONSENSUS_HEIGHTS_ATTRIBUTE_KEY, consensus_heights.join(","))
}

pub fn upgrade_client_event(
    client_type: IbcClientType,
    consensus_height: Height,
    message: MsgUpgradeClient,
) -> Event {
    Event::new(IbcEventType::UpgradeClient.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, message.client_id.as_str())
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type.as_str())
        .add_attribute(CONSENSUS_HEIGHT_ATTRIBUTE_KEY, consensus_height)
}

pub fn client_misbehaviour_event(client_id: IbcClientId, client_type: IbcClientType) -> Event {
    Event::new(IbcEventType::ClientMisbehaviour.as_str())
        .add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
        .add_attribute(CLIENT_TYPE_ATTRIBUTE_KEY, client_type.as_str())
}

pub fn event_client_id_generated(client_id: IbcClientId) -> Event {
    Event::new("client_id_created").add_attribute(CLIENT_ID_ATTRIBUTE_KEY, client_id.as_str())
}
