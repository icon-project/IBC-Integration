use std::str::FromStr;

use cw_common::xcall_types::network_address::NetworkAddress;

use crate::types::LOG_PREFIX;

use super::*;

impl<'a> CwCallService<'a> {
    /// This function sends a cross-chain call packet to another contract and handles the response if
    /// needed.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct, which provides mutable access to the contract's
    /// dependencies such as storage, querier, and API. It is used to interact with the blockchain and
    /// other contracts.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being executed, such as the sender address, the amount of funds sent with the message, and the
    /// gas limit.
    /// * `env`: `env` is a parameter of type `Env` which contains information about the current
    /// blockchain environment, such as the block height and time. It is used in this function to create
    /// a timeout height for the IBC packet being sent.
    /// * `to`: The `to` parameter is a string representing the address of the contract or module that
    /// the packet is being sent to.
    /// * `data`: `data` is a vector of bytes representing the message payload to be sent in the packet.
    /// It can contain any data that can be serialized into bytes, such as JSON, protobuf, or custom
    /// binary formats. The contents and format of the payload are determined by the specific use case
    /// and protocol being used
    /// * `rollback`: `rollback` is an optional `Vec<u8>` parameter that represents the data to be used
    /// for rolling back the transaction in case of an error. If this parameter is `Some`, it means that
    /// the transaction needs to be rolled back in case of an error, and the provided `Vec<u8
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing possible errors that can occur
    /// during contract execution.
    pub fn send_packet(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        _env: Env,
        to: String,
        sources: Vec<String>,
        destinations: Vec<String>,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
    ) -> Result<Response, ContractError> {
        let from_address = info.sender.to_string();

        self.ensure_caller_is_contract_and_rollback_is_null(
            deps.as_ref(),
            info.sender.clone(),
            rollback.clone(),
        )?;

        let need_response = rollback.is_some();

        let rollback_data = match rollback {
            Some(data) => data,
            None => vec![],
        };

        self.ensure_data_length(data.len())?;
        self.ensure_rollback_length(&rollback_data)?;
        println!("{LOG_PREFIX} Packet Validated");

        // TODO : ADD fee logic

        let sequence_no = self.increment_last_sequence_no(deps.storage)?;
        let mut confirmed_sources = sources;

        if confirmed_sources.is_empty() {
            let na = NetworkAddress::from_str(&to)?;
            let default = self.get_default_connection(deps.as_ref().storage, na.get_nid())?;
            confirmed_sources = vec![default.to_string()]
        }

        if need_response {
            let request = CallRequest::new(
                from_address,
                to.clone(),
                destinations.clone(),
                rollback_data,
                need_response,
            );

            self.set_call_request(deps.storage, sequence_no, request)?;
        }

        let call_request = CallServiceMessageRequest::new(
            info.sender.to_string(),
            to.clone(),
            sequence_no,
            destinations,
            need_response,
            data.to_vec(),
        );

        let message: CallServiceMessage = call_request.into();

        let event = event_xcall_message_sent(info.sender.to_string(), sequence_no, &message);

        let message = cw_common::xcall_connection_msg::ExecuteMsg::SendMessage {
            to,
            sn: 0,
            msg: to_vec(&message).unwrap(),
        };

        let submessages = confirmed_sources
            .iter()
            .map(|r| {
                let cosm_msg = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: r.to_string(),
                    msg: to_binary(&message).map_err(ContractError::Std)?,
                    funds: info.funds.clone(),
                });
                let submessage = SubMsg {
                    id: SEND_CALL_MESSAGE_REPLY_ID,
                    msg: cosm_msg,
                    gas_limit: None,
                    reply_on: cosmwasm_std::ReplyOn::Always,
                };
                println!("{LOG_PREFIX} sent message to connection :{r}");
                Ok(submessage)
            })
            .collect::<Result<Vec<SubMsg>, ContractError>>()?;

        Ok(Response::new()
            .add_submessages(submessages)
            .add_attribute("action", "xcall-service")
            .add_attribute("method", "send_packet")
            .add_attribute("sequence_no", sequence_no.to_string())
            .add_event(event))
    }

    pub fn query_protocol_fee(
        &self,
        querier: &QuerierWrapper,
        connection: &str,
    ) -> Result<u128, ContractError> {
        let query_message = cw_common::xcall_connection_msg::QueryMsg::GetProtocolFee {};

        let query_request = QueryRequest::Wasm(cosmwasm_std::WasmQuery::Smart {
            contract_addr: connection.to_string(),
            msg: to_binary(&query_message).map_err(ContractError::Std)?,
        });
        let fee: u128 = querier.query(&query_request).map_err(ContractError::Std)?;
        Ok(fee)
    }
}

#[cfg(feature = "native_ibc")]
impl<'a> CwCallService<'a> {
    /// This function creates an IBC message to send a packet with a timeout to a destination endpoint.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract. It is used to
    /// interact with the storage and other modules of the contract.
    /// * `env`: `env` is an object that contains information about the current blockchain environment,
    /// such as the current block height, time, and chain ID. It is used to calculate the timeout for the
    /// IBC packet.
    /// * `time_out_height`: The height of the block at which the timeout for the packet will occur.
    /// * `message`: `message` is a `CallServiceMessage` struct that contains the information needed to
    /// create a request packet to be sent over the IBC channel. This includes the method name, input
    /// arguments, and any other relevant data needed for the service call.
    ///
    /// Returns:
    ///
    /// a `Result` with an `IbcMsg` on success or a `ContractError` on failure.
    fn create_request_packet(
        &self,
        deps: DepsMut,
        env: Env,
        time_out_height: u64,
        message: CallServiceMessage,
    ) -> Result<IbcMsg, ContractError> {
        let ibc_config = self
            .ibc_config()
            .load(deps.as_ref().storage)
            .map_err(ContractError::Std)?;

        let timeout_block = IbcTimeoutBlock {
            revision: 0,
            height: time_out_height,
        };
        let timeout = IbcTimeout::with_both(timeout_block, env.block.time.plus_seconds(300));

        Ok(IbcMsg::SendPacket {
            channel_id: ibc_config.dst_endpoint().channel_id.clone(),
            data: to_binary(&message).unwrap(),
            timeout,
        })
    }
}
