use crate::types::{config::Config, LOG_PREFIX};

use super::*;
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-xcall-app";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwCallService<'a> {
    /// This function instantiates a contract and initializes it with the provided message and
    /// information.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is short for "dependencies mutable". It is a struct
    /// that provides access to the contract's dependencies, such as the storage, API, and querier. The
    /// `DepsMut` object is passed as a parameter to most of the
    /// * `_env`: The `_env` parameter in the `instantiate` function is of type `Env`, which represents
    /// the environment in which the contract is being executed. It contains information such as the
    /// current block height, the current time, and the address of the contract being executed. However,
    /// in the given code snippet
    /// * `info`: `info` is a struct that contains information about the message sender, such as their
    /// address, the amount of tokens they sent with the message, and the maximum amount of gas they are
    /// willing to pay for the transaction. This information can be used to determine whether the sender
    /// is authorized to perform certain actions
    /// * `msg`: The `msg` parameter in the `instantiate` function is of type `InstantiateMsg` and
    /// contains the message sent by the user when instantiating the contract. It can contain any custom
    /// data that the user wants to pass to the contract during instantiation. The `msg` parameter is
    /// used by the
    ///
    /// Returns:
    ///
    /// The `instantiate` function returns a `Result<Response, ContractError>` where `Response` is a
    /// struct representing the response to a message and `ContractError` is an enum representing the
    /// possible errors that can occur during contract execution. The function returns the result of
    /// calling the `init` function with the provided arguments.
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        self.init(deps.storage, info, msg)
    }

    /// This function executes various messages based on their type and returns a response or an error.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct which provides access to the contract's dependencies such
    /// as storage, API, and querier. It is mutable, meaning the contract can modify its dependencies.
    /// * `env`: `env` is a struct that contains information about the current blockchain environment,
    /// such as the block height, time, and chain ID. It is passed as a parameter to the `execute`
    /// function in order to provide context for the execution of the contract.
    /// * `info`: `info` is a struct that contains information about the message sender, including their
    /// address, public key, and the amount of tokens they sent with the message. It is of type
    /// `MessageInfo`.
    /// * `msg`: The `msg` parameter is of type `ExecuteMsg` and represents the message that is being
    /// executed by the contract. It is matched against different variants to determine the action to be
    /// taken.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing an error that occurred during contract
    /// execution.
    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::SetAdmin { address } => {
                let validated_address =
                    CwCallService::validate_address(deps.api, address.as_str())?;
                self.add_admin(deps.storage, info, validated_address)
            }
            ExecuteMsg::SetProtocolFee { value } => {
                self.set_protocol_fee(deps, info, value).unwrap();
                Ok(Response::new())
            }
            ExecuteMsg::SetProtocolFeeHandler { address } => {
                self.set_protocol_feehandler(deps, env, info, address)
            }
            ExecuteMsg::SendCallMessage {
                to,
                sources,
                destinations,
                data,
                rollback,
            } => {
                println!("{LOG_PREFIX} Received Send Call Message");
                let sources = sources.unwrap_or(vec![]);
                let dests = destinations.unwrap_or(vec![]);
                self.send_call_message(deps, info, env, to, sources, dests, data, rollback)
            }
            ExecuteMsg::HandleCallMessage { msg, from, sn } => {
                self.receive_packet_data(deps, info, from, sn, msg)
            }
            ExecuteMsg::HandleError { sn, code, msg } => {
                todo!()
            }
            ExecuteMsg::ExecuteCall { request_id } => self.execute_call(deps, info, request_id),
            ExecuteMsg::ExecuteRollback { sequence_no } => {
                self.execute_rollback(deps, info, sequence_no)
            }
            ExecuteMsg::UpdateAdmin { address } => {
                let validated_address =
                    CwCallService::validate_address(deps.api, address.as_str())?;
                self.update_admin(deps.storage, info, validated_address)
            }
            ExecuteMsg::RemoveAdmin {} => self.remove_admin(deps.storage, info),

            ExecuteMsg::SetTimeoutHeight { height } => {
                self.ensure_admin(deps.as_ref().storage, info.sender)?;

                self.store_timeout_height(deps.storage, height)?;

                Ok(Response::new()
                    .add_attribute("method", "set_timeout_height")
                    .add_attribute("timeout_hieght", height.to_string()))
            }
            #[cfg(feature = "native_ibc")]
            _ => Err(ContractError::DecodeFailed {
                error: "InvalidMessage Variant".to_string(),
            }),
        }
    }

    /// The `query` function takes in dependencies, environment, and a message, and returns a binary
    /// result based on the type of message received.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is an object of type `Deps` which contains various dependencies required for
    /// executing the contract. These dependencies include the storage, API, and querier. The `deps`
    /// object is passed as an argument to the `query` function to access these dependencies.
    /// * `_env`: The `_env` parameter is an instance of the `Env` struct, which provides information
    /// about the current blockchain environment, such as the block height and time. However, it is not
    /// used in this particular `query` function.
    /// * `msg`: `msg` is a parameter of type `QueryMsg` which is an enum that represents different types
    /// of queries that can be made to the smart contract. The function matches on the variant of
    /// `QueryMsg` that is passed in and executes the corresponding logic.
    ///
    /// Returns:
    ///
    /// The `query` function returns a `StdResult<Binary>` which can contain either the binary
    /// representation of the result of the query or an error if the query fails. The specific result
    /// being returned depends on the type of `QueryMsg` being passed in and the logic of the
    /// corresponding match arm.
    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetAdmin {} => match self.query_admin(deps.storage) {
                Ok(admin) => Ok(to_binary(&admin)?),
                Err(error) => Err(StdError::NotFound {
                    kind: error.to_string(),
                }),
            },

            QueryMsg::GetProtocolFee {} => to_binary(&self.get_protocol_fee(deps.storage).unwrap()),
            QueryMsg::GetProtocolFeeHandler {} => to_binary(&self.get_protocol_feehandler(deps)),
            QueryMsg::GetTimeoutHeight {} => to_binary(&self.get_timeout_height(deps.storage)),
        }
    }
    /// This function handles different types of reply messages and calls corresponding functions based on
    /// the message ID.
    ///
    /// Arguments:
    ///
    /// * `deps`: A mutable reference to the dependencies of the contract, which includes access to the
    /// storage, API, and other modules.
    /// * `env`: `env` is an environment variable that provides information about the current execution
    /// environment of the smart contract. It includes information such as the current block height, the
    /// sender address, the contract address, and the current time. This information can be used by the
    /// smart contract to make decisions or perform actions based on
    /// * `msg`: `msg` is a `Reply` struct that contains the message ID and any associated data that was
    /// sent as a reply to a previous message. The `reply` function uses the message ID to determine which
    /// specific reply message was received and then calls the appropriate function to handle that message.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is the response to the message and
    /// `ContractError` is an error type that can be returned if there is an error in processing the
    /// message.

    pub fn reply(&self, deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.id {
            EXECUTE_CALL_ID => self.reply_execute_call_message(deps.as_ref(), env, msg),
            EXECUTE_ROLLBACK_ID => self.reply_execute_rollback(deps.as_ref(), msg),
            SEND_CALL_MESSAGE_REPLY_ID => self.reply_sendcall_message(msg),
            ACK_FAILURE_ID => self.reply_ack_on_error(msg),
            _ => Err(ContractError::ReplyError {
                code: msg.id,
                msg: "Unknown".to_string(),
            }),
        }
    }

    pub fn validate_send_call(
        &self,
        deps: Deps,
        nid: &str,
        sources: &Vec<String>,
        destinations: &Vec<String>,
        rollback: &Option<Vec<u8>>,
        info: &MessageInfo,
    ) -> Result<(), ContractError> {
        if sources.len() != destinations.len() {
            return Err(ContractError::ProtocolsMismatch);
        }
        let has_rollback = rollback.is_some();
        let fees = sources
            .iter()
            .map(|r| self.get_total_required_fee(deps, nid, has_rollback, sources))
            .collect::<Result<Vec<u128>, ContractError>>()?;

        let total_required_fee: u128 = fees.iter().sum();
        self.ensure_enough_funds(total_required_fee, info)?;
        Ok(())
    }
}

impl<'a> CwCallService<'a> {
    /// This function initializes the contract with default values and sets various parameters such as
    /// the timeout height and IBC host.
    ///
    /// Arguments:
    ///
    /// * `store`: A mutable reference to a trait object that implements the `Storage` trait. This is
    /// used to store and retrieve data from the contract's storage.
    /// * `info`: `info` is a struct that contains information about the message sender, such as their
    /// address and the amount of tokens they sent with the message. It is of type `MessageInfo`.
    /// * `msg`: InstantiateMsg is a struct that contains the parameters passed during contract
    /// instantiation. It is defined somewhere in the code and likely contains fields such as
    /// `timeout_height` and `ibc_host`.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can
    /// occur during contract execution.
    fn init(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let last_sequence_no = u128::default();
        let last_request_id = u128::default();
        let owner = info.sender.as_str().to_string();

        self.add_owner(store, owner.clone())?;
        self.add_admin(store, info, owner)?;
        self.init_last_sequence_no(store, last_sequence_no)?;
        self.init_last_request_id(store, last_request_id)?;
        self.store_config(
            store,
            &Config {
                network_id: msg.network_id,
                denom: msg.denom,
            },
        )?;
        // self.set_timeout_height(store, msg.timeout_height)?;
        // self.set_connection_host(store, msg.connection_host.clone())?;

        Ok(Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("method", "init"))
    }

    /// This function handles the response of a call to a service and generates a response with an
    /// event.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is an instance of the `Deps` struct, which provides access to the contract's
    /// dependencies such as storage, API, and context.
    /// * `msg`: `msg` is a `Reply` struct that contains the result of a sub-message that was sent by
    /// the contract to another contract or external system. It is used to construct a
    /// `CallServiceMessageResponse` that will be returned as part of the `Response` to the original
    /// message that triggered the
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to be
    /// returned by the contract and `ContractError` is an enum representing any errors that may occur
    /// during contract execution.
    fn reply_execute_rollback(&self, deps: Deps, msg: Reply) -> Result<Response, ContractError> {
        let sn = self.get_current_sn(deps.storage)?;

        let response = match msg.result {
            cosmwasm_std::SubMsgResult::Ok(_res) => CallServiceMessageResponse::new(
                sn,
                CallServiceResponseType::CallServiceResponseSuccess,
                "",
            ),
            cosmwasm_std::SubMsgResult::Err(err) => {
                let error_message = format!("CallService Reverted : {err}");
                CallServiceMessageResponse::new(
                    sn,
                    CallServiceResponseType::CallServiceResponseFailure,
                    &error_message,
                )
            }
        };

        let event = event_rollback_executed(
            sn,
            to_int(response.response_code()),
            &to_string(response.message()).unwrap(),
        );

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_rollback")
            .add_event(event))
    }

    #[allow(unused_variables)]
    fn reply_execute_call_message(
        &self,
        deps: Deps,
        env: Env,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let req_id = self.last_request_id().load(deps.storage)?;
        let request = self.message_request().load(deps.storage, req_id)?;

        let responses = match msg.result {
            cosmwasm_std::SubMsgResult::Ok(_res) => {
                let code = 0;

                let message_response = CallServiceMessageResponse::new(
                    request.sequence_no(),
                    CallServiceResponseType::CallServiceResponseSuccess,
                    "success",
                );
                let event = event_call_executed(req_id, code, message_response.message());
                (message_response, event)
            }
            cosmwasm_std::SubMsgResult::Err(err) => {
                let code = -1;
                let error_message = format!("CallService Reverted : {err}");
                let message_response = CallServiceMessageResponse::new(
                    request.sequence_no(),
                    CallServiceResponseType::CallServiceResponseFailure,
                    &error_message,
                );
                let event = event_call_executed(req_id, code, &error_message);
                (message_response, event)
            }
        };

        if !request.rollback() {
            let message: CallServiceMessage = responses.0.into();

            #[cfg(feature = "native_ibc")]
            {
                let packet = self.create_packet_response(deps, env, to_binary(&message).unwrap());

                return Ok(Response::new()
                    .add_attribute("action", "call_message")
                    .add_attribute("method", "execute_callback")
                    .add_message(packet));
            }
        }

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_callback")
            .add_event(responses.1))
    }

    #[cfg(feature = "native_ibc")]
    fn create_packet_response(&self, deps: Deps, env: Env, data: Binary) -> IbcMsg {
        let ibc_config = self.ibc_config().may_load(deps.storage).unwrap().unwrap();

        let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(300));

        IbcMsg::SendPacket {
            channel_id: ibc_config.dst_endpoint().channel_id.clone(),
            data,
            timeout,
        }
    }
    fn reply_ack_on_error(&self, reply: Reply) -> Result<Response, ContractError> {
        match reply.result {
            SubMsgResult::Ok(_) => Ok(Response::new()),
            SubMsgResult::Err(err) => Ok(Response::new().set_data(make_ack_fail(err))),
        }
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
    fn reply_sendcall_message(&self, message: Reply) -> Result<Response, ContractError> {
        println!("{LOG_PREFIX} Received Callback From SendCallMessage");

        match message.result {
            SubMsgResult::Ok(_) => {
                println!("{LOG_PREFIX} Call Success");
                Ok(Response::new()
                    .add_attribute("action", "reply")
                    .add_attribute("method", "sendcall_message")
                    .add_event(
                        Event::new("xcall_app_send_call_message_reply")
                            .add_attribute("status", "success"),
                    ))
            }
            SubMsgResult::Err(error) => {
                println!("{} Call Failed with error {}", LOG_PREFIX, &error);
                Err(ContractError::ReplyError {
                    code: message.id,
                    msg: error,
                })
            }
        }
    }
}
