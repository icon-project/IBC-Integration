use super::*;
use common::icon::icon::lightclient::v1::{
    ClientState as RawClientState, ConsensusState as RawConsensusState,
};
use common::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use cw_common::hex_string::HexString;
use cw_common::raw_types::channel::*;
use cw_common::raw_types::connection::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-ibc-core";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwIbcCoreContext<'a> {
   /// The `instantiate` function initializes the contract and sets default values for channel, client,
   /// and connection counters, as well as setting the contract owner.
   /// 
   /// Arguments:
   /// 
   /// * `deps`: A mutable reference to the dependencies of the contract, which includes access to the
   /// storage, API, and other contracts.
   /// * `_env`: _env is an object that represents the current blockchain environment, including
   /// information such as the block height, time, and chain ID. It is passed as a parameter to the
   /// instantiate function in the CosmWasm smart contract framework.
   /// * `info`: MessageInfo is a struct that contains information about the message being executed,
   /// such as the sender address, the amount of tokens sent with the message, and the message ID. It is
   /// passed as a parameter to the instantiate function in order to set the owner of the contract.
   /// * `_msg`: The `_msg` parameter is of type `InstantiateMsg` and represents the message sent by the
   /// user to instantiate the contract. It contains any custom data or parameters required for the
   /// contract initialization.
   /// 
   /// Returns:
   /// 
   /// A `Result<Response, ContractError>` is being returned. The `Response` contains an attribute
   /// "method" with the value "instantiate". If there are no errors, the `Result` will be `Ok`,
   /// otherwise it will be `Err(ContractError::Std)`.
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;

        self.init_channel_counter(deps.storage, u64::default())?;
        self.init_client_counter(deps.storage, u64::default())?;
        self.init_connection_counter(deps.storage, u64::default())?;
        self.set_owner(deps.storage, info.sender)?;

        Ok(Response::new().add_attribute("method", "instantiate"))
    }

    /// This function handles the execution of various IBC-related messages in a contract.
    /// 
    /// Arguments:
    /// 
    /// * `deps`: `deps` is a `DepsMut` object, which provides access to the contract's dependencies
    /// such as storage, querier, and API interfaces. It is used to interact with the blockchain and
    /// other contracts.
    /// * `env`: `env` is a struct that contains information about the current execution environment,
    /// such as the block height and time, the chain ID, and the sender address. It is passed as an
    /// argument to the `execute` function in a CosmWasm smart contract.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being executed, such as the sender address, the amount of funds sent with the message, and the
    /// gas limit. It is passed as an argument to the `execute` function in the Cosmos SDK.
    /// * `msg`: The `msg` parameter in the `execute` function is of type `CoreExecuteMsg` and
    /// represents the message that the contract should execute. The function matches the type of the
    /// message and calls the appropriate function to handle it.
    /// 
    /// Returns:
    /// 
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing possible errors that can occur
    /// during contract execution.
    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: CoreExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            CoreExecuteMsg::RegisterClient {
                client_type,
                client_address,
            } => {
                self.check_sender_is_owner(deps.as_ref().storage, info.sender.clone())?;
                let client_type = ClientType::new(client_type);
                self.register_client(deps, client_type, client_address)
            }
            CoreExecuteMsg::CreateClient {
                client_state,
                consensus_state,
                signer,
            } => {
                self.check_sender_is_owner(deps.as_ref().storage, info.sender.clone())?;

                let client_state = Self::from_raw::<RawClientState, ClientState>(&client_state)?;
                let consensus_state =
                    Self::from_raw::<RawConsensusState, ConsensusState>(&consensus_state)?;

                let signer = Self::to_signer(&signer)?;
                let msg = IbcMsgCreateClient {
                    client_state: client_state.into(),
                    consensus_state: consensus_state.into(),
                    signer,
                };
                self.create_client(deps, info, msg)
            }
            CoreExecuteMsg::UpdateClient {
                client_id,
                header,
                signer,
            } => {
                self.check_sender_is_owner(deps.as_ref().storage, info.sender.clone())?;

                let header = Self::from_raw::<RawSignedHeader, SignedHeader>(&header)?;

                let signer = Self::to_signer(&signer)?;
                let msg = IbcMsgUpdateClient {
                    client_id: IbcClientId::from_str(&client_id).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    header: header.into(),
                    signer,
                };
                println!("Updating Client For {}", &client_id);
                self.update_client(deps, info, msg)
            }
            CoreExecuteMsg::UpgradeClient {} => {
                unimplemented!()
            }
            CoreExecuteMsg::ClientMisbehaviour {} => {
                unimplemented!()
            }
            CoreExecuteMsg::ConnectionOpenInit { msg } => {
                let message: MsgConnectionOpenInit =
                    Self::from_raw::<RawMsgConnectionOpenInit, MsgConnectionOpenInit>(&msg)?;
                self.connection_open_init(deps, message)
            }
            CoreExecuteMsg::ConnectionOpenTry { msg } => {
                let message: MsgConnectionOpenTry =
                    Self::from_raw::<RawMsgConnectionOpenTry, MsgConnectionOpenTry>(&msg)?;
                self.connection_open_try(deps, info, message)
            }
            CoreExecuteMsg::ConnectionOpenAck { msg } => {
                let message: MsgConnectionOpenAck =
                    Self::from_raw::<RawMsgConnectionOpenAck, MsgConnectionOpenAck>(&msg)?;
                self.connection_open_ack(deps, info, message)
            }
            CoreExecuteMsg::ConnectionOpenConfirm { msg } => {
                let message: MsgConnectionOpenConfirm =
                    Self::from_raw::<RawMsgConnectionOpenConfirm, MsgConnectionOpenConfirm>(&msg)?;
                self.connection_open_confirm(deps, info, message)
            }
            CoreExecuteMsg::ChannelOpenInit { msg } => {
                let message: MsgChannelOpenInit =
                    Self::from_raw::<RawMsgChannelOpenInit, MsgChannelOpenInit>(&msg)?;
                self.validate_channel_open_init(deps, info, &message)
            }
            CoreExecuteMsg::ChannelOpenTry { msg } => {
                let message: MsgChannelOpenTry =
                    Self::from_raw::<RawMsgChannelOpenTry, MsgChannelOpenTry>(&msg)?;
                self.validate_channel_open_try(deps, info, &message)
            }
            CoreExecuteMsg::ChannelOpenAck { msg } => {
                let message: MsgChannelOpenAck =
                    Self::from_raw::<RawMsgChannelOpenAck, MsgChannelOpenAck>(&msg)?;
                self.validate_channel_open_ack(deps, info, &message)
            }
            CoreExecuteMsg::ChannelOpenConfirm { msg } => {
                let message: MsgChannelOpenConfirm =
                    Self::from_raw::<RawMsgChannelOpenConfirm, MsgChannelOpenConfirm>(&msg)?;
                self.validate_channel_open_confirm(deps, info, &message)
            }
            CoreExecuteMsg::ChannelCloseInit {
                port_id_on_a,
                chan_id_on_a,
                signer,
            } => {
                let signer = Self::to_signer(&signer)?;
                let message = MsgChannelCloseInit {
                    port_id_on_a: IbcPortId::from_str(&port_id_on_a).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    chan_id_on_a: IbcChannelId::from_str(&chan_id_on_a).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    signer,
                };

                self.validate_channel_close_init(deps, info, &message)
            }
            CoreExecuteMsg::ChannelCloseConfirm { msg } => {
                let message: MsgChannelCloseConfirm =
                    Self::from_raw::<RawMsgChannelCloseConfirm, MsgChannelCloseConfirm>(&msg)?;

                self.validate_channel_close_confirm(deps, info, &message)
            }
            CoreExecuteMsg::SendPacket { packet } => {
                let packet_bytes = packet.to_bytes().unwrap();
                let packet: RawPacket =
                    Message::decode(packet_bytes.as_slice()).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?;

                let data: Packet = Packet::try_from(packet)
                    .map_err(|error| ContractError::IbcPacketError { error })?;

                self.send_packet(deps, data)
            }
            CoreExecuteMsg::ReceivePacket { msg } => {
                let message: MsgRecvPacket =
                    Self::from_raw::<RawMessageRecvPacket, MsgRecvPacket>(&msg)?;
                self.validate_receive_packet(deps, info, &message)
            }
            CoreExecuteMsg::AcknowledgementPacket { msg } => {
                let message: MsgAcknowledgement =
                    Self::from_raw::<RawMessageAcknowledgement, MsgAcknowledgement>(&msg)?;
                self.acknowledgement_packet_validate(deps, info, &message)
            }
            CoreExecuteMsg::RequestTimeout {} => todo!(),
            CoreExecuteMsg::Timeout { msg } => {
                let message: MsgTimeout = Self::from_raw::<RawMessageTimeout, MsgTimeout>(&msg)?;
                self.timeout_packet_validate(
                    deps,
                    info,
                    cw_common::types::TimeoutMsgType::Timeout(message),
                )
            }
            CoreExecuteMsg::TimeoutOnClose { msg } => {
                let message: MsgTimeoutOnClose =
                    Self::from_raw::<RawMessageTimeoutOnclose, MsgTimeoutOnClose>(&msg)?;
                self.timeout_packet_validate(
                    deps,
                    info,
                    cw_common::types::TimeoutMsgType::TimeoutOnClose(message),
                )
            }
            CoreExecuteMsg::BindPort { port_id, address } => {
                let port_id = IbcPortId::from_str(&port_id).map_err(|error| {
                    ContractError::IbcDecodeError {
                        error: error.to_string(),
                    }
                })?;
                self.bind_port(deps.storage, &port_id, address)
            }
            CoreExecuteMsg::SetExpectedTimePerBlock { block_time } => {
                self.set_expected_time_per_block(deps.storage, block_time)?;
                Ok(Response::new()
                    .add_attribute("method", "set_expected_time_per_block")
                    .add_attribute("time", block_time.to_string()))
            }
        }
    }
    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        todo!()
    }

    /// This function handles different types of replies based on their ID and executes the
    /// corresponding function.
    /// 
    /// Arguments:
    /// 
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract, which includes
    /// access to the storage, API, and other modules. It is of type `DepsMut`.
    /// * `_env`: _env is an object of type `Env` which represents the environment in which the contract
    /// is executing. It contains information such as the current block height, the current time, and
    /// the address of the contract.
    /// * `message`: The `message` parameter is of type `Reply` and contains information about the reply
    /// being processed, including the ID of the reply and any associated data.
    /// 
    /// Returns:
    /// 
    /// a `Result<Response, ContractError>` where `Response` and `ContractError` are defined types.
    pub fn reply(
        &self,
        deps: DepsMut,
        _env: Env,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.id {
            EXECUTE_CREATE_CLIENT => self.execute_create_client_reply(deps, message),
            EXECUTE_UPDATE_CLIENT => self.execute_update_client_reply(deps, message),
            EXECUTE_UPGRADE_CLIENT => self.execute_upgrade_client_reply(deps, message),
            MISBEHAVIOUR => self.execute_misbehaviour_reply(deps, message),
            EXECUTE_CONNECTION_OPENTRY => self.execute_connection_open_try(deps, message),
            EXECUTE_CONNECTION_OPENACK => self.execute_connection_open_ack(deps, message),
            EXECUTE_CONNECTION_OPENCONFIRM => self.execute_connection_openconfirm(deps, message),
            EXECUTE_ON_CHANNEL_OPEN_INIT => self.execute_channel_open_init(deps, message),
            EXECUTE_ON_CHANNEL_OPEN_TRY => self.execute_channel_open_try(deps, message),
            EXECUTE_ON_CHANNEL_OPEN_TRY_ON_LIGHT_CLIENT => {
                self.execute_open_try_from_light_client(deps, message)
            }
            EXECUTE_ON_CHANNEL_OPEN_ACK_ON_LIGHT_CLIENT => {
                self.execute_open_ack_from_light_client_reply(deps, message)
            }
            EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE => self.execute_channel_open_ack(deps, message),
            EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_LIGHT_CLIENT => {
                self.execute_open_confirm_from_light_client_reply(deps, message)
            }
            EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE => {
                self.execute_channel_open_confirm(deps, message)
            }
            EXECUTE_ON_CHANNEL_CLOSE_INIT => self.execute_channel_close_init(deps, message),
            EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_LIGHT_CLIENT => {
                self.execute_close_confirm_from_light_client_reply(deps, message)
            }
            EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE => {
                self.execute_channel_close_confirm(deps, message)
            }

            VALIDATE_ON_PACKET_TIMEOUT_ON_LIGHT_CLIENT => {
                self.timeout_packet_validate_reply_from_light_client(deps, message)
            }
            VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE => self.execute_timeout_packet(deps, message),
            VALIDATE_ON_PACKET_RECEIVE_ON_LIGHT_CLIENT => {
                self.receive_packet_validate_reply_from_light_client(deps, message)
            }
            VALIDATE_ON_PACKET_RECEIVE_ON_MODULE => self.execute_receive_packet(deps, message),
            VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_LIGHT_CLIENT => {
                self.acknowledgement_packet_validate_reply_from_light_client(deps, message)
            }
            VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE => {
                self.acknowledgement_packet_execute(deps, message)
            }

            _ => Err(ContractError::ReplyError {
                code: message.id,
                msg: "InvalidReplyID".to_string(),
            }),
        }
    }

    /// This function calculates the fee for a given expected gas amount and gas price.
    /// 
    /// Arguments:
    /// 
    /// * `expected_gas`: `expected_gas` is an input parameter of type `u64` which represents the
    /// expected amount of gas required to execute a transaction on the blockchain. Gas is a unit of
    /// measurement for the computational effort required to execute a transaction or contract on the
    /// Ethereum network. The higher the gas limit, the more
    /// 
    /// Returns:
    /// 
    /// The function `calculate_fee` returns a `u128` value, which represents the calculated fee based
    /// on the expected gas and the gas price.
    pub fn calculate_fee(&self, expected_gas: u64) -> u128 {
        let fee = expected_gas as u128 * self.gas_price();

        fee.checked_div(GAS_DENOMINATOR as u128).unwrap()
    }

    /// This function calculates the gas price in Rust programming language.
    /// 
    /// Returns:
    /// 
    /// an unsigned 128-bit integer, which represents the gas price calculated based on the default gas
    /// numerator and gas adjustment numerator values.
    pub fn gas_price(&self) -> u128 {
        let price = GAS_NUMERATOR_DEFAULT * GAS_ADJUSTMENT_NUMERATOR_DEFAULT;

        price.checked_div(GAS_DENOMINATOR).unwrap();

        price as u128
    }
    /// The function updates the balance of each coin in a vector by subtracting a fee and returns the
    /// updated vector.
    /// 
    /// Arguments:
    /// 
    /// * `coins`: A vector of `Coin` structs representing the current balance of the user. Each `Coin`
    /// struct contains an amount and a denomination.
    /// * `fee`: The `fee` parameter is an unsigned 128-bit integer representing the amount of fee to be
    /// deducted from each coin's balance.
    /// 
    /// Returns:
    /// 
    /// a `Result` type with a vector of `Coin` objects as the successful result or a `ContractError` if
    /// there is an insufficient balance.
    pub fn update_fee(&self, coins: Vec<Coin>, fee: u128) -> Result<Vec<Coin>, ContractError> {
        if coins.is_empty() {
            return Err(ContractError::InsufficientBalance {});
        }

        let updated_coins = coins
            .into_iter()
            .map(|coin| {
                let updated_balance = coin.amount.u128().checked_sub(fee).unwrap();

                Coin::new(updated_balance, coin.denom)
            })
            .collect::<Vec<Coin>>();

        Ok(updated_coins)
    }

    /// This function converts a hexadecimal string to a Rust type that implements the Message trait and
    /// can be converted to another type using the TryFrom trait.
    /// 
    /// Arguments:
    /// 
    /// * `hex_str`: A hexadecimal string that represents the serialized bytes of a protobuf message.
    /// 
    /// Returns:
    /// 
    /// a `Result` with a generic type `T` which is the converted message from the raw bytes provided in
    /// the `HexString`. If the conversion is successful, it returns an `Ok` variant with the converted
    /// message. If there is an error during the conversion, it returns an `Err` variant with a
    /// `ContractError` that describes the error encountered.
    pub fn from_raw<R: Message + std::default::Default + Clone, T: TryFrom<R>>(
        hex_str: &HexString,
    ) -> Result<T, ContractError>
    where
        <T as TryFrom<R>>::Error: std::fmt::Debug,
    {
        let bytes = hex_str
            .to_bytes()
            .map_err(|e| ContractError::IbcDecodeError {
                error: e.to_string(),
            })?;
        let raw = <R as Message>::decode(bytes.as_slice()).map_err(|error| {
            ContractError::IbcDecodeError {
                error: error.to_string(),
            }
        })?;
        let message = T::try_from(raw).map_err(|error| {
            let err = format!("Failed to convert to ibc type with error {:?}", error);
            ContractError::IbcRawConversionError { error: err }
        })?;
        Ok(message)
    }

    /// The function converts a hexadecimal string to a Signer object and returns an error if the
    /// conversion fails.
    /// 
    /// Arguments:
    /// 
    /// * `str`: A hexadecimal string representing a signer address.
    /// 
    /// Returns:
    /// 
    /// This function returns a `Result` containing either a `Signer` or a `ContractError`.
    pub fn to_signer(str: &HexString) -> Result<Signer, ContractError> {
        let bytes = str.to_bytes().map_err(|e| ContractError::IbcDecodeError {
            error: e.to_string(),
        })?;
        let signer_string =
            String::from_utf8(bytes).map_err(|error| ContractError::IbcDecodeError {
                error: error.to_string(),
            })?;

        let signer =
            Signer::from_str(&signer_string).map_err(|error| ContractError::IbcDecodeError {
                error: error.to_string(),
            })?;
        Ok(signer)
    }
}
