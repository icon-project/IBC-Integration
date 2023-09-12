use crate::{
    conversions::{to_ibc_channel, to_ibc_channel_id, to_ibc_height, to_ibc_port_id},
    validations::ensure_connection_state,
};

use super::*;
pub mod open_init;
pub mod open_try;
use self::{
    open_init::{
        channel_open_init_msg_validate, create_channel_submesssage, on_chan_open_init_submessage,
    },
    open_try::channel_open_try_msg_validate,
};

use common::ibc::core::ics04_channel::Version;
use cw_common::{
    commitment,
    raw_types::channel::{
        RawChannel, RawMsgChannelCloseConfirm, RawMsgChannelCloseInit, RawMsgChannelOpenAck,
        RawMsgChannelOpenConfirm, RawMsgChannelOpenInit, RawMsgChannelOpenTry,
    },
};
pub mod close_init;
use close_init::*;
pub mod open_ack;

use handler::open_try::on_chan_open_try_submessage;
use open_ack::*;
pub mod open_confirm;
use open_confirm::*;

pub mod close_confirm;
pub use close_confirm::*;

use prost::Message;
pub mod validate_channel;
use cw_common::cw_println;
use validate_channel::*;

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
        message: &RawMsgChannelOpenInit,
    ) -> Result<cosmwasm_std::Response, ContractError> {
        // connection hops should be 1
        cw_println!(
            deps,
            "inside validate channel open init: input parameter: {:?}",
            message
        );
        let mut channel_end = to_ibc_channel(message.channel.clone())?;
        let src_port = to_ibc_port_id(&message.port_id)?;

        if channel_end.connection_hops.len() != 1 {
            return Err(ChannelError::InvalidConnectionHopsLength {
                expected: 1,
                actual: channel_end.connection_hops.len(),
            })
            .map_err(Into::<ContractError>::into)?;
        }
        let connection_id = channel_end.connection_hops[0].clone();
        // An IBC connection running on the local (host) chain should exist.
        let connection_end = self.connection_end(deps.storage, &connection_id)?;
        let client_id = connection_end.client_id();
        let client_state = self.client_state(deps.as_ref(), client_id)?;

        if client_state.is_frozen() {
            return Err(ClientError::ClientFrozen {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }
        channel_open_init_msg_validate(&channel_end, connection_end)?;
        let counter = self.channel_counter(deps.storage)?;
        let src_channel = ChannelId::new(counter);
        let contract_address =
            self.lookup_modules(deps.storage, message.port_id.clone().as_bytes().to_vec())?;

        cw_println!(deps, "contract address is : {:?} ", contract_address);

        channel_end.state = State::Init;
        self.store_channel_end(deps.storage, &src_port, &src_channel, &channel_end)?;

        let sub_message =
            on_chan_open_init_submessage(&channel_end, &src_port, &src_channel, &connection_id);

        let _sequence = self.increase_channel_sequence(deps.storage)?;
        self.store_next_sequence_send(deps.storage, &src_port, &src_channel, &Sequence::from(1))?;
        self.store_next_sequence_recv(deps.storage, &src_port, &src_channel, &Sequence::from(1))?;
        self.store_next_sequence_ack(deps.storage, &src_port, &src_channel, &Sequence::from(1))?;

        self.store_channel_commitment(deps.storage, &src_port, &src_channel, &channel_end)?;
        let channel_id_event = create_channel_id_generated_event(src_channel.clone());
        let init_event = create_channel_event(
            IbcEventType::OpenInitChannel,
            src_port.as_str(),
            src_channel.as_str(),
            &channel_end,
        )?;
        let data = cw_common::ibc_dapp_msg::ExecuteMsg::IbcChannelOpen { msg: sub_message };
        let data = to_binary(&data).map_err(ContractError::Std)?;
        let on_chan_open_init = create_channel_submesssage(
            contract_address,
            data,
            info.funds,
            EXECUTE_ON_CHANNEL_OPEN_INIT,
        );

        Ok(Response::new()
            .add_attribute("action", "channel")
            .add_attribute("method", "channel_open_init_validation")
            .add_submessage(on_chan_open_init)
            .add_event(init_event)
            .add_event(channel_id_event))
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
        message: &RawMsgChannelOpenTry,
    ) -> Result<Response, ContractError> {
        let mut channel_end = to_ibc_channel(message.channel.clone())?;
        if channel_end.connection_hops.len() != 1 {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::InvalidConnectionHopsLength {
                    expected: 1,
                    actual: channel_end.connection_hops.len(),
                },
            });
        }
        cw_println!(deps, "Reached in channel open try");
        let connection_id = channel_end.connection_hops[0].clone();
        let connection_end = self.connection_end(deps.storage, &connection_id)?;

        channel_open_try_msg_validate(&channel_end, &connection_end)?;
        cw_println!(deps, "channel open try msg validate ");

        let counter = self.channel_counter(deps.storage)?;
        let dest_channel = ChannelId::new(counter); // creating new channel_id
        let dest_port = to_ibc_port_id(&message.port_id)?;

        let source_port = channel_end.remote.port_id.clone();
        let source_channel = channel_end.remote.channel_id.clone().unwrap();

        cw_println!(
            deps,
            "stoed: channel id: {:?}  portid :{:?} channel_end :{:?}",
            &dest_channel,
            &dest_port,
            &channel_end
        );

        let proof_height = to_ibc_height(message.proof_height.clone())?;
        let client_id = connection_end.client_id();
        let client_state = self.client_state(deps.as_ref(), client_id)?;
        let consensus_state = self.consensus_state(deps.as_ref(), client_id, &proof_height)?;
        let prefix_on_a = connection_end.counterparty().prefix();

        let conn_id_on_a = connection_end.counterparty().connection_id().ok_or(
            ContractError::IbcChannelError {
                error: ChannelError::UndefinedConnectionCounterparty {
                    connection_id: channel_end.connection_hops[0].clone(),
                },
            },
        )?;

        if client_state.is_frozen() {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::FrozenClient {
                    client_id: client_id.clone(),
                },
            });
        }

        cw_println!(deps, "after frozen check");
        let expected_channel_end = ChannelEnd::new(
            State::Init,
            *channel_end.ordering(),
            Counterparty::new(dest_port.clone(), None),
            vec![conn_id_on_a.clone()],
            channel_end.version().clone(),
        );
        let raw_expected_chan = RawChannel::try_from(expected_channel_end).unwrap();
        let chan_end_path_on_a = commitment::channel_path(&source_port, &source_channel);
        let vector = raw_expected_chan.encode_to_vec();

        let verify_channel_state = VerifyChannelState {
            proof_height: proof_height.to_string(),
            counterparty_prefix: prefix_on_a.clone().into_vec(),
            proof: message.proof_init.clone(),
            root: consensus_state.clone().root().into_vec(),
            counterparty_chan_end_path: chan_end_path_on_a,
            expected_counterparty_channel_end: vector,
            client_id: connection_end.client_id().to_string(),
        };
        let client_id = connection_end.client_id().clone();

        let client = self.get_light_client(deps.as_ref().storage, &client_id)?;
        client.verify_channel(deps.as_ref(), verify_channel_state)?;

        let contract_address = self.lookup_modules(deps.storage, dest_port.as_bytes().to_vec())?;
        cw_println!(deps, "contract addres is  {:?}", contract_address);

        channel_end.state = State::TryOpen;
        self.store_channel_end(deps.storage, &dest_port, &dest_channel, &channel_end)?;
        let _sequence = self.increase_channel_sequence(deps.storage)?;
        self.store_next_sequence_send(deps.storage, &dest_port, &dest_channel, &Sequence::from(1))?;
        self.store_next_sequence_recv(deps.storage, &dest_port, &dest_channel, &Sequence::from(1))?;
        self.store_next_sequence_ack(deps.storage, &dest_port, &dest_channel, &Sequence::from(1))?;
        let channel_id_event = create_channel_id_generated_event(dest_channel.clone());

        let main_event = create_channel_event(
            IbcEventType::OpenTryChannel,
            dest_port.as_str(),
            dest_channel.as_str(),
            &channel_end,
        )?;

        self.store_channel_commitment(deps.storage, &dest_port, &dest_channel, &channel_end)?;

        // Generate event for calling on channel open try in x-call
        let sub_message = on_chan_open_try_submessage(
            &channel_end,
            &dest_port,
            &dest_channel,
            &channel_end.connection_hops[0].clone(),
        );

        let data = cw_common::xcall_connection_msg::ExecuteMsg::IbcChannelOpen { msg: sub_message };

        let data = to_binary(&data).map_err(ContractError::Std)?;
        cw_println!(deps, "after converting data to binary ");

        let on_chan_open_try = create_channel_submesssage(
            contract_address,
            data,
            info.funds,
            EXECUTE_ON_CHANNEL_OPEN_TRY,
        );

        Ok(Response::new()
            .add_attribute("action", "channel")
            .add_attribute("method", "channel_open_init_module_validation")
            .add_submessage(on_chan_open_try)
            .add_event(main_event)
            .add_event(channel_id_event))
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
        message: &RawMsgChannelOpenAck,
    ) -> Result<Response, ContractError> {
        let src_port = to_ibc_port_id(&message.port_id)?;
        let src_channel = to_ibc_channel_id(&message.channel_id)?;

        let mut channel_end = self.get_channel_end(deps.storage, &src_port, &src_channel)?;
        ensure_channel_state(&src_channel, &channel_end, &State::Init)?;
        let dst_port = channel_end.counterparty().port_id.clone();
        let dst_channel = to_ibc_channel_id(&message.counterparty_channel_id)?;
        let connection_id = channel_end.connection_hops()[0].clone();

        channel_open_ack_validate(&src_channel, &channel_end)?;
        let connection_end = self.connection_end(deps.storage, &connection_id)?;
        ensure_connection_state(&connection_id, &connection_end, &ConnectionState::Open)?;
        let client_id = connection_end.client_id();

        let client_state = self.client_state(deps.as_ref(), client_id)?;

        let proof_height = to_ibc_height(message.proof_height.clone())?;

        let consensus_state = self.consensus_state(deps.as_ref(), client_id, &proof_height)?;

        let counterparty_prefix = connection_end.counterparty().prefix();
        let counterparty_connection_id = connection_end.counterparty().connection_id().ok_or(
            ContractError::IbcChannelError {
                error: ChannelError::UndefinedConnectionCounterparty {
                    connection_id: channel_end.connection_hops()[0].clone(),
                },
            },
        )?;
        if client_state.is_frozen() {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::FrozenClient {
                    client_id: client_id.clone(),
                },
            });
        }
        let version: Version = message.counterparty_version.clone().into();
        let expected_channel_end = ChannelEnd::new(
            State::TryOpen,
            *channel_end.ordering(),
            Counterparty::new(src_port.clone(), Some(src_channel.clone())),
            vec![counterparty_connection_id.clone()],
            version.clone(),
        );

        let raw_expected_chan = RawChannel::try_from(expected_channel_end).unwrap();
        let chan_end_path_on_b = commitment::channel_path(&dst_port, &dst_channel);
        let vector = raw_expected_chan.encode_to_vec();

        let verify_channel_state = VerifyChannelState {
            proof_height: proof_height.to_string(),
            counterparty_prefix: counterparty_prefix.clone().into_vec(),
            proof: message.proof_try.clone(),
            root: consensus_state.clone().root().into_vec(),
            counterparty_chan_end_path: chan_end_path_on_b,
            expected_counterparty_channel_end: vector,
            client_id: client_id.to_string(),
        };

        let client = self.get_light_client(deps.as_ref().storage, client_id)?;
        client.verify_channel(deps.as_ref(), verify_channel_state)?;

        channel_end.set_version(version);
        channel_end.set_counterparty_channel_id(dst_channel);
        channel_end.set_state(State::Open); // State Change
        self.store_channel_end(deps.storage, &src_port, &src_channel, &channel_end)?;
        self.store_channel_commitment(deps.storage, &src_port, &src_channel, &channel_end)?;

        let event = create_channel_event(
            IbcEventType::OpenAckChannel,
            src_port.as_str(),
            src_channel.as_str(),
            &channel_end,
        )?;

        let module_contract_address =
            self.lookup_modules(deps.storage, src_port.as_bytes().to_vec())?;

        let sub_message =
            on_chan_open_ack_submessage(&channel_end, &src_port, &src_channel, &connection_id)?;

        let data =
            cw_common::xcall_connection_msg::ExecuteMsg::IbcChannelConnect { msg: sub_message };
        let data = to_binary(&data).unwrap();
        let on_chan_open_try = create_channel_submesssage(
            module_contract_address,
            data,
            info.funds,
            EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
        );

        Ok(Response::new()
            .add_attribute("action", "channel")
            .add_attribute("method", "channel_open_init_module_validation")
            .add_submessage(on_chan_open_try)
            .add_event(event))
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
        message: &RawMsgChannelOpenConfirm,
    ) -> Result<Response, ContractError> {
        let dest_port = to_ibc_port_id(&message.port_id)?;
        let dest_channel = to_ibc_channel_id(&message.channel_id)?;

        let mut channel_end = self.get_channel_end(deps.storage, &dest_port, &dest_channel)?;
        ensure_channel_state(&dest_channel, &channel_end, &State::TryOpen)?;
        let src_port = &channel_end.counterparty().port_id;
        let src_channel =
            channel_end
                .counterparty()
                .channel_id()
                .ok_or(ContractError::IbcChannelError {
                    error: ChannelError::InvalidCounterpartyChannelId,
                })?;
        channel_open_confirm_validate(&dest_channel, &channel_end)?;

        let connection_id = channel_end.connection_hops()[0].clone();
        let connection_end = self.connection_end(deps.storage, &connection_id)?;

        ensure_connection_state(&connection_id, &connection_end, &ConnectionState::Open)?;

        let client_id = connection_end.client_id();
        let client_state = self.client_state(deps.as_ref(), client_id)?;
        let proof_height = to_ibc_height(message.proof_height.clone())?;
        let consensus_state = self.consensus_state(deps.as_ref(), client_id, &proof_height)?;
        let counterparty_prefix = connection_end.counterparty().prefix();

        let counterparty_connection_id = connection_end.counterparty().connection_id().ok_or(
            ContractError::IbcChannelError {
                error: ChannelError::UndefinedConnectionCounterparty {
                    connection_id: channel_end.connection_hops()[0].clone(),
                },
            },
        )?;
        if client_state.is_frozen() {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::FrozenClient {
                    client_id: client_id.clone(),
                },
            });
        }
        let expected_channel_end = ChannelEnd::new(
            State::Open,
            *channel_end.ordering(),
            Counterparty::new(dest_port.clone(), Some(dest_channel.clone())),
            vec![counterparty_connection_id.clone()],
            channel_end.version.clone(),
        );

        let raw_expected_chan = RawChannel::try_from(expected_channel_end).unwrap();
        let counterparty_channel_path = commitment::channel_path(src_port, src_channel);

        let expected_counterparty_channel_end = raw_expected_chan.encode_to_vec();

        let verify_channel_state = VerifyChannelState {
            proof_height: proof_height.to_string(),
            counterparty_prefix: counterparty_prefix.clone().into_vec(),
            proof: message.proof_ack.clone(),
            root: consensus_state.root().into_vec(),
            counterparty_chan_end_path: counterparty_channel_path,
            expected_counterparty_channel_end,
            client_id: client_id.to_string(),
        };

        let client_id = connection_end.client_id().clone();

        let client = self.get_light_client(deps.as_ref().storage, &client_id)?;
        client.verify_channel(deps.as_ref(), verify_channel_state)?;

        channel_end.set_state(State::Open);
        self.store_channel_end(deps.storage, &dest_port, &dest_channel, &channel_end)?;
        self.store_channel_commitment(deps.storage, &dest_port, &dest_channel, &channel_end)?;
        let event = create_channel_event(
            IbcEventType::OpenConfirmChannel,
            dest_port.as_str(),
            dest_channel.as_str(),
            &channel_end,
        )?;

        let contract_address = self.lookup_modules(deps.storage, dest_port.as_bytes().to_vec())?;

        let sub_message = on_chan_open_confirm_submessage(&channel_end, &dest_port, &dest_channel)?;

        let data =
            cw_common::xcall_connection_msg::ExecuteMsg::IbcChannelConnect { msg: sub_message };
        let data = to_binary(&data).unwrap();
        let on_chan_open_confirm = create_channel_submesssage(
            contract_address,
            data,
            info.funds,
            EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE,
        );

        Ok(Response::new()
            .add_attribute("action", "channel")
            .add_attribute("method", "channel_open_confirm_module_validation")
            .add_submessage(on_chan_open_confirm)
            .add_event(event))
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
        message: &RawMsgChannelCloseInit,
    ) -> Result<Response, ContractError> {
        let src_port = to_ibc_port_id(&message.port_id)?;
        let src_channel = to_ibc_channel_id(&message.channel_id)?;
        let mut channel_end = self.get_channel_end(deps.storage, &src_port, &src_channel)?;

        channel_close_init_validate(&src_channel, &channel_end)?;
        let connection_id = channel_end.connection_hops()[0].clone();
        let connection_end = self.connection_end(deps.storage, &connection_id)?;
        let client_id = connection_end.client_id();
        let client_state = self.client_state(deps.as_ref(), client_id)?;

        if client_state.is_frozen() {
            return Err(ClientError::ClientFrozen {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }

        ensure_connection_state(&connection_id, &connection_end, &ConnectionState::Open)?;

        channel_end.set_state(State::Closed); // State change
        self.store_channel_end(deps.storage, &src_port, &src_channel, &channel_end)?;

        self.store_channel_commitment(deps.storage, &src_port, &src_channel, &channel_end)?;

        let event = create_channel_event(
            IbcEventType::CloseInitChannel,
            src_port.as_str(),
            src_channel.as_str(),
            &channel_end,
        )?;
        let contract_address = self.lookup_modules(deps.storage, src_port.as_bytes().to_vec())?;

        let sub_message =
            on_chan_close_init_submessage(&src_port, &src_channel, &channel_end, &connection_id);

        let data =
            cw_common::xcall_connection_msg::ExecuteMsg::IbcChannelClose { msg: sub_message };
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
            .add_submessage(on_chan_close_init)
            .add_event(event))
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
        message: &RawMsgChannelCloseConfirm,
    ) -> Result<Response, ContractError> {
        let dest_port = to_ibc_port_id(&message.port_id)?;
        let dest_channel = to_ibc_channel_id(&message.channel_id)?;
        let mut channel_end = self.get_channel_end(deps.storage, &dest_port, &dest_channel)?;
        let counterparty = channel_end.counterparty().clone();
        let src_port = &counterparty.port_id;
        let src_channel = counterparty
            .channel_id()
            .ok_or(ContractError::IbcChannelError {
                error: ChannelError::InvalidCounterpartyChannelId,
            })?;
        channel_close_confirm_validate(&dest_channel, &channel_end)?;
        let connection_id = channel_end.connection_hops()[0].clone();
        let connection_end = self.connection_end(deps.storage, &connection_id)?;
        ensure_connection_state(&connection_id, &connection_end, &ConnectionState::Open)?;

        channel_end.set_state(State::Closed); // State Change
        self.store_channel_end(deps.storage, &dest_port, &dest_channel, &channel_end)?;
        self.store_channel_commitment(deps.storage, &dest_port, &dest_channel, &channel_end)?;
        let event = create_channel_event(
            IbcEventType::CloseConfirmChannel,
            dest_port.as_str(),
            dest_channel.as_str(),
            &channel_end,
        )?;

        let client_id = connection_end.client_id();
        let client_state = self.client_state(deps.as_ref(), client_id)?;
        let proof_height = to_ibc_height(message.proof_height.clone())?;
        let consensus_state = self.consensus_state(deps.as_ref(), client_id, &proof_height)?;
        let prefix_on_a = connection_end.counterparty().prefix();

        let conn_id_on_a = connection_end.counterparty().connection_id().ok_or(
            ContractError::IbcChannelError {
                error: ChannelError::UndefinedConnectionCounterparty {
                    connection_id: channel_end.connection_hops()[0].clone(),
                },
            },
        )?;
        if client_state.is_frozen() {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::FrozenClient {
                    client_id: client_id.clone(),
                },
            });
        }
        let expected_channel_end = ChannelEnd::new(
            State::Closed,
            *channel_end.ordering(),
            Counterparty::new(dest_port.clone(), Some(dest_channel.clone())),
            vec![conn_id_on_a.clone()],
            channel_end.version().clone(),
        );
        let raw_expected_chan = RawChannel::try_from(expected_channel_end).unwrap();

        let counterparty_chan_end_path = commitment::channel_path(src_port, src_channel);
        let expected_counterparty_channel_end = raw_expected_chan.encode_to_vec();

        let verify_channel_state = VerifyChannelState {
            proof_height: proof_height.to_string(),
            counterparty_prefix: prefix_on_a.clone().into_vec(),
            proof: message.proof_init.clone(),
            root: consensus_state.root().into_vec(),
            counterparty_chan_end_path,
            expected_counterparty_channel_end,
            client_id: connection_end.client_id().to_string(),
        };

        let client_id = connection_end.client_id().clone();
        let client = self.get_light_client(deps.as_ref().storage, &client_id)?;
        client.verify_channel(deps.as_ref(), verify_channel_state)?;

        // Getting the module address for on channel open try call
        let contract_address = self.lookup_modules(deps.storage, dest_port.as_bytes().to_vec())?;

        // Generate event for calling on channel open try in x-call
        let sub_message =
            on_chan_close_confirm_submessage(&channel_end, &dest_port, &dest_channel)?;

        let data =
            cw_common::xcall_connection_msg::ExecuteMsg::IbcChannelClose { msg: sub_message };
        let data = to_binary(&data).map_err(Into::<ContractError>::into)?;
        let on_chan_close_confirm = create_channel_submesssage(
            contract_address,
            data,
            info.funds,
            EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE,
        );

        Ok(Response::new()
            .add_attribute("action", "channel")
            .add_attribute("method", "channel_close_confirm_module_validation")
            .add_submessage(on_chan_close_confirm)
            .add_event(event))
    }
}
