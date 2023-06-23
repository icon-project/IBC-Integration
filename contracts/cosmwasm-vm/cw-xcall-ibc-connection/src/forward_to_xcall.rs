use cosmwasm_std::DepsMut;
use debug_print::debug_println;

use super::*;
use crate::{events::event_packet_received, state::XCALL_FORWARD_REPLY_ID};

impl<'a> CwIbcConnection<'a> {
    /// This function receives packet data, decodes it, and then handles either a request or a response
    /// based on the message type.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is short for "dependencies mutable". It is a
    /// struct that provides access to the dependencies needed by the contract to execute its logic.
    /// These dependencies include the storage, the API to interact with the blockchain, and the querier
    /// to query data
    /// * `message`: The `message` parameter is of type `IbcPacket` and represents the packet received
    /// by the contract from another chain. It contains the data sent by the sender chain and metadata
    /// about the packet, such as the sender and receiver addresses, the sequence number, and the
    /// timeout height.
    ///
    /// Returns:
    ///
    /// a `Result` object with either an `IbcReceiveResponse` or a `ContractError`.
    pub fn receive_packet_data(
        &self,
        deps: DepsMut,
        message: CwPacket,
    ) -> Result<CwReceiveResponse, ContractError> {
        self.forward_to_xcall(deps, message)
    }

    pub fn forward_to_xcall(
        &self,
        deps: DepsMut,
        message: CwPacket,
    ) -> Result<CwReceiveResponse, ContractError> {
        let event = event_packet_received(&message);
        debug_println!("[IBCConnection]: forwarding to xcall");
        let data = message.data.clone();
        let xcall_msg = cw_common::xcall_app_msg::ExecuteMsg::ReceiveCallMessage {
            msg: data.0,
            sn: Some(0),
            from: "".to_string(),
        };
        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self
                .get_xcall_host(deps.as_ref().storage)
                .unwrap()
                .to_string(),
            msg: to_binary(&xcall_msg).unwrap(),
            funds: vec![],
        });
        debug_println!("[IBCConnection]: message payload built");

        let acknowledgement_data =
            to_binary(&cw_common::client_response::XcallPacketResponseData {
                packet: message,
                acknowledgement: make_ack_success().to_vec(),
            })
            .map_err(ContractError::Std)?;

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, XCALL_FORWARD_REPLY_ID);

        Ok(CwReceiveResponse::new()
            .add_attribute("action", "receive_packet_data")
            .add_attribute("method", "forward_to_xcall")
            .set_ack(acknowledgement_data)
            .add_event(event)
            .add_submessage(sub_msg))
    }
}
