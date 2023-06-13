use super::*;
pub mod open_init;
pub mod open_try;
use self::{
    open_init::{
        channel_open_init_msg_validate, create_channel_submesssage, on_chan_open_init_submessage,
    },
    open_try::channel_open_try_msg_validate,
};

use cw_common::{commitment, from_binary_response, raw_types::channel::RawChannel};
pub mod close_init;
use close_init::*;
pub mod open_ack;
use debug_print::debug_println;
use open_ack::*;
pub mod open_confirm;
use open_confirm::*;

pub mod close_confirm;
pub use close_confirm::*;
use prost::Message;

impl<'a> ValidateChannel for CwIbcCoreContext<'a> {
    /// This function validates a channel open initialization message and generates an event for calling
    /// on channel open init in x-call.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides access to the contract's dependencies
    /// such as storage, API, and querier. It is used to interact with the blockchain and other
    /// contracts.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// sender, such as the sender's address, the amount of funds sent with the message, and the gas
    /// limit for executing the message.
    /// * `message`: The `message` parameter is a reference to a `MsgChannelOpenInit` struct, which
    /// contains the details of the channel opening initialization message being validated.
    ///
    /// Returns:
    ///
    /// A `Result` containing either a `cosmwasm_std::Response` or a `ContractError`.
    fn validate_channel_open_init(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelOpenInit,
    ) -> Result<cosmwasm_std::Response, ContractError> {
        // connection hops should be 1
        debug_println!(
            "inside validate channel open init: input parameter: {:?}",
            message
        );

        if message.connection_hops_on_a.len() != 1 {
            return Err(ChannelError::InvalidConnectionHopsLength {
                expected: 1,
                actual: message.connection_hops_on_a.len(),
            })
            .map_err(Into::<ContractError>::into)?;
        }
        let connection_id = message.connection_hops_on_a[0].clone();
        // An IBC connection running on the local (host) chain should exist.
        let conn_end_on_a = self.connection_end(deps.storage, connection_id.clone())?;
        channel_open_init_msg_validate(message, conn_end_on_a)?;
        let counter = match self.channel_counter(deps.storage) {
            Ok(counter) => counter,
            Err(error) => return Err(error),
        };
        let channel_id_on_a = ChannelId::new(counter); // creating new channel_id
        let contract_address = match self.lookup_modules(
            deps.storage,
            message.port_id_on_a.clone().as_bytes().to_vec(),
        ) {
            Ok(addr) => addr,
            Err(error) => return Err(error),
        };

        debug_println!("contract address is : {:?} ", contract_address);
        // let module_id = cw_common::ibc_types::IbcModuleId::from_str(module_id.as_str());
        // let contract_address = match self.get_route(deps.storage, ) {
        //     Ok(addr) => addr,
        //     Err(error) => return Err(error),
        // };
        // Store the channel details
        let counter_party = Counterparty::new(message.port_id_on_b.clone(), None);
        let channel_end = ChannelEnd::new(
            State::Uninitialized,
            message.ordering,
            counter_party,
            message.connection_hops_on_a.clone(),
            message.version_proposal.clone(),
        );
        self.store_channel_end(
            deps.storage,
            message.port_id_on_a.clone(),
            channel_id_on_a.clone(),
            channel_end,
        )?;

        // Generate event for calling on channel open init in x-call
        let sub_message = on_chan_open_init_submessage(message, &channel_id_on_a, &connection_id);
        let data = cw_common::xcall_msg::ExecuteMsg::IbcChannelOpen { msg: sub_message };
        let data = to_binary(&data).unwrap();
        let on_chan_open_init = create_channel_submesssage(
            contract_address,
            data,
            info.funds,
            EXECUTE_ON_CHANNEL_OPEN_INIT,
        );

        Ok(Response::new()
            .add_attribute("action", "channel")
            .add_attribute("method", "channel_open_init_validation")
            .add_submessage(on_chan_open_init))
    }

    /// This function validates and creates a new channel end for a channel open try message, and sends
    /// a submessage to a light client to verify the channel state.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// sender and the funds sent with the message. It is passed as an argument to the function
    /// `validate_channel_open_try` in the code snippet provided.
    /// * `message`: The `message` parameter is a reference to a `MsgChannelOpenTry` struct, which
    /// contains the information needed to try opening a new channel between two IBC-connected chains.
    ///
    /// Returns:
    ///
    /// A `Result<Response, ContractError>` is being returned.
    fn validate_channel_open_try(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelOpenTry,
    ) -> Result<Response, ContractError> {
        if message.connection_hops_on_b.len() != 1 {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::InvalidConnectionHopsLength {
                    expected: 1,
                    actual: message.connection_hops_on_b.len(),
                },
            });
        }
        debug_println!("Reached in channel open try");
        let connection_id = message.connection_hops_on_b[0].clone();
        let conn_end_on_b = self.connection_end(deps.storage, connection_id)?;

        channel_open_try_msg_validate(message, &conn_end_on_b)?;
        debug_println!("channel open try msg validate ");

        let counter = match self.channel_counter(deps.storage) {
            Ok(counter) => counter,
            Err(error) => return Err(error),
        };
        let channel_id_on_b = ChannelId::new(counter); // creating new channel_id

        let counter_party = Counterparty::new(
            message.port_id_on_a.clone(),
            Some(message.chan_id_on_a.clone()),
        );

        let channel_end = ChannelEnd::new(
            State::Uninitialized,
            message.ordering,
            counter_party,
            message.connection_hops_on_b.clone(),
            message.version_supported_on_a.clone(),
        );
        debug_println!(
            "stoed: channel id: {:?}  portid :{:?} channel_end :{:?}",
            channel_id_on_b,
            message.port_id_on_b,
            channel_end
        );
        self.store_channel_end(
            deps.storage,
            message.port_id_on_b.clone(),
            channel_id_on_b.clone(),
            channel_end,
        )?;

        let client_id_on_b = conn_end_on_b.client_id();
        let client_state_of_a_on_b = self.client_state(deps.storage, client_id_on_b)?;
        let consensus_state_of_a_on_b =
            self.consensus_state(deps.storage, client_id_on_b, &message.proof_height_on_a)?;
        let prefix_on_a = conn_end_on_b.counterparty().prefix();
        let port_id_on_a = message.port_id_on_a.clone();
        let chan_id_on_a = message.chan_id_on_a.clone();
        let conn_id_on_a =
            conn_end_on_b
                .counterparty()
                .connection_id()
                .ok_or(ContractError::IbcChannelError {
                    error: ChannelError::UndefinedConnectionCounterparty {
                        connection_id: message.connection_hops_on_b[0].clone(),
                    },
                })?;

        if client_state_of_a_on_b.is_frozen() {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::FrozenClient {
                    client_id: client_id_on_b.clone(),
                },
            });
        }

        debug_println!("after frozen check");
        let expected_chan_end_on_a = ChannelEnd::new(
            State::Init,
            message.ordering,
            Counterparty::new(message.port_id_on_b.clone(), None),
            vec![conn_id_on_a.clone()],
            message.version_supported_on_a.clone(),
        );
        let raw_expected_chan = RawChannel::try_from(expected_chan_end_on_a).unwrap();
        let chan_end_path_on_a = commitment::channel_path(&port_id_on_a, &chan_id_on_a);
        let vector = raw_expected_chan.encode_to_vec();

        // let fee = self.calculate_fee(GAS_FOR_SUBMESSAGE_LIGHTCLIENT);

        debug_println!("after fee calculatation");
        // let funds = self.update_fee(info.funds.clone(), fee)?;
        // debug_println!("after funding update");

        let create_client_message = LightClientMessage::VerifyChannel {
            endpoint: CwEndPoint {
                port_id: message.port_id_on_b.clone().to_string(),
                channel_id: channel_id_on_b.to_string(),
            },
            message_info: cw_common::types::MessageInfo {
                sender: info.sender,
                funds: vec![],
            },
            verify_channel_state: VerifyChannelState {
                proof_height: message.proof_height_on_a.to_string(),
                counterparty_prefix: prefix_on_a.clone().into_vec(),
                proof: message.proof_chan_end_on_a.clone().into(),
                root: consensus_state_of_a_on_b.clone().root().into_vec(),
                counterparty_chan_end_path: chan_end_path_on_a,
                expected_counterparty_channel_end: vector,
                client_id: conn_end_on_b.client_id().to_string(),
            },
        };
        let client_type = client_state_of_a_on_b.client_type();

        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type)?;
        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&create_client_message).map_err(ContractError::Std)?,
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_always(
            create_client_message,
            EXECUTE_ON_CHANNEL_OPEN_TRY_ON_LIGHT_CLIENT,
        )
        .with_gas_limit(GAS_FOR_SUBMESSAGE_LIGHTCLIENT);

        debug_println!("before calling light client ");
        Ok(Response::new()
            .add_attribute("action", "Light client channel open try call")
            .add_submessage(sub_msg))
    }

    /// This function validates a channel open acknowledgement message and creates a sub-message to
    /// execute on a light client.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides access to the contract's storage and
    /// other dependencies that can be mutated.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being processed, such as the sender and the amount of funds sent with the message.
    /// * `message`: The `message` parameter is a reference to a `MsgChannelOpenAck` struct, which
    /// contains information about a channel open acknowledgement message.
    ///
    /// Returns:
    ///
    /// a `Result` object with either a `Response` or a `ContractError`.
    fn validate_channel_open_ack(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelOpenAck,
    ) -> Result<Response, ContractError> {
        let mut chan_end_on_a = self.get_channel_end(
            deps.storage,
            message.port_id_on_a.clone(),
            message.chan_id_on_a.clone(),
        )?;
        channel_open_ack_validate(message, &chan_end_on_a)?;
        let conn_end_on_a =
            self.connection_end(deps.storage, chan_end_on_a.connection_hops()[0].clone())?;
        if !conn_end_on_a.state_matches(&ConnectionState::Open) {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::ConnectionNotOpen {
                    connection_id: chan_end_on_a.connection_hops()[0].clone(),
                },
            });
        }
        let client_id_on_a = conn_end_on_a.client_id();

        let client_state_of_b_on_a = self.client_state(deps.storage, client_id_on_a)?;
        let consensus_state_of_b_on_a =
            self.consensus_state(deps.storage, client_id_on_a, &message.proof_height_on_b)?;
        let prefix_on_b = conn_end_on_a.counterparty().prefix();
        let port_id_on_b = &chan_end_on_a.counterparty().port_id;
        let conn_id_on_b =
            conn_end_on_a
                .counterparty()
                .connection_id()
                .ok_or(ContractError::IbcChannelError {
                    error: ChannelError::UndefinedConnectionCounterparty {
                        connection_id: chan_end_on_a.connection_hops()[0].clone(),
                    },
                })?;
        if client_state_of_b_on_a.is_frozen() {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::FrozenClient {
                    client_id: client_id_on_a.clone(),
                },
            });
        }
        let expected_chan_end_on_b = ChannelEnd::new(
            State::TryOpen,
            *chan_end_on_a.ordering(),
            Counterparty::new(
                message.port_id_on_a.clone(),
                Some(message.chan_id_on_a.clone()),
            ),
            vec![conn_id_on_b.clone()],
            message.version_on_b.clone(),
        );

        let raw_expected_chan = RawChannel::try_from(expected_chan_end_on_b).unwrap();
        let chan_end_path_on_b = commitment::channel_path(port_id_on_b, &message.chan_id_on_b);
        let vector = raw_expected_chan.encode_to_vec();

        // let fee = self.calculate_fee(GAS_FOR_SUBMESSAGE_LIGHTCLIENT);
        // let funds = self.update_fee(info.funds.clone(), fee)?;
        let create_client_message = LightClientMessage::VerifyChannel {
            message_info: cw_common::types::MessageInfo {
                sender: info.sender,
                funds: vec![],
            },
            endpoint: CwEndPoint {
                port_id: message.port_id_on_a.clone().to_string(),
                channel_id: message.chan_id_on_a.clone().to_string(),
            },
            verify_channel_state: VerifyChannelState {
                proof_height: message.proof_height_on_b.to_string(),
                counterparty_prefix: prefix_on_b.clone().into_vec(),
                proof: message.proof_chan_end_on_b.clone().into(),
                root: consensus_state_of_b_on_a.clone().root().into_vec(),
                counterparty_chan_end_path: chan_end_path_on_b,
                expected_counterparty_channel_end: vector,
                client_id: conn_end_on_a.client_id().to_string(),
            },
        };
        let client_type = client_state_of_b_on_a.client_type();
        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type)?;
        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&create_client_message).unwrap(),
            funds: info.funds,
        });
        let sub_msg: SubMsg = SubMsg::reply_always(
            create_client_message,
            EXECUTE_ON_CHANNEL_OPEN_ACK_ON_LIGHT_CLIENT,
        )
        .with_gas_limit(GAS_FOR_SUBMESSAGE_LIGHTCLIENT);

        chan_end_on_a.set_version(message.version_on_b.clone());
        chan_end_on_a.set_counterparty_channel_id(message.chan_id_on_b.clone());
        self.store_channel_end(
            deps.storage,
            message.port_id_on_a.clone(),
            message.chan_id_on_a.clone(),
            chan_end_on_a,
        )?;

        Ok(Response::new()
            .add_attribute("action", "Light client channel open ack call")
            .add_submessage(sub_msg))
    }

    /// This function validates a channel open confirmation message and creates a submessage to execute
    /// on a light client.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, API, and other modules that the contract may
    /// use.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being processed, such as the sender's address and the amount of funds sent with the message.
    /// * `message`: The `message` parameter is a reference to a `MsgChannelOpenConfirm` struct, which
    /// contains information about a channel open confirmation message.
    ///
    /// Returns:
    ///
    /// a `Result` object with either a `Response` or a `ContractError`.
    fn validate_channel_open_confirm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelOpenConfirm,
    ) -> Result<Response, ContractError> {
        let chan_end_on_b = self.get_channel_end(
            deps.storage,
            message.port_id_on_b.clone(),
            message.chan_id_on_b.clone(),
        )?;
        channel_open_confirm_validate(message, &chan_end_on_b)?;
        let conn_end_on_b =
            self.connection_end(deps.storage, chan_end_on_b.connection_hops()[0].clone())?;

        if !conn_end_on_b.state_matches(&ConnectionState::Open) {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::ConnectionNotOpen {
                    connection_id: chan_end_on_b.connection_hops()[0].clone(),
                },
            });
        }

        let client_id_on_b = conn_end_on_b.client_id();
        let client_state_of_a_on_b = self.client_state(deps.storage, client_id_on_b)?;
        let consensus_state_of_a_on_b =
            self.consensus_state(deps.storage, client_id_on_b, &message.proof_height_on_a)?;
        let prefix_on_a = conn_end_on_b.counterparty().prefix();
        let port_id_on_a = &chan_end_on_b.counterparty().port_id;
        let chan_id_on_a =
            chan_end_on_b
                .counterparty()
                .channel_id()
                .ok_or(ContractError::IbcChannelError {
                    error: ChannelError::InvalidCounterpartyChannelId,
                })?;
        let conn_id_on_a =
            conn_end_on_b
                .counterparty()
                .connection_id()
                .ok_or(ContractError::IbcChannelError {
                    error: ChannelError::UndefinedConnectionCounterparty {
                        connection_id: chan_end_on_b.connection_hops()[0].clone(),
                    },
                })?;
        if client_state_of_a_on_b.is_frozen() {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::FrozenClient {
                    client_id: client_id_on_b.clone(),
                },
            });
        }
        let expected_chan_end_on_a = ChannelEnd::new(
            State::Open,
            *chan_end_on_b.ordering(),
            Counterparty::new(
                message.port_id_on_b.clone(),
                Some(message.chan_id_on_b.clone()),
            ),
            vec![conn_id_on_a.clone()],
            chan_end_on_b.version.clone(),
        );

        let raw_expected_chan = RawChannel::try_from(expected_chan_end_on_a).unwrap();
        let chan_end_path_on_a = commitment::channel_path(port_id_on_a, chan_id_on_a);

        let vector = raw_expected_chan.encode_to_vec();

        // let fee = self.calculate_fee(GAS_FOR_SUBMESSAGE_LIGHTCLIENT);
        //
        // let funds = self.update_fee(info.funds.clone(), fee)?;
        let create_client_message = LightClientMessage::VerifyChannel {
            message_info: cw_common::types::MessageInfo {
                sender: info.sender,
                funds: vec![],
            },
            endpoint: CwEndPoint {
                port_id: message.port_id_on_b.clone().to_string(),
                channel_id: message.chan_id_on_b.clone().to_string(),
            },
            verify_channel_state: VerifyChannelState {
                proof_height: message.proof_height_on_a.to_string(),
                counterparty_prefix: prefix_on_a.clone().into_vec(),
                proof: message.proof_chan_end_on_a.clone().into(),
                root: consensus_state_of_a_on_b.clone().root().into_vec(),
                counterparty_chan_end_path: chan_end_path_on_a,
                expected_counterparty_channel_end: vector,
                client_id: conn_end_on_b.client_id().to_string(),
            },
        };
        let client_type = client_state_of_a_on_b.client_type();
        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type)?;
        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&create_client_message).unwrap(),
            funds: info.funds,
        });
        let sub_msg: SubMsg = SubMsg::reply_always(
            create_client_message,
            EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_LIGHT_CLIENT,
        )
        .with_gas_limit(GAS_FOR_SUBMESSAGE_LIGHTCLIENT);

        Ok(Response::new()
            .add_attribute("action", "light_client_channel_open_confirm_call")
            .add_submessage(sub_msg))
    }

    /// This function validates a channel close initiation message and creates a submessage to execute
    /// on channel close initiation on xcall.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// sender, including the sender's address, the amount of funds sent with the message, and the gas
    /// limit and price.
    /// * `message`: The `message` parameter is a reference to a `MsgChannelCloseInit` struct, which
    /// contains information about the channel close initiation message being validated.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing the possible errors that can occur during
    /// contract execution.
    fn validate_channel_close_init(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelCloseInit,
    ) -> Result<Response, ContractError> {
        let port_id = message.port_id_on_a.clone();
        let channel_id = message.chan_id_on_a.clone();
        let chan_end_on_a = self.get_channel_end(deps.storage, port_id, channel_id)?;

        channel_close_init_validate(&chan_end_on_a, message)?;
        let connection_id = chan_end_on_a.connection_hops()[0].clone();
        let conn_end_on_a = self.connection_end(deps.storage, connection_id.clone())?;

        if !conn_end_on_a.state_matches(&ConnectionState::Open) {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::ConnectionNotOpen {
                    connection_id: chan_end_on_a.connection_hops()[0].clone(),
                },
            });
        }

        let contract_address =
            match self.lookup_modules(deps.storage, message.port_id_on_a.as_bytes().to_vec()) {
                Ok(addr) => addr,
                Err(error) => return Err(error),
            };

        let sub_message = on_chan_close_init_submessage(message, &chan_end_on_a, &connection_id);
        let data = cw_common::xcall_msg::ExecuteMsg::IbcChannelClose { msg: sub_message };
        let data = to_binary(&data).unwrap();
        let on_chan_close_init = create_channel_submesssage(
            contract_address,
            data,
            info.funds,
            EXECUTE_ON_CHANNEL_CLOSE_INIT,
        );

        Ok(Response::new()
            .add_attribute("action", "channel")
            .add_attribute("method", "channel_close_init_validation")
            .add_submessage(on_chan_close_init))
    }

    /// This function validates a channel close confirmation message and creates a submessage to execute
    /// on a light client.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, API, and other modules that the contract may
    /// use.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being processed, such as the sender and the amount of funds sent with the message.
    /// * `message`: The `message` parameter is a reference to a `MsgChannelCloseConfirm` struct, which
    /// contains information about a channel close confirmation message being sent on a specific channel
    /// and port.
    ///
    /// Returns:
    ///
    /// A `Result<Response, ContractError>` is being returned.
    fn validate_channel_close_confirm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelCloseConfirm,
    ) -> Result<Response, ContractError> {
        let chan_end_on_b = self.get_channel_end(
            deps.storage,
            message.port_id_on_b.clone(),
            message.chan_id_on_b.clone(),
        )?;
        channel_close_confirm_validate(message, &chan_end_on_b)?;
        let conn_end_on_b =
            self.connection_end(deps.storage, chan_end_on_b.connection_hops()[0].clone())?;
        if !conn_end_on_b.state_matches(&ConnectionState::Open) {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::ConnectionNotOpen {
                    connection_id: chan_end_on_b.connection_hops()[0].clone(),
                },
            });
        }

        let client_id_on_b = conn_end_on_b.client_id();
        let client_state_of_a_on_b = self.client_state(deps.storage, client_id_on_b)?;
        let consensus_state_of_a_on_b =
            self.consensus_state(deps.storage, client_id_on_b, &message.proof_height_on_a)?;
        let prefix_on_a = conn_end_on_b.counterparty().prefix();
        let port_id_on_a = &chan_end_on_b.counterparty().port_id;
        let chan_id_on_a =
            chan_end_on_b
                .counterparty()
                .channel_id()
                .ok_or(ContractError::IbcChannelError {
                    error: ChannelError::InvalidCounterpartyChannelId,
                })?;
        let conn_id_on_a =
            conn_end_on_b
                .counterparty()
                .connection_id()
                .ok_or(ContractError::IbcChannelError {
                    error: ChannelError::UndefinedConnectionCounterparty {
                        connection_id: chan_end_on_b.connection_hops()[0].clone(),
                    },
                })?;
        if client_state_of_a_on_b.is_frozen() {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::FrozenClient {
                    client_id: client_id_on_b.clone(),
                },
            });
        }
        let expected_chan_end_on_a = ChannelEnd::new(
            State::Closed,
            *chan_end_on_b.ordering(),
            Counterparty::new(
                message.port_id_on_b.clone(),
                Some(message.chan_id_on_b.clone()),
            ),
            vec![conn_id_on_a.clone()],
            chan_end_on_b.version().clone(),
        );
        let raw_expected_chan = RawChannel::try_from(expected_chan_end_on_a).unwrap();

        let chan_end_path_on_a = commitment::channel_path(port_id_on_a, chan_id_on_a);
        let vector = raw_expected_chan.encode_to_vec();
        // let fee = self.calculate_fee(GAS_FOR_SUBMESSAGE_LIGHTCLIENT);
        //
        // let funds = self.update_fee(info.funds.clone(), fee)?;
        let create_client_message = LightClientMessage::VerifyChannel {
            message_info: cw_common::types::MessageInfo {
                sender: info.sender,
                funds: vec![],
            },
            endpoint: CwEndPoint {
                port_id: message.port_id_on_b.clone().to_string(),
                channel_id: message.chan_id_on_b.clone().to_string(),
            },
            verify_channel_state: VerifyChannelState {
                proof_height: message.proof_height_on_a.to_string(),
                counterparty_prefix: prefix_on_a.clone().into_vec(),
                proof: message.proof_chan_end_on_a.clone().into(),
                root: consensus_state_of_a_on_b.clone().root().into_vec(),
                counterparty_chan_end_path: chan_end_path_on_a,
                expected_counterparty_channel_end: vector,
                client_id: conn_end_on_b.client_id().to_string(),
            },
        };
        let client_type = client_state_of_a_on_b.client_type();
        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type)?;
        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&create_client_message).unwrap(),
            funds: info.funds,
        });
        let sub_msg: SubMsg = SubMsg::reply_always(
            create_client_message,
            EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_LIGHT_CLIENT,
        )
        .with_gas_limit(GAS_FOR_SUBMESSAGE_LIGHTCLIENT);

        Ok(Response::new()
            .add_attribute("action", "light_client_channel_close_confirm_call")
            .add_submessage(sub_msg))
    }
}

impl<'a> ExecuteChannel for CwIbcCoreContext<'a> {
    /// This function executes the channel open initialization process for an IBC channel and change the state to INIT.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier. It is used to interact with the blockchain and
    /// other modules.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module. It is used to extract the data returned by the sub-message and
    /// update the state of a channel.
    ///
    /// Returns:
    ///
    /// This function returns a `Result<Response, ContractError>` where `Response` is a struct
    /// representing the response to a contract execution and `ContractError` is an enum representing
    /// the possible errors that can occur during contract execution.
    fn execute_channel_open_init(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary_response::<cosmwasm_std::IbcEndpoint>(&res).unwrap();
                    let port_id = IbcPortId::from_str(&data.port_id).unwrap();
                    let channel_id = IbcChannelId::from_str(&data.channel_id).unwrap();
                    let mut channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;

                    if channel_end.state != State::Uninitialized {
                        return Err(ChannelError::UnknownState { state: 5 }).map_err(|e| e.into());
                    }
                    channel_end.state = State::Init;
                    self.store_channel_end(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        channel_end.clone(),
                    )?;
                    let _sequence = self.increase_channel_sequence(deps.storage)?;
                    self.store_next_sequence_send(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    self.store_next_sequence_recv(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    self.store_next_sequence_ack(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;

                    self.store_channel_commitment(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        channel_end.clone(),
                    )?;
                    let channel_id_event = create_channel_id_generated_event(channel_id.clone());

                    let main_event = create_open_init_channel_event(
                        channel_id.as_str(),
                        port_id.as_str(),
                        channel_end.counterparty().port_id().as_str(),
                        channel_end.connection_hops()[0].as_str(),
                        channel_end.version().as_str(),
                    );
                    Ok(Response::new()
                        .add_event(channel_id_event)
                        .add_event(main_event))
                }
                None => Err(ChannelError::Other {
                    description: "Data from module is Missing".to_string(),
                })
                .map_err(|e| e.into()),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ChannelError::Other { description: error }).map_err(|e| e.into())
            }
        }
    }

    /// This function executes a channel open try operation in chain b and change the state ti INIT.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides mutable access to the contract's
    /// dependencies such as storage, querier, and API.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module. It is used to extract the data returned by the sub-message and
    /// update the state of the channel accordingly.
    ///
    /// Returns:
    ///
    /// This function returns a `Result<Response, ContractError>` where `Response` is a struct
    /// representing the response to a contract execution and `ContractError` is an enum representing
    /// the possible errors that can occur during contract execution.
    fn execute_channel_open_try(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        debug_println!("reached execute_channelopenTry");
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary_response::<cosmwasm_std::IbcEndpoint>(&res).unwrap();
                    let port_id = IbcPortId::from_str(&data.port_id).unwrap();
                    let channel_id = IbcChannelId::from_str(&data.channel_id).unwrap();
                    let mut channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;

                    if channel_end.state != State::Uninitialized {
                        return Err(ChannelError::UnknownState { state: 5 }).map_err(|e| e.into());
                    }
                    channel_end.state = State::TryOpen;
                    self.store_channel_end(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        channel_end.clone(),
                    )?;
                    let _sequence = self.increase_channel_sequence(deps.storage)?;
                    self.store_next_sequence_send(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    self.store_next_sequence_recv(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    self.store_next_sequence_ack(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    let channel_id_event = create_channel_id_generated_event(channel_id.clone());
                    let main_event = create_open_try_channel_event(
                        channel_id.as_str(),
                        port_id.as_str(),
                        channel_end.counterparty().port_id().as_str(),
                        channel_end
                            .counterparty()
                            .channel_id
                            .clone()
                            .unwrap()
                            .as_str(),
                        channel_end.connection_hops()[0].as_str(),
                        channel_end.version().as_str(),
                    );

                    self.store_channel_commitment(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        channel_end,
                    )?;
                    Ok(Response::new()
                        .add_event(channel_id_event)
                        .add_event(main_event))
                }
                None => Err(ChannelError::Other {
                    description: "Data from module is Missing".to_string(),
                })
                .map_err(|e| e.into()),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ChannelError::Other { description: error }).map_err(|e| e.into())
            }
        }
    }

    /// This function handles the closing of an IBC channel and updates its state in storage.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier. It is used to interact with the blockchain and
    /// other modules.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module. It is used to extract the data returned by the sub-message and
    /// update the state of a channel accordingly.
    ///
    /// Returns:
    ///
    /// This function returns a `Result<Response, ContractError>` where `Response` is a struct
    /// representing the response to a contract execution and `ContractError` is an enum representing
    /// the possible errors that can occur during contract execution.
    fn execute_channel_close_init(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary_response::<cosmwasm_std::IbcEndpoint>(&res).unwrap();
                    let port_id = IbcPortId::from_str(&data.port_id).unwrap();
                    let channel_id = IbcChannelId::from_str(&data.channel_id).unwrap();
                    let mut channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;

                    channel_end.set_state(State::Closed); // State change
                    self.store_channel_end(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        channel_end.clone(),
                    )?;

                    self.store_channel_commitment(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        channel_end,
                    )?;

                    let event =
                        create_close_init_channel_event(port_id.as_str(), channel_id.as_str());
                    Ok(Response::new().add_event(event))
                }
                None => Err(ChannelError::Other {
                    description: "Data from module is Missing".to_string(),
                })
                .map_err(|e| e.into()),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ChannelError::Other { description: error }).map_err(|e| e.into())
            }
        }
    }

    /// This function handles the execution of a channel open acknowledgement message in an IBC protocol.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier. It is used to interact with the blockchain and
    /// other contracts.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module. It is used to handle the response of the `channel open ack`
    /// sub-message sent to the IBC module.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can
    /// occur during contract execution.
    fn execute_channel_open_ack(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary_response::<cosmwasm_std::IbcEndpoint>(&res).unwrap();
                    let port_id = IbcPortId::from_str(&data.port_id).unwrap();
                    let channel_id = IbcChannelId::from_str(&data.channel_id).unwrap();
                    let mut channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;
                    if !channel_end.state_matches(&State::Init) {
                        return Err(ChannelError::InvalidChannelState {
                            channel_id,
                            state: channel_end.state,
                        })
                        .map_err(|e| e.into());
                    }
                    channel_end.set_state(State::Open); // State Change
                    self.store_channel_end(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        channel_end.clone(),
                    )?;
                    self.store_channel_commitment(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        channel_end.clone(),
                    )?;

                    let event = create_open_ack_channel_event(
                        port_id.as_str(),
                        channel_id.as_str(),
                        channel_end.counterparty().port_id().as_str(),
                        channel_end.counterparty().channel_id().unwrap().as_str(),
                        channel_end.connection_hops()[0].as_str(),
                    );
                    Ok(Response::new()
                        .add_event(event)
                        .add_attribute("method", "execute_channel_open_ack"))
                }
                None => Err(ChannelError::Other {
                    description: "Data from module is Missing".to_string(),
                })
                .map_err(|e| e.into()),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ChannelError::Other { description: error }).map_err(|e| e.into())
            }
        }
    }

    /// This function executes the confirmation of a channel opening and updates the
    /// channel state accordingly.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier. It is used to interact with the blockchain and
    /// other modules.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module. It is used to confirm the opening of an IBC channel.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can
    /// occur during contract execution.
    fn execute_channel_open_confirm(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary_response::<cosmwasm_std::IbcEndpoint>(&res).unwrap();
                    let port_id = IbcPortId::from_str(&data.port_id).unwrap();
                    let channel_id = IbcChannelId::from_str(&data.channel_id).unwrap();
                    let mut channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;
                    if !channel_end.state_matches(&State::TryOpen) {
                        return Err(ChannelError::InvalidChannelState {
                            channel_id,
                            state: channel_end.state,
                        })
                        .map_err(|e| e.into());
                    }
                    channel_end.set_state(State::Open); // State Change
                    self.store_channel_end(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        channel_end.clone(),
                    )?;
                    self.store_channel_commitment(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        channel_end.clone(),
                    )?;

                    let event = create_open_confirm_channel_event(
                        port_id.as_str(),
                        channel_id.as_str(),
                        channel_end.counterparty().port_id().as_str(),
                        channel_end.counterparty().channel_id().unwrap().as_str(),
                        channel_end.connection_hops()[0].as_str(),
                    );
                    Ok(Response::new().add_event(event))
                }
                None => Err(ChannelError::Other {
                    description: "Data from module is Missing".to_string(),
                })
                .map_err(|e| e.into()),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ChannelError::Other { description: error }).map_err(|e| e.into())
            }
        }
    }
    /// This function handles the confirmation of closing an IBC channel and updates the channel state
    /// accordingly.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, API, and other modules that the contract may
    /// use.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module. It is used to confirm the closure of an IBC channel.
    ///
    /// Returns:
    ///
    /// This function returns a `Result<Response, ContractError>` where `Response` is a struct
    /// representing the response to a contract execution and `ContractError` is an enum representing
    /// the possible errors that can occur during contract execution.
    fn execute_channel_close_confirm(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary_response::<cosmwasm_std::IbcEndpoint>(&res).unwrap();
                    let port_id = IbcPortId::from_str(&data.port_id).unwrap();
                    let channel_id = IbcChannelId::from_str(&data.channel_id).unwrap();
                    let mut channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;
                    if channel_end.state_matches(&State::Closed) {
                        return Err(ChannelError::InvalidChannelState {
                            channel_id,
                            state: channel_end.state,
                        })
                        .map_err(|e| e.into());
                    }
                    channel_end.set_state(State::Closed); // State Change
                    self.store_channel_end(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        channel_end.clone(),
                    )?;
                    self.store_channel_commitment(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        channel_end.clone(),
                    )?;
                    let event =
                        create_close_confirm_channel_event(port_id.as_str(), channel_id.as_str());

                    Ok(Response::new().add_event(event))
                }
                None => Err(ChannelError::Other {
                    description: "Data from module is Missing".to_string(),
                })
                .map_err(|e| e.into()),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ChannelError::Other { description: error }).map_err(|e| e.into())
            }
        }
    }
}
