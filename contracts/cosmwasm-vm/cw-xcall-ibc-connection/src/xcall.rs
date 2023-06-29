use crate::types::LOG_PREFIX;
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, Empty, MessageInfo, Storage, SubMsg, WasmMsg};
use cw_common::{
    hex_string::HexString, raw_types::channel::RawPacket, xcall_types::network_address::NetId,
    ProstMessage,
};
use debug_print::debug_println;

use crate::{
    error::ContractError,
    state::{
        CwIbcConnection, HOST_SEND_MESSAGE_REPLY_ID, XCALL_HANDLE_ERROR_REPLY_ID,
        XCALL_HANDLE_MESSAGE_REPLY_ID,
    },
};

impl<'a> CwIbcConnection<'a> {
    pub fn call_xcall_handle_message(
        &self,
        store: &dyn Storage,
        nid: &NetId,
        msg: Vec<u8>,
        sn: Option<i64>,
    ) -> Result<SubMsg, ContractError> {
        let xcall_host = self.get_xcall_host(store)?;
        let xcall_msg = cw_common::xcall_app_msg::ExecuteMsg::HandleCallMessage {
            msg,
            sn,
            from: nid.clone(),
        };
        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: xcall_host.to_string(),
            msg: to_binary(&xcall_msg).unwrap(),
            funds: vec![],
        });
        let sub_msg: SubMsg = SubMsg::reply_always(call_message, XCALL_HANDLE_MESSAGE_REPLY_ID);
        Ok(sub_msg)
    }

    pub fn call_xcall_handle_error(
        &self,
        store: &dyn Storage,
        sn: i64,
        code: i64,
        msg: String,
    ) -> Result<SubMsg, ContractError> {
        let xcall_host = self.get_xcall_host(store)?;
        let xcall_msg = cw_common::xcall_app_msg::ExecuteMsg::HandleError { sn, code, msg };
        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: xcall_host.to_string(),
            msg: to_binary(&xcall_msg).unwrap(),
            funds: vec![],
        });
        let sub_msg: SubMsg = SubMsg::reply_always(call_message, XCALL_HANDLE_ERROR_REPLY_ID);
        Ok(sub_msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_dependencies_with_balance, mock_env};
    use cosmwasm_std::{coin, Addr, CosmosMsg};

    #[test]
    fn test_call_xcall_handle_message() {
        let mut deps = mock_dependencies();
        let connection = CwIbcConnection::default();
        let store = deps.as_mut().storage;
        let msg = vec![1, 2, 3];
        let sn = Some(42);
        connection
            .set_xcall_host(store, Addr::unchecked("xcall-address"))
            .unwrap();
        let nid = NetId::from("nid".to_string());
        let res = connection.call_xcall_handle_message(store, &nid.clone(), msg.clone(), sn);
        assert!(res.is_ok());

        let expected_xcall_host = connection.get_xcall_host(store).unwrap().to_string();
        let expected_xcall_msg =
            cw_common::xcall_app_msg::ExecuteMsg::HandleCallMessage { msg, sn, from: nid };
        let expected_call_message = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: expected_xcall_host,
            msg: to_binary(&expected_xcall_msg).unwrap(),
            funds: vec![],
        });
        let expected_sub_msg =
            SubMsg::reply_always(expected_call_message, XCALL_HANDLE_MESSAGE_REPLY_ID);
        assert_eq!(res.unwrap(), expected_sub_msg);
    }

    #[test]
    fn test_call_xcall_handle_error() {
        let mut deps = mock_dependencies();
        let connection = CwIbcConnection::default();
        let store = deps.as_mut().storage;
        connection
            .set_xcall_host(store, Addr::unchecked("xcall-address"))
            .unwrap();
        let store = deps.storage;
        let sn = 42;
        let code = 500;
        let msg = "error message".to_string();

        let res = connection.call_xcall_handle_error(&store, sn, code, msg.clone());
        assert!(res.is_ok());

        let expected_xcall_host = connection.get_xcall_host(&store).unwrap().to_string();
        let expected_xcall_msg =
            cw_common::xcall_app_msg::ExecuteMsg::HandleError { sn, code, msg: msg };
        let expected_call_message = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: expected_xcall_host,
            msg: to_binary(&expected_xcall_msg).unwrap(),
            funds: vec![],
        });
        let expected_sub_msg =
            SubMsg::reply_always(expected_call_message, XCALL_HANDLE_ERROR_REPLY_ID);
        assert_eq!(res.unwrap(), expected_sub_msg);
    }

    #[test]
    fn test_call_xcall_send_message() {
        let mut deps = mock_dependencies_with_balance(&[coin(100, "ATOM")]);
        let connection = CwIbcConnection::default();
        let store = deps.as_mut().storage;
        connection
            .set_xcall_host(store, Addr::unchecked("xcall-address"))
            .unwrap();
        connection
            .set_ibc_host(store, Addr::unchecked("ibc-host"))
            .unwrap();
        let env = mock_env();
        let info = MessageInfo {
            sender: env.contract.address,
            funds: vec![coin(100, "ATOM")],
        };
        let packet = RawPacket::default();

        let res = connection.call_host_send_message(deps.as_mut(), info, packet.clone());
        println!("{:?}", res);
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
                funds: vec![coin(100, "ATOM")],
            }),
            gas_limit: None,
            reply_on: cosmwasm_std::ReplyOn::Always,
        };
        assert_eq!(res.unwrap(), expected_sub_msg);
    }
}
