use common::rlp::Nullable;
use cw_common::xcall_types::network_address::NetId;

use crate::types::{message::Message, network_fees::NetworkFees};

use super::*;
/// This is an implementation of two methods for the `CwCallService` struct.

impl<'a> CwIbcConnection<'a> {
    pub fn claim_fees(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        nid: NetId,
        address: String,
    ) -> Result<SubMsg, ContractError> {
        let caller = info.sender.clone();
        let fees = self.get_unclaimed_packet_fee(deps.as_ref().storage, &nid, caller.as_ref())?;
        if fees == 0 {
            return Err(ContractError::NoFeesAccrued);
        }
        let ibc_config = self.get_ibc_config(deps.as_ref().storage, &nid)?;

        self.reset_unclaimed_packet_fees(deps.storage, &nid, caller.as_ref())?;
        let sequence_no = self.query_host_sequence_no(deps.as_ref(), &ibc_config)?;
        let message = Message {
            sn: Nullable::new(None),
            fee: fees,
            data: address.as_bytes().to_vec(),
        };
        let timeout_height =
            self.query_timeout_height(deps.as_ref(), &ibc_config.src_endpoint().channel_id)?;
        let packet = self.create_packet(ibc_config, timeout_height, sequence_no, message);
        let sub_msg = self.call_host_send_message(deps, info, packet)?;
        Ok(sub_msg)
    }

    pub fn set_fee(
        &self,
        store: &mut dyn Storage,
        nid: NetId,
        packet_fee: u128,
        ack_fee: u128,
    ) -> Result<Response, ContractError> {
        let net_fee = NetworkFees {
            send_packet_fee: packet_fee,
            ack_fee,
        };
        self.store_network_fees(store, nid, &net_fee)?;
        Ok(Response::new())
    }

    pub fn get_unclaimed_fee(&self, store: &dyn Storage, nid: NetId, address: String) -> u128 {
        self.get_unclaimed_packet_fee(store, &nid, &address)
            .unwrap_or(0)
    }
}
