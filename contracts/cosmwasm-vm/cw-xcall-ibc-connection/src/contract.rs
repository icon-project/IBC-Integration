use cosmwasm_std::{from_slice, to_vec};
use debug_print::debug_println;

use crate::{
    state::{HOST_FORWARD_REPLY_ID, XCALL_FORWARD_REPLY_ID},
    types::LOG_PREFIX,
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
            ExecuteMsg::MessageFromXCall { data } => {
                println!("{LOG_PREFIX} Received Payload From XCall App");
                self.forward_to_host(deps, info, env, data)
            }
            ExecuteMsg::SetXCallHost { address } => {
                self.ensure_owner(deps.as_ref().storage, &info)?;
                let validated_address =
                    CwIbcConnection::validate_address(deps.api, address.as_str())?;
                self.set_xcall_host(deps.storage, validated_address)?;
                Ok(Response::new())
            }
            ExecuteMsg::UpdateAdmin { address } => {
                let validated_address =
                    CwIbcConnection::validate_address(deps.api, address.as_str())?;
                self.update_admin(deps.storage, info, validated_address.to_string())
            }
            ExecuteMsg::RemoveAdmin {} => self.remove_admin(deps.storage, info),
            ExecuteMsg::SetIbcConfig { ibc_config } => {
                self.ensure_owner(deps.as_ref().storage, &info)?;
                let config = from_slice(&ibc_config).unwrap();
                self.save_config(deps.storage, &config)?;
                Ok(Response::new())
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcChannelOpen { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_channel_open(msg)?)
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
                Ok(self.on_packet_ack(msg)?)
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcPacketTimeout { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_packet_timeout(msg)?)
            }
            ExecuteMsg::SetTimeoutHeight { height } => {
                self.ensure_admin(deps.as_ref().storage, info.sender)?;

                self.set_timeout_height(deps.storage, height)?;

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
            QueryMsg::GetTimeoutHeight {} => to_binary(&self.get_timeout_height(deps.storage)),
            QueryMsg::GetProtocolFee {} => to_binary(&self.get_protocol_fee(deps).unwrap()),
            QueryMsg::GetProtocolFeeHandler {} => to_binary(&self.get_protocol_feehandler(deps)),
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
            XCALL_FORWARD_REPLY_ID => self.reply_forward_xcall(deps, msg),
            HOST_FORWARD_REPLY_ID => self.reply_forward_host(deps, msg),
            ACK_FAILURE_ID => self.reply_ack_on_error(msg),
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
        self.set_timeout_height(store, msg.timeout_height)?;
        self.set_ibc_host(store, msg.ibc_host.clone())?;
        self.add_fee(store, msg.protocol_fee)?;

        Ok(Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("method", "init")
            .add_attribute("ibc_host", msg.ibc_host))
    }

    fn reply_forward_xcall(
        &self,
        _deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        println!("{LOG_PREFIX} Reply From Forward XCall");
        match message.result {
            SubMsgResult::Ok(_) => Ok(Response::new()
                .add_attribute("action", "call_message")
                .add_attribute("method", "reply_forward_xcall")),
            SubMsgResult::Err(error) => Err(ContractError::ReplyError {
                code: message.id,
                msg: error,
            }),
        }
    }

    fn reply_forward_host(
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
    fn on_channel_open(&self, msg: CwChannelOpenMsg) -> Result<Response, ContractError> {
        debug_println!("[IbcConnection]: Called On channel open");
        println!("{msg:?}");
        let ibc_endpoint = match msg.clone() {
            CwChannelOpenMsg::OpenInit { channel } => channel.endpoint,
            CwChannelOpenMsg::OpenTry {
                channel,
                counterparty_version: _,
            } => channel.endpoint,
        };
        let channel = msg.channel();

        check_order(&channel.order)?;
        debug_println!("[IbcConnection]: check order pass");

        if let Some(counter_version) = msg.counterparty_version() {
            check_version(counter_version)?;
        }
        debug_println!("[IbcConnection]: check version pass");

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
    fn on_channel_connect(
        &self,
        store: &mut dyn Storage,
        msg: CwChannelConnectMsg,
    ) -> Result<Response, ContractError> {
        let ibc_endpoint = match msg.clone() {
            CwChannelConnectMsg::OpenAck {
                channel,
                counterparty_version: _,
            } => channel.endpoint,
            CwChannelConnectMsg::OpenConfirm { channel } => channel.endpoint,
        };
        let channel = msg.channel();

        check_order(&channel.order)?;

        if let Some(counter_version) = msg.counterparty_version() {
            check_version(counter_version)?;
        }

        let source = msg.channel().endpoint.clone();
        let destination = msg.channel().counterparty_endpoint.clone();

        let ibc_config = IbcConfig::new(source, destination);
        let mut call_service = CwIbcConnection::default();
        call_service.save_config(store, &ibc_config)?;

        Ok(Response::new()
            .set_data(to_binary(&ibc_endpoint).unwrap())
            .add_attribute("method", "on_channel_connect")
            .add_attribute(
                "source_channel_id",
                msg.channel().endpoint.channel_id.as_str(),
            )
            .add_attribute("source_port_id", msg.channel().endpoint.port_id.as_str())
            .add_attribute(
                "destination_channel_id",
                msg.channel().counterparty_endpoint.channel_id.as_str(),
            )
            .add_attribute(
                "destination_port_id",
                msg.channel().counterparty_endpoint.port_id.as_str(),
            ))
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
    fn on_channel_close(&self, msg: CwChannelCloseMsg) -> Result<Response, ContractError> {
        let ibc_endpoint = match msg.clone() {
            CwChannelCloseMsg::CloseInit { channel } => channel.endpoint,
            CwChannelCloseMsg::CloseConfirm { channel } => channel.endpoint,
        };
        let channel = msg.channel().endpoint.channel_id.clone();

        Ok(Response::new()
            .add_attribute("method", "ibc_channel_close")
            .add_attribute("channel", channel)
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
    fn on_packet_receive(
        &self,
        deps: DepsMut,
        msg: CwPacketReceiveMsg,
    ) -> Result<Response, ContractError> {
        match self.receive_packet_data(deps, msg.packet) {
            Ok(ibc_response) => Ok(Response::new()
                .add_attributes(ibc_response.attributes.clone())
                .set_data(to_vec(&ibc_response.acknowledgement).unwrap())
                .add_events(ibc_response.events)),
            Err(error) => Ok(Response::new()
                .add_attribute("method", "ibc_packet_receive")
                .add_attribute("error", error.to_string())
                .set_data(make_ack_fail(error.to_string()))),
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
    fn on_packet_ack(&self, ack: CwPacketAckMsg) -> Result<Response, ContractError> {
        let ack_response: Ack = from_binary(&ack.acknowledgement.data)?;
        // let message: CallServiceMessage = from_binary(&ack.original_packet.data)?;
        // let message_type = match message.message_type() {
        //     CallServiceMessageType::CallServiceRequest => "call_service_request",
        //     CallServiceMessageType::CallServiceResponse => "call_service_response",
        // };

        match ack_response {
            Ack::Result(_) => {
                let attributes = vec![attr("action", "acknowledge"), attr("success", "true")];

                Ok(Response::new().add_attributes(attributes))
            }
            Ack::Error(err) => Ok(Response::new()
                .add_attribute("action", "acknowledge")
                .add_attribute("success", "false")
                .add_attribute("error", err)),
        }
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

    fn on_packet_timeout(&self, _msg: CwPacketTimeoutMsg) -> Result<Response, ContractError> {
        let submsg = SubMsg::reply_on_error(CosmosMsg::Custom(Empty {}), ACK_FAILURE_ID);
        Ok(Response::new()
            .add_submessage(submsg)
            .add_attribute("method", "ibc_packet_timeout"))
    }
}
