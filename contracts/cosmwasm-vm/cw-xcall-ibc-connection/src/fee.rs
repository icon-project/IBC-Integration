use common::rlp::Nullable;
use cw_xcall_lib::network_address::NetId;

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
        let caller = info.sender;
        let fees = self.get_unclaimed_packet_fee(deps.as_ref().storage, &nid, caller.as_ref());
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
        let sub_msg = self.call_host_send_message(deps, packet)?;
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
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::testing::{mock_dependencies, mock_info};
    use cosmwasm_std::Response;
    use cw_xcall_lib::network_address::NetId;

    use super::CwIbcConnection;

    #[test]
    #[should_panic(expected = "NoFeesAccrued")]
    fn test_claim_fees_on_zero_unclaimed_fee() {
        let mut deps = mock_dependencies();
        let contract = CwIbcConnection::new();
        let info = mock_info("relayer", &[]);

        let nid = NetId::from("default".to_string());

        contract
            .claim_fees(deps.as_mut(), info, nid, "relayer".to_string())
            .unwrap();
    }

    #[test]
    fn test_set_fee() {
        let mut deps = mock_dependencies();
        let contract = CwIbcConnection::new();

        let nid = NetId::from("default".to_string());

        let res = contract
            .set_fee(deps.as_mut().storage, nid, 1_000, 1_000)
            .unwrap();

        assert_eq!(res, Response::new())
    }

    #[test]
    fn test_reset_unclaimed_ack_fees() {
        let mut deps = mock_dependencies();
        let contract = CwIbcConnection::new();

        let nid = NetId::from("default".to_string());

        contract
            .add_unclaimed_ack_fees(deps.as_mut().storage, &nid, 1, 10)
            .unwrap();

        let ack_fee = contract.get_unclaimed_ack_fee(deps.as_ref().storage, nid.as_str(), 1);
        assert_eq!(ack_fee, 10);

        contract
            .reset_unclaimed_ack_fees(deps.as_mut().storage, nid.as_str(), 1)
            .unwrap();

        let ack_fee = contract.get_unclaimed_ack_fee(deps.as_ref().storage, nid.as_str(), 1);
        assert_eq!(ack_fee, 0)
    }
}
