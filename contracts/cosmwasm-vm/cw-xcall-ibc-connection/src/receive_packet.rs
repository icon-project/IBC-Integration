use std::str::from_utf8;

use common::rlp;
use cosmwasm_std::{coins, BankMsg, DepsMut};
use debug_print::debug_println;

use super::*;
use crate::types::message::Message;

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
    pub fn do_packet_receive(
        &self,
        deps: DepsMut,
        message: CwPacket,
        relayer: Addr,
    ) -> Result<CwReceiveResponse, ContractError> {
        let channel = message.dest.channel_id.clone();
        let n_message: Message = rlp::decode(&message.data.0).unwrap();
        let channel_config = self.get_channel_config(deps.as_ref().storage, &channel)?;
        let nid = channel_config.counterparty_nid;

        self.add_unclaimed_packet_fees(deps.storage, &nid.as_str(), relayer.as_str(), n_message.fee)?;

        if n_message.sn.is_none() {
            let receiver_address = from_utf8(&n_message.data).unwrap();
            let amount = n_message.fee;
            let msg = BankMsg::Send {
                to_address: receiver_address.to_string(),
                amount: coins(amount, "arch"),
            };
            return Ok(CwReceiveResponse::new().add_message(msg));
        }

        if let Some(sn) = n_message.sn.0 {
            if sn > 0 {
                self.store_incoming_packet_sequence(deps.storage, &channel, sn, message.sequence)?;
            }
        }
        debug_println!("[IBCConnection]: forwarding to xcall");
        let data = message.data.0.clone();
        let xcall_submessage =
            self.call_xcall_handle_message(deps.storage, &nid, data, n_message.sn.0)?;

        debug_println!("[IBCConnection]: message payload built");

        let acknowledgement_data =
            to_binary(&cw_common::client_response::XcallPacketResponseData {
                packet: message,
                acknowledgement: make_ack_success().to_vec(),
            })
            .map_err(ContractError::Std)?;

        Ok(CwReceiveResponse::new()
            .set_ack(acknowledgement_data)
            .add_submessage(xcall_submessage))
    }
}
