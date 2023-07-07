use std::str::from_utf8;

use super::*;
use crate::types::message::Message;
use common::ibc::core::ics04_channel::timeout::TimeoutHeight;
use common::{ibc::Height, rlp};
use cosmwasm_std::{coins, BankMsg, DepsMut};
use cw_common::{hex_string::HexString, raw_types::channel::RawPacket, ProstMessage};
use debug_print::debug_println;
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
        let denom = self.get_denom(deps.as_ref().storage)?;
        if n_message.sn.is_none() {
            let receiver_address = from_utf8(&n_message.data).unwrap();
            let amount = n_message.fee;
            let msg = BankMsg::Send {
                to_address: receiver_address.to_string(),
                amount: coins(amount, denom),
            };
            return Ok(CwReceiveResponse::new().add_message(msg));
        }
        self.add_unclaimed_packet_fees(deps.storage, &nid, relayer.as_str(), n_message.fee)?;

        if let Some(sn) = n_message.sn.0 {
            if sn > 0 {
                let height = Height::new(
                    message.timeout.block().unwrap().revision,
                    message.timeout.block().unwrap().height,
                )
                .map_err(|_error| StdError::GenericErr {
                    msg: "failed to map height".to_string(),
                })?;

                let timeout_height = TimeoutHeight::At(height);
                let packet = RawPacket {
                    sequence: message.sequence,
                    destination_port: message.dest.port_id,
                    destination_channel: message.dest.channel_id,
                    source_port: message.src.port_id,
                    source_channel: message.src.channel_id,
                    data: message.data.to_vec(),
                    timeout_height: timeout_height.into(),
                    timeout_timestamp: 0,
                };
                self.store_incoming_packet(
                    deps.storage,
                    &channel,
                    sn,
                    HexString::from_bytes(&packet.encode_to_vec()),
                )?;
            }
        }
        debug_println!("[IBCConnection]: forwarding to xcall");
        let data = n_message.data;
        let xcall_submessage =
            self.call_xcall_handle_message(deps.storage, &nid, data, n_message.sn.0)?;

        Ok(CwReceiveResponse::new().add_submessage(xcall_submessage))
    }
}
