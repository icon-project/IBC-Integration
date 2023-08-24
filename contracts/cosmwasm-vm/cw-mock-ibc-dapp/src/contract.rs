use common::{
    ibc::Height,
    rlp::{self},
};
use cosmwasm_std::IbcChannel;
use cw_common::raw_types::channel::RawPacket;
use debug_print::debug_println;

use crate::types::{message::Message, LOG_PREFIX};

use super::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-mock-ibc-dapp";
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
            ExecuteMsg::SendMessage {
                msg,
                timeout_height,
            } => {
                println!("{LOG_PREFIX} Received Payload From XCall App");
                // return Ok(Response::new());
                self.send_message(deps, info, env, msg, timeout_height)
            }

            ExecuteMsg::IbcChannelOpen { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_channel_open(deps.storage, msg)?)
            }

            ExecuteMsg::IbcChannelConnect { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_channel_connect(deps.storage, msg)?)
            }

            ExecuteMsg::IbcChannelClose { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_channel_close(msg)?)
            }

            ExecuteMsg::IbcPacketReceive { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_packet_receive(deps, msg)?)
            }

            ExecuteMsg::IbcPacketAck { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_packet_ack(deps, msg)?)
            }

            ExecuteMsg::IbcPacketTimeout { msg } => {
                self.ensure_ibc_handler(deps.as_ref().storage, info.sender)?;
                Ok(self.on_packet_timeout(deps, msg)?)
            }
            ExecuteMsg::IbcWriteAcknowledgement { seq } => {
                let packet = self.get_received_packet(deps.as_ref().storage, seq)?;
                self.write_acknowledgement(deps.storage, packet)
            }
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

    pub fn reply(&self, _deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
        Err(ContractError::ReplyError {
            code: msg.id,
            msg: "Unknown".to_string(),
        })
    }

    pub fn migrate(
        &self,
        deps: DepsMut,
        _env: Env,
        _msg: MigrateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(Response::default().add_attribute("migrate", "successful"))
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

        Ok(Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("method", "init")
            .add_attribute("ibc_host", msg.ibc_host))
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
        debug_println!("this is inside on channel open of mock ibc dapp ");
        debug_println!("[IbcConnection]: Called On channel open");
        println!("{msg:?}");

        let channel = msg.channel();
        let ibc_endpoint = channel.endpoint.clone();

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
                .add_events(ibc_response.events)
                .set_data(Binary::from(vec![1]))),
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
        _deps: DepsMut,
        ack: CwPacketAckMsg,
    ) -> Result<Response, ContractError> {
        let packet = ack.original_packet;
        let _acknowledgement = ack.acknowledgement;
        let _channel = packet.src.channel_id.clone();
        let _seq = packet.sequence;

        Ok(Response::new())
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
        _deps: DepsMut,
        msg: CwPacketTimeoutMsg,
    ) -> Result<Response, ContractError> {
        let packet = msg.packet;

        let n_message: Message = rlp::decode(&packet.data).unwrap();

        if n_message.sn.is_none() {
            return Ok(Response::new());
        }

        Ok(Response::new())
    }

    pub fn setup_channel(
        &mut self,
        store: &mut dyn Storage,
        channel: IbcChannel,
    ) -> Result<(), ContractError> {
        let source = channel.endpoint.clone();
        let destination = channel.counterparty_endpoint;
        let _channel_id = source.channel_id.clone();

        let ibc_config = IbcConfig::new(source, destination);
        debug_println!("[IBCConnection]: save ibc config is {:?}", ibc_config);

        self.store_ibc_config(store, &ibc_config)?;

        debug_println!("[IBCConnection]: Channel Config Stored");

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
            source_port: ibc_config.src_endpoint().port_id.clone(),
            source_channel: ibc_config.src_endpoint().channel_id.clone(),
            destination_port: ibc_config.dst_endpoint().port_id.clone(),
            destination_channel: ibc_config.dst_endpoint().channel_id.clone(),
            data: rlp::encode(&data).to_vec(),
            timeout_height: Some(timeout_height.into()),
            timeout_timestamp: 0,
        };
        packet
    }
}
