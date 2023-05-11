use cosmwasm_std::DepsMut;

use crate::{
    events::{ event_packet_received},
    state::XCALL_FORWARD_REPLY_ID,
};
use super::*;

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
        let data = message.data;
        let xcall_msg = cw_common::xcall_app_msg::ExecuteMsg::ReceiveCallMessage {
            data: data.0.clone(),
        };
        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self
                .get_xcall_host(deps.as_ref().storage)
                .unwrap()
                .to_string(),
            msg: to_binary(&xcall_msg).unwrap(),
            funds: vec![],
        });

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, XCALL_FORWARD_REPLY_ID);

        Ok(CwReceiveResponse::new()
            .add_attribute("action", "receive_packet_data")
            .add_attribute("method", "forward_to_xcall")
            .set_ack(make_ack_success().to_vec())
            .add_event(event)
            .add_submessage(sub_msg))
    }

    /// This function saves an IBC configuration to a storage and returns an error if it fails.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the storage of the smart contract. The `save` method of the `ibc_config` struct
    /// is called with this `store` parameter to save the `config` parameter to the
    /// * `config`: The `config` parameter is a reference to an `IbcConfig` struct that contains the
    /// configuration settings for the IBC module. This struct contains various fields such as the chain
    /// ID, the module account address, and the packet timeout settings. The `save_config` function is
    /// responsible for persisting
    ///
    /// Returns:
    ///
    /// This function returns a `Result` with either an empty `Ok(())` value if the `config` is
    /// successfully saved to the `store`, or a `ContractError` wrapped in `Err` if there was an error
    /// while saving the `config`.
    pub fn save_config(
        &mut self,
        store: &mut dyn Storage,
        config: &IbcConfig,
    ) -> Result<(), ContractError> {
        match self.ibc_config().save(store, config) {
            Ok(_) => Ok(()),
            Err(err) => Err(ContractError::Std(err)),
        }
    }
}
