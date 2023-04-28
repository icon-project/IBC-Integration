use super::*;

pub fn make_ack_success() -> Binary {
    let res = Ack::Result(b"1".into());

    to_binary(&res).unwrap()
}

pub fn make_ack_fail(err: String) -> Binary {
    let res = Ack::Error(err);

    to_binary(&res).unwrap()
}

pub fn on_ack_sucess(packet: IbcPacket) -> Result<IbcBasicResponse, ContractError> {
    let message: CallServiceMessage = from_binary(&packet.data)?;

    let message_type = match message.message_type() {
        CallServiceMessageType::CallServiceRequest => "call_service_request",
        CallServiceMessageType::CallServiceResponse => "call_service_response",
    };

    let attributes = vec![
        attr("action", "acknowledge"),
        attr("success", "true"),
        attr("message_type", message_type),
    ];

    Ok(IbcBasicResponse::new().add_attributes(attributes))
}

pub fn on_ack_failure(packet: IbcPacket, error: &str) -> Result<IbcBasicResponse, ContractError> {
    let message: CallServiceMessage = from_binary(&packet.data)?;
    let message_type = match message.message_type() {
        CallServiceMessageType::CallServiceRequest => "call_service_request",
        CallServiceMessageType::CallServiceResponse => "call_service_response",
    };

    Ok(IbcBasicResponse::new()
        .add_attribute("action", "acknowledge")
        .add_attribute("message_type", message_type)
        .add_attribute("success", "false")
        .add_attribute("error", error))
}

pub fn acknowledgement_data_on_success(packet: &IbcPacket) -> Result<Binary, ContractError> {
    to_binary(&cw_common::client_response::XcallPacketResponseData {
        packet: packet.clone(),
        acknowledgement: make_ack_success().to_vec(),
    })
    .map_err(ContractError::Std)
}
