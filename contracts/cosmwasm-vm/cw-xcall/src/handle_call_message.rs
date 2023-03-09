use super::*;

impl<'a> CwCallService<'a> {
    pub fn execute_call(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        request_id: u128,
    ) -> Result<Response, ContractError> {
        let proxy_reqs = self
            .query_message_request(deps.storage, request_id)
            .unwrap();

        self.ensure_request_not_null(request_id, &proxy_reqs)
            .unwrap();

        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: proxy_reqs.to().to_string(),
            msg: proxy_reqs.data().into(),
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, EXECUTE_CALL_ID);

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_call")
            .add_submessage(sub_msg))
    }

    pub fn execute_rollback(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        sequence_no: u128,
    ) -> Result<Response, ContractError> {
        let call_request = self.query_request(deps.storage, sequence_no)?;

        self.enusre_call_request_not_null(sequence_no, &call_request)
            .unwrap();
        self.ensure_rollback_enabled(call_request.enabled())
            .unwrap();

        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: call_request.to().to_string(),
            msg: call_request.rollback().into(),
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, EXECUTE_ROLLBACK_ID);

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_call")
            .add_submessage(sub_msg))
    }
}
