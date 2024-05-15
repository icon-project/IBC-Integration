use crate::state::IbcConfig;
use crate::types::LOG_PREFIX;
use common::ibc::Height;
use cosmwasm_std::{to_binary, CosmosMsg, Deps, DepsMut, Storage, SubMsg, WasmMsg};
use cw_common::cw_types::CwPacket;
use cw_common::query_helpers::build_smart_query;
use cw_common::raw_types::channel::RawChannel;
use cw_common::{hex_string::HexString, raw_types::channel::RawPacket, ProstMessage};

use cw_common::cw_println;

use crate::{
    error::ContractError,
    state::{CwIbcConnection, HOST_SEND_MESSAGE_REPLY_ID, HOST_WRITE_ACKNOWLEDGEMENT_REPLY_ID},
};

impl<'a> CwIbcConnection<'a> {
    pub fn call_host_write_acknowledgement(
        &self,
        store: &dyn Storage,
        packet: CwPacket,
        msg: Vec<u8>,
    ) -> Result<SubMsg, ContractError> {
        let host_address = self.get_ibc_host(store)?;

        let message = cw_common::core_msg::ExecuteMsg::WriteAcknowledgement {
            packet,
            acknowledgement: HexString::from_bytes(&msg),
        };

        let cosm_msg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: host_address.to_string(),
            msg: to_binary(&message).map_err(ContractError::Std)?,
            funds: vec![],
        });
        let submessage = SubMsg {
            id: HOST_WRITE_ACKNOWLEDGEMENT_REPLY_ID,
            msg: cosm_msg,
            gas_limit: None,
            reply_on: cosmwasm_std::ReplyOn::Always,
        };

        Ok(submessage)
    }

    pub fn call_host_send_message(
        &self,
        deps: DepsMut,
        packet: RawPacket,
    ) -> Result<SubMsg, ContractError> {
        let message = cw_common::core_msg::ExecuteMsg::SendPacket {
            packet: HexString::from_bytes(&packet.encode_to_vec()),
        };
        let ibc_host = self.get_ibc_host(deps.as_ref().storage)?;
        let submessage = SubMsg {
            id: HOST_SEND_MESSAGE_REPLY_ID,
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: ibc_host.to_string(),
                msg: to_binary(&message).map_err(ContractError::Std)?,
                funds: vec![],
            }),
            gas_limit: None,
            reply_on: cosmwasm_std::ReplyOn::Always,
        };
        cw_println!(
            deps.api,
            "{LOG_PREFIX} Packet Forwarded To IBCHost {ibc_host} "
        );
        Ok(submessage)
    }

    pub fn query_timeout_height(
        &self,
        deps: Deps,
        channel_id: &str,
    ) -> Result<Height, ContractError> {
        let channel_config = self.get_channel_config(deps.storage, channel_id)?;
        let ibc_host = self.get_ibc_host(deps.storage)?;
        let message = to_binary(&cw_common::core_msg::QueryMsg::GetLatestHeight {
            client_id: channel_config.client_id,
        })
        .unwrap();
        let query = build_smart_query(ibc_host.to_string(), message);
        let latest_height: u64 = deps.querier.query(&query).map_err(ContractError::Std)?;
        let timeout_height = latest_height + channel_config.timeout_height;
        Ok(Height::new(0, timeout_height).unwrap())
    }

    pub fn query_channel_state(
        &self,
        deps: Deps,
        port: String,
        channel: String,
    ) -> Result<i32, ContractError> {
        let ibc_host = self.get_ibc_host(deps.storage)?;
        let message = to_binary(&cw_common::core_msg::QueryMsg::GetChannel {
            port_id: port,
            channel_id: channel,
        })
        .unwrap();
        let query = build_smart_query(ibc_host.to_string(), message);
        let encoded_channel: String = deps.querier.query(&query).map_err(ContractError::Std)?;
        let channel = RawChannel::decode(hex::decode(encoded_channel).unwrap().as_slice()).unwrap();
        Ok(channel.state)
    }

    pub fn query_host_sequence_no(
        &self,
        deps: Deps,
        ibc_config: &IbcConfig,
    ) -> Result<u64, ContractError> {
        let ibc_host = self.get_ibc_host(deps.storage)?;
        let query_message = to_binary(&cw_common::core_msg::QueryMsg::GetNextSequenceSend {
            port_id: ibc_config.src_endpoint().clone().port_id,
            channel_id: ibc_config.src_endpoint().clone().channel_id,
        })
        .unwrap();

        let query_request = build_smart_query(ibc_host.to_string(), query_message);
        println!("{LOG_PREFIX} Created Query Request {ibc_host}");

        let sequence_number_host: u64 = deps
            .querier
            .query(&query_request)
            .map_err(ContractError::Std)?;
        Ok(sequence_number_host)
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{CwIbcConnection, HOST_SEND_MESSAGE_REPLY_ID};
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies_with_balance, mock_env},
        to_binary, Addr, CosmosMsg, SubMsg, WasmMsg,
    };
    use cw_common::{hex_string::HexString, raw_types::channel::RawPacket, ProstMessage};

    #[test]
    fn test_call_host_send_message() {
        let mut deps = mock_dependencies_with_balance(&[coin(100, "ATOM")]);
        let connection = CwIbcConnection::default();
        let store = deps.as_mut().storage;
        connection
            .set_xcall_host(store, Addr::unchecked("xcall-address"))
            .unwrap();
        connection
            .set_ibc_host(store, Addr::unchecked("ibc-host"))
            .unwrap();
        let _env = mock_env();

        let packet = RawPacket::default();

        let res = connection.call_host_send_message(deps.as_mut(), packet.clone());
        println!("{res:?}");
        assert!(res.is_ok());

        let expected_ibc_host = connection
            .get_ibc_host(deps.as_ref().storage)
            .unwrap()
            .to_string();
        let expected_message = cw_common::core_msg::ExecuteMsg::SendPacket {
            packet: HexString::from_bytes(&packet.encode_to_vec()),
        };
        let expected_sub_msg = SubMsg {
            id: HOST_SEND_MESSAGE_REPLY_ID,
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: expected_ibc_host,
                msg: to_binary(&expected_message).unwrap(),
                funds: vec![],
            }),
            gas_limit: None,
            reply_on: cosmwasm_std::ReplyOn::Always,
        };
        assert_eq!(res.unwrap(), expected_sub_msg);
    }
}
