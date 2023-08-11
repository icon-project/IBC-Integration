use super::*;

use cosmwasm_std::DepsMut;

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
        packet: CwPacket,
        _relayer: Addr,
    ) -> Result<CwReceiveResponse, ContractError> {
        debug_println!("[MockDapp]: Packet Received");
        self.store_received_packet(deps.storage, packet.sequence, packet)?;

        Ok(CwReceiveResponse::new())
    }

    pub fn write_acknowledgement(
        &self,
        store: &mut dyn Storage,
        packet: CwPacket,
    ) -> Result<Response, ContractError> {
        let submsg = self.call_host_write_acknowledgement(store, packet, b"ack".to_vec())?;
        Ok(Response::new().add_submessage(submsg))
    }
}
