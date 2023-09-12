use cw_xcall_lib::network_address::NetId;

use crate::types::network_fees::NetworkFees;

use super::*;
impl<'a> CwIbcConnection<'a> {
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
}
