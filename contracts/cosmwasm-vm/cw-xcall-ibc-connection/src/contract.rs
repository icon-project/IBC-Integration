use common::{
    ibc::Height,
    rlp::{self},
};
use cosmwasm_std::{coins, BankMsg, IbcChannel};
use cw_common::{raw_types::channel::RawPacket, xcall_types::network_address::NetId};
use debug_print::debug_println;

use crate::{
    state::{
        HOST_SEND_MESSAGE_REPLY_ID, HOST_WRITE_ACKNOWLEDGEMENT_REPLY_ID,
        XCALL_HANDLE_ERROR_REPLY_ID, XCALL_HANDLE_MESSAGE_REPLY_ID,
    },
    types::{
        channel_config::ChannelConfig, config::Config, connection_config::ConnectionConfig,
        message::Message, LOG_PREFIX,
    },
};

use super::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-xcall-ibc-connection";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwIbcConnection<'a> {
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
                    CwIbcConnection::validate_address(deps.api, address.as_str())?;
                self.add_admin(deps.storage, info, validated_address.to_string())
            }
            ExecuteMsg::SendMessage { to, sn, msg } => {
                println!("{LOG_PREFIX} Received Payload From XCall App");
                // return Ok(Response::new());
                self.send_message(deps, info, env, to, sn, msg)
            }
            ExecuteMsg::SetXCallHost { address } => {
                self.ensure_owner(deps.as_ref().storage, &info)?;
                let validated_address =
                    CwIbcConnection::validate_address(deps.api, address.as_str())?;
                self.set_xcall_host(deps.storage, validated_address)?;
                Ok(Response::new())
            }
            ExecuteMsg::ConfigureConnection {
                connection_id,
                counterparty_port_id,
                counterparty_nid,
                client_id,
                timeout_height,
            } => {
                self.ensure_owner(deps.as_ref().storage, &info)?;
                self.configure_connection(
                    deps.storage,
                    connection_id,
                    counterparty_port_id,
                    counterparty_nid,
                    client_id,
                    timeout_height,
                )?;
                Ok(Response::new())
            }
            ExecuteMsg::ClaimFees { nid, address } => {
                let fee_msg = self.claim_fees(deps, info, nid, address)?;
                Ok(Response::new().add_submessage(fee_msg))
            }
            ExecuteMsg::SetFees {
                nid,
                packet_fee,
                ack_fee,
            } => self.set_fee(deps.storage, nid, packet_fee, ack_fee),
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcChannelOpen { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_channel_open(deps.storage, msg)?)
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcChannelConnect { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_channel_connect(deps.storage, msg)?)
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcChannelClose { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_channel_close(msg)?)
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcPacketReceive { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_packet_receive(deps, msg)?)
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcPacketAck { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_packet_ack(deps, msg)?)
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcPacketTimeout { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_packet_timeout(deps, msg)?)
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
            QueryMsg::GetTimeoutHeight { channel_id } => {
                let config = self.get_channel_config(deps.storage, &channel_id).unwrap();
                to_binary(&config.timeout_height)
            }
            QueryMsg::GetFee { nid, response } => {
                let fees = self.get_network_fees(deps.storage, nid);
                if response {
                    return to_binary(&(fees.send_packet_fee + fees.ack_fee));
                }
                to_binary(&(fees.send_packet_fee))
            }
            QueryMsg::GetUnclaimedFee { nid, relayer } => {
                to_binary(&self.get_unclaimed_fee(deps.storage, nid, relayer))
            }
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

    pub fn reply(&self, deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.id {
            ACK_FAILURE_ID => self.reply_ack_on_error(msg),
            XCALL_HANDLE_MESSAGE_REPLY_ID => self.xcall_handle_message_reply(deps, msg),
            XCALL_HANDLE_ERROR_REPLY_ID => self.xcall_handle_error_reply(deps, msg),
            HOST_WRITE_ACKNOWLEDGEMENT_REPLY_ID => self.host_write_acknowledgement_reply(deps, msg),
            HOST_SEND_MESSAGE_REPLY_ID => self.host_send_message_reply(deps, msg),
            _ => Err(ContractError::ReplyError {
                code: msg.id,
                msg: "Unknown".to_string(),
            }),
        }
    }
}

impl<'a> CwIbcConnection<'a> {
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
        let owner = info.sender.as_str().to_string();

        self.add_owner(store, owner.clone())?;
        self.add_admin(store, info, owner)?;
        // self.set_timeout_height(store, msg.timeout_height)?;
        self.set_ibc_host(store, msg.ibc_host.clone())?;
        self.set_xcall_host(store, msg.xcall_address)?;
        let config = Config {
            port_id: msg.port_id,
            denom: msg.denom,
        };
        self.store_config(store, &config)?;

        Ok(Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("method", "init")
            .add_attribute("ibc_host", msg.ibc_host))
    }

    fn xcall_handle_message_reply(
        &self,
        _deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        println!("{LOG_PREFIX} Reply From Forward XCall");
        match message.result {
            SubMsgResult::Ok(_) => Ok(Response::new()
                .add_attribute("action", "call_message")
                .add_attribute("method", "xcall_handle_message_reply")),
            SubMsgResult::Err(error) => Err(ContractError::ReplyError {
                code: message.id,
                msg: error,
            }),
        }
    }

    fn xcall_handle_error_reply(
        &self,
        _deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        println!("{LOG_PREFIX} Reply From Forward XCall");
        match message.result {
            SubMsgResult::Ok(_) => {
                // self.remove_outgoing_packet_sn(store, channel_id, sequence)
                Ok(Response::new()
                    .add_attribute("action", "call_message")
                    .add_attribute("method", "xcall_handle_error_reply"))
            }
            SubMsgResult::Err(error) => Err(ContractError::ReplyError {
                code: message.id,
                msg: error,
            }),
        }
    }

    fn host_send_message_reply(
        &self,
        _deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        println!("{LOG_PREFIX} Reply From Forward Host");
        match message.result {
            SubMsgResult::Ok(_) => Ok(Response::new()
                .add_attribute("action", "call_message")
                .add_attribute("method", "reply_forward_host")),
            SubMsgResult::Err(error) => Err(ContractError::ReplyError {
                code: message.id,
                msg: error,
            }),
        }
    }

    fn host_write_acknowledgement_reply(
        &self,
        _deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        println!("{LOG_PREFIX} Reply From Write Acknowledgement Host");
        match message.result {
            SubMsgResult::Ok(_) => Ok(Response::new()
                .add_attribute("action", "call_message")
                .add_attribute("method", "reply_write_acknowledgement")),
            SubMsgResult::Err(error) => Err(ContractError::ReplyError {
                code: message.id,
                msg: error,
            }),
        }
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
    /// This function handles the opening of an IBC channel and returns a response with relevant
    /// attributes.
    ///
    /// Arguments:
    ///
    /// * `msg`: The `msg` parameter is of type `IbcChannelOpenMsg`, which is a message that represents
    /// the opening of an IBC channel. It contains information about the channel, such as the endpoint
    /// and the order of packet delivery.
    ///
    /// Returns:
    ///
    /// The function `on_channel_open` returns a `Result<Response, ContractError>` where `Response` is a
    /// struct that contains data and attributes that will be returned to the caller, and
    /// `ContractError` is an enum that represents any errors that may occur during the execution of the
    /// function.
    pub fn on_channel_open(
        &mut self,
        store: &mut dyn Storage,
        msg: CwChannelOpenMsg,
    ) -> Result<Response, ContractError> {
        debug_println!("[IbcConnection]: Called On channel open");
        println!("{msg:?}");

        let channel = msg.channel();
        let ibc_endpoint = channel.endpoint.clone();

        check_order(&channel.order)?;
        debug_println!("[IbcConnection]: check order pass");

        if let Some(counter_version) = msg.counterparty_version() {
            check_version(counter_version)?;
        }
        debug_println!("[IbcConnection]: check version pass");
        self.setup_channel(store, channel.clone())?;

        Ok(Response::new()
            .set_data(to_binary(&ibc_endpoint).unwrap())
            .add_attribute("method", "on_channel_open")
            .add_attribute("version", IBC_VERSION))
    }
    /// This is a Rust function that handles a channel connection message in an IBC protocol implementation,
    /// saving the configuration and returning a response.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and persist data.
    /// * `msg`: The `msg` parameter is of type `IbcChannelConnectMsg`, which is a message that represents a
    /// channel connection event in the Inter-Blockchain Communication (IBC) protocol. It contains
    /// information about the channel being connected, such as the channel ID, port ID, and endpoints.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct that contains data and attributes
    /// related to the IBC channel connection, and `ContractError` is an enum that represents any errors
    /// that may occur during the execution of the function.
    pub fn on_channel_connect(
        &mut self,
        store: &mut dyn Storage,
        msg: CwChannelConnectMsg,
    ) -> Result<Response, ContractError> {
        let channel = msg.channel();
        debug_println!("[IBCConnection]: channel connect called");

        check_order(&channel.order)?;
        debug_println!("[IBCConnection]: check order pass");

        if let Some(counter_version) = msg.counterparty_version() {
            check_version(counter_version)?;
        }

        debug_println!("[IBCConnection]: check version passed");
        self.setup_channel(store, channel.clone())?;

        Ok(Response::new()
            .set_data(to_binary(&channel.endpoint.clone()).unwrap())
            .add_attribute("method", "on_channel_connect"))
    }
    /// This function handles an IBC channel close message and returns a response with relevant attributes
    /// and data.
    ///
    /// Arguments:
    ///
    /// * `msg`: The `msg` parameter is of type `IbcChannelCloseMsg`, which is a message that represents
    /// the closing of an IBC channel. It can be either a `CloseInit` message or a `CloseConfirm` message,
    /// both of which contain information about the channel being closed.
    ///
    /// Returns:
    ///
    /// A `Result` containing a `Response` or a `ContractError`.
    pub fn on_channel_close(&self, msg: CwChannelCloseMsg) -> Result<Response, ContractError> {
        let ibc_endpoint = match msg {
            CwChannelCloseMsg::CloseInit { channel } => channel.endpoint,
            CwChannelCloseMsg::CloseConfirm { channel } => channel.endpoint,
        };

        Ok(Response::new()
            .add_attribute("method", "ibc_channel_close")
            .set_data(to_binary(&ibc_endpoint).unwrap()))
    }
    /// This function receives an IBC packet and returns a response with acknowledgement and events or an
    /// error message.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides access to the mutable dependencies of the
    /// contract. These dependencies include the storage, querier, and API interfaces.
    /// * `msg`: The `msg` parameter is an `IbcPacketReceiveMsg` struct that contains information about the
    /// received IBC packet, such as the source and destination chain IDs, the packet data, and the packet
    /// sequence number.
    ///
    /// Returns:
    ///
    /// A `Result<Response, ContractError>` is being returned.
    pub fn on_packet_receive(
        &self,
        deps: DepsMut,
        msg: CwPacketReceiveMsg,
    ) -> Result<Response, ContractError> {
        match self.do_packet_receive(deps, msg.packet, msg.relayer) {
            Ok(ibc_response) => Ok(Response::new()
                .add_attributes(ibc_response.attributes.clone())
                .add_submessages(ibc_response.messages)
                .add_events(ibc_response.events)),
            Err(error) => Err(error),
        }
    }

    /// The function handles the acknowledgement of a packet and returns a response with attributes
    /// indicating success or failure.
    ///
    /// Arguments:
    ///
    /// * `ack`: The `ack` parameter is an `IbcPacketAckMsg` struct, which represents the acknowledgement
    /// message for an Inter-Blockchain Communication (IBC) packet. It contains information about the
    /// acknowledgement, including the original packet data and the acknowledgement data.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to be
    /// returned to the caller and `ContractError` is an enum representing the possible errors that can
    /// occur during the execution of the function.
    pub fn on_packet_ack(
        &self,
        deps: DepsMut,
        ack: CwPacketAckMsg,
    ) -> Result<Response, ContractError> {
        let packet = ack.original_packet;
        let acknowledgement = ack.acknowledgement;
        let channel = packet.src.channel_id.clone();
        let seq = packet.sequence;
        let channel_config = self.get_channel_config(deps.as_ref().storage, &channel)?;
        let nid = channel_config.counterparty_nid;

        let sn = self.get_outgoing_packet_sn(deps.as_ref().storage, &channel, seq)?;
        self.remove_outgoing_packet_sn(deps.storage, &channel, seq);

        let submsg =
            self.call_xcall_handle_message(deps.storage, &nid, acknowledgement.data.0, Some(sn))?;

        let bank_msg = self.settle_unclaimed_ack_fee(
            deps.storage,
            nid.as_str(),
            seq,
            ack.relayer.to_string(),
        )?;

        Ok(Response::new()
            .add_messages(bank_msg)
            .add_submessage(submsg))
    }
    /// This function handles a timeout event for an IBC packet and sends a reply message with an error
    /// code.
    ///
    /// Arguments:
    ///
    /// * `_msg`: The `_msg` parameter is of type `IbcPacketTimeoutMsg`, which is a struct that contains
    /// information about a timed-out IBC packet. This information includes the packet sequence, the port
    /// and channel identifiers, and the height at which the packet was sent.
    ///
    /// Returns:
    ///
    /// a `Result` object that contains a `Response` object if the function executes successfully, or a
    /// `ContractError` object if an error occurs. The `Response` object contains a submessage and an
    /// attribute. The submessage is a reply on error with a `CosmosMsg::Custom` object that contains an
    /// `Empty` message. The attribute is a key-value pair

    pub fn on_packet_timeout(
        &self,
        deps: DepsMut,
        msg: CwPacketTimeoutMsg,
    ) -> Result<Response, ContractError> {
        let packet = msg.packet;
        let channel_id = packet.src.channel_id.clone();
        let channel_config = self.get_channel_config(deps.as_ref().storage, &channel_id)?;
        let nid = channel_config.counterparty_nid;
        let sn = self.get_outgoing_packet_sn(deps.storage, &channel_id, packet.sequence)?;

        let n_message: Message = rlp::decode(&packet.data).unwrap();
        self.remove_outgoing_packet_sn(deps.storage, &channel_id, packet.sequence);
        self.add_unclaimed_ack_fees(deps.storage, &nid, packet.sequence, n_message.fee)?;
        let submsg = self.call_xcall_handle_error(deps.storage, sn, -1, "Timeout".to_string())?;
        let bank_msg = self.settle_unclaimed_ack_fee(
            deps.storage,
            nid.as_str(),
            packet.sequence,
            msg.relayer.to_string(),
        )?;

        Ok(Response::new()
            .add_messages(bank_msg)
            .add_submessage(submsg))
    }

    pub fn settle_unclaimed_ack_fee(
        &self,
        store: &mut dyn Storage,
        nid: &str,
        seq: u64,
        relayer: String,
    ) -> Result<Vec<BankMsg>, ContractError> {
        let ack_fee = self.get_unclaimed_ack_fee(store, nid, seq);
        if ack_fee <= 0 {
            return Ok(vec![]);
        }
        self.reset_unclaimed_ack_fees(store, nid, seq)?;
        let denom = self.get_denom(store)?;

        let msg = BankMsg::Send {
            to_address: relayer,
            amount: coins(ack_fee, denom),
        };

        Ok(vec![msg])
    }

    pub fn setup_channel(
        &mut self,
        store: &mut dyn Storage,
        channel: IbcChannel,
    ) -> Result<(), ContractError> {
        let source = channel.endpoint.clone();
        let destination = channel.counterparty_endpoint.clone();
        let channel_id = source.channel_id.clone();

        let our_port = self.get_port(store)?;
        if our_port != source.port_id {
            return Err(ContractError::InvalidPortId);
        }

        let nid = self.get_counterparty_nid(store, &channel.connection_id, &destination.port_id)?;
        let connection_config = self.get_connection_config(store, &channel.connection_id)?;
        let ibc_config = IbcConfig::new(source, destination);
        debug_println!("[IBCConnection]: save ibc config is {:?}", ibc_config);

        self.store_ibc_config(store, &nid, &ibc_config)?;

        self.store_channel_config(
            store,
            &channel_id,
            &ChannelConfig {
                timeout_height: connection_config.timeout_height,
                client_id: connection_config.client_id,
                counterparty_nid: nid,
            },
        )?;

        Ok(())
    }

    pub fn configure_connection(
        &self,
        store: &mut dyn Storage,
        connection_id: String,
        counterparty_port_id: String,
        counterparty_nid: NetId,
        client_id: String,
        timeout_height: u64,
    ) -> Result<(), ContractError> {
        if self
            .get_counterparty_nid(store, &connection_id, &counterparty_port_id)
            .is_ok()
        {
            return Err(ContractError::ConnectionAlreadyConfigured {
                connection_id,
                port_id: counterparty_port_id,
            });
        }
        self.store_counterparty_nid(
            store,
            &connection_id,
            &counterparty_port_id,
            &counterparty_nid,
        )?;

        self.store_connection_config(
            store,
            &connection_id,
            &ConnectionConfig {
                timeout_height,
                client_id,
            },
        )?;

        Ok(())
    }

    pub fn create_packet<T: common::rlp::Encodable>(
        &self,
        ibc_config: IbcConfig,
        timeout_height: Height,
        sequence_no: u64,
        data: T,
    ) -> RawPacket {
        let packet = RawPacket {
            sequence: sequence_no,
            source_port: ibc_config.src_endpoint().clone().port_id,
            source_channel: ibc_config.src_endpoint().clone().channel_id,
            destination_port: ibc_config.dst_endpoint().clone().port_id,
            destination_channel: ibc_config.dst_endpoint().clone().channel_id,
            data: rlp::encode(&data).to_vec(),
            timeout_height: Some(timeout_height.into()),
            timeout_timestamp: 0,
        };
        packet
    }
}
