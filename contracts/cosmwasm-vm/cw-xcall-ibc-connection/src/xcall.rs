use cosmwasm_std::{to_json_binary as to_binary, CosmosMsg, Empty, Storage, SubMsg, WasmMsg};
use cw_xcall_lib::network_address::NetId;

use crate::{
    error::ContractError,
    state::{CwIbcConnection, XCALL_HANDLE_ERROR_REPLY_ID, XCALL_HANDLE_MESSAGE_REPLY_ID},
};

impl<'a> CwIbcConnection<'a> {
    pub fn call_xcall_handle_message(
        &self,
        store: &dyn Storage,
        nid: &NetId,
        msg: Vec<u8>,
    ) -> Result<SubMsg, ContractError> {
        let xcall_host = self.get_xcall_host(store)?;
        let xcall_msg = cw_xcall_lib::xcall_msg::ExecuteMsg::HandleMessage {
            from_nid: nid.clone(),
            msg,
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
    ) -> Result<SubMsg, ContractError> {
        let xcall_host = self.get_xcall_host(store)?;
        let xcall_msg = cw_xcall_lib::xcall_msg::ExecuteMsg::HandleError {
            sn: sn.try_into().unwrap(),
        };
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
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::{Addr, CosmosMsg};
    use cw_xcall_lib::network_address::NetId;

    #[test]
    fn test_call_xcall_handle_message() {
        let mut deps = mock_dependencies();
        let connection = CwIbcConnection::default();
        let store = deps.as_mut().storage;
        let msg = vec![1, 2, 3];
        connection
            .set_xcall_host(store, Addr::unchecked("xcall-address"))
            .unwrap();
        let nid = NetId::from("nid".to_string());
        let res = connection.call_xcall_handle_message(store, &nid, msg.clone());
        assert!(res.is_ok());

        let expected_xcall_host = connection.get_xcall_host(store).unwrap().to_string();
        let expected_xcall_msg =
            cw_xcall_lib::xcall_msg::ExecuteMsg::HandleMessage { msg, from_nid: nid };
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
        let res = connection.call_xcall_handle_error(&store, sn);
        assert!(res.is_ok());

        let expected_xcall_host = connection.get_xcall_host(&store).unwrap().to_string();
        let expected_xcall_msg = cw_xcall_lib::xcall_msg::ExecuteMsg::HandleError {
            sn: sn.try_into().unwrap(),
        };
        let expected_call_message = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: expected_xcall_host,
            msg: to_binary(&expected_xcall_msg).unwrap(),
            funds: vec![],
        });
        let expected_sub_msg =
            SubMsg::reply_always(expected_call_message, XCALL_HANDLE_ERROR_REPLY_ID);
        assert_eq!(res.unwrap(), expected_sub_msg);
    }
}
