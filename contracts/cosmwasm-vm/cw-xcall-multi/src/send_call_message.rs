use cosmwasm_std::{coins, BankMsg};
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
    pub fn send_call_message(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        _env: Env,
        to: NetworkAddress,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
        sources: Vec<String>,
        destinations: Vec<String>,
    ) -> Result<Response, ContractError> {
        let caller = info.sender;
        let config = self.get_config(deps.as_ref().storage)?;
        let nid = config.network_id;

        self.ensure_caller_is_contract_and_rollback_is_null(
            deps.as_ref(),
            caller.clone(),
            rollback.clone(),
        )?;

        let need_response = rollback.is_some();

        let rollback_data = match rollback {
            Some(data) => data,
            None => vec![],
        };

        self.ensure_rollback_length(&rollback_data)?;
        println!("{LOG_PREFIX} Packet Validated");

        let sequence_no = self.get_next_sn(deps.storage)?;
        let mut confirmed_sources = sources.clone();
        let from = NetworkAddress::new(&nid, caller.as_ref());

        if confirmed_sources.is_empty() {
            let default = self.get_default_connection(deps.as_ref().storage, to.nid())?;
            confirmed_sources = vec![default.to_string()]
        }

        if need_response {
            let request =
                CallRequest::new(caller.clone(), to.clone(), sources, rollback_data, false);

            self.store_call_request(deps.storage, sequence_no, &request)?;
        }

        let call_request = CallServiceMessageRequest::new(
            from,
            to.account(),
            sequence_no,
            need_response,
            data.to_vec(),
            destinations,
        );

        let message: CallServiceMessage = call_request.into();
        let sn: i64 = if need_response { sequence_no as i64 } else { 0 };

        let submessages = confirmed_sources
            .iter()
            .map(|r| {
                return self
                    .query_connection_fee(deps.as_ref(), to.nid(), need_response, r)
                    .and_then(|fee| {
                        let fund = if fee > 0 {
                            coins(fee, config.denom.clone())
                        } else {
                            vec![]
                        };

                        self.call_connection_send_message(
                            &r.to_string(),
                            fund,
                            to.nid(),
                            sn,
                            &message,
                        )
                    });
            })
            .collect::<Result<Vec<SubMsg>, ContractError>>()?;
        let protocol_fee = self.get_protocol_fee(deps.storage);
        let fee_handler = self.fee_handler().load(deps.storage)?;

        let event = event_xcall_message_sent(caller.to_string(), to.to_string(), sequence_no);
        println!("{LOG_PREFIX} Sent Bank Message");
        let mut res = Response::new()
            .add_submessages(submessages)
            .add_attribute("action", "xcall-service")
            .add_attribute("method", "send_packet")
            .add_attribute("sequence_no", sequence_no.to_string())
            .add_event(event);
        if protocol_fee > 0 {
            let msg = BankMsg::Send {
                to_address: fee_handler,
                amount: coins(protocol_fee, config.denom),
            };
            res = res.add_message(msg);
        }
        Ok(res)
    }

    /// This function sends a reply message and returns a response or an error.
    ///
    /// Arguments:
    ///
    /// * `message`: The `message` parameter is of type `Reply`, which is a struct that contains
    /// information about the result of a sub-message that was sent by the contract. It has two fields:
    /// `id`, which is a unique identifier for the sub-message, and `result`, which is an enum that
    /// represents
    ///
    /// Returns:
    ///
    /// The function `reply_sendcall_message` returns a `Result` object, which can either be an `Ok`
    /// variant containing a `Response` object with two attributes ("action" and "method"), or an `Err`
    /// variant containing a `ContractError` object with a code and a message.
    pub fn send_call_message_reply(&self, message: Reply) -> Result<Response, ContractError> {
        println!("{LOG_PREFIX} Received Callback From SendCallMessage");

        match message.result {
            SubMsgResult::Ok(res) => {
                println!("{LOG_PREFIX} Call Success");
                println!("{:?}", res);
                Ok(Response::new()
                    .add_attribute("action", "reply")
                    .add_attribute("method", "sendcall_message"))
            }
            SubMsgResult::Err(error) => {
                println!(
                    "{} SendMessageCall Failed with error {}",
                    LOG_PREFIX, &error
                );
                Err(ContractError::ReplyError {
                    code: message.id,
                    msg: error,
                })
            }
        }
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
