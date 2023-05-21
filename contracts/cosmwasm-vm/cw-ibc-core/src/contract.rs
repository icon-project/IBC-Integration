use super::*;
use common::ibc::core::ics04_channel::packet::Receipt;

use cosmwasm_std::to_binary;
use cw_common::hex_string::HexString;
use cw_common::raw_types::channel::{
    RawChannel, RawMessageAcknowledgement, RawMessageRecvPacket, RawMessageTimeout,
    RawMessageTimeoutOnclose, RawMsgChannelCloseConfirm, RawMsgChannelOpenAck,
    RawMsgChannelOpenConfirm, RawMsgChannelOpenInit, RawMsgChannelOpenTry, RawPacket,
};
use cw_common::raw_types::connection::*;
use cw_common::raw_types::Any;
use cw_common::raw_types::Protobuf;
use cw_common::raw_types::RawHeight;
use hex::FromHexError;
use prost::{DecodeError, Message};

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
        _env: Env,
        info: MessageInfo,
        msg: CoreExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            CoreExecuteMsg::RegisterClient {
                client_type,
                client_address,
            } => {
                self.check_sender_is_owner(deps.as_ref().storage, info.sender.clone())?;
                let client_type = IbcClientType::new(client_type);
                self.register_client(deps, client_type, client_address)
            }
            CoreExecuteMsg::CreateClient {
                client_state,
                consensus_state,
                signer,
            } => {
                self.check_sender_is_owner(deps.as_ref().storage, info.sender.clone())?;
                let client_state_bytes = client_state
                    .to_bytes()
                    .map_err(|e| Into::<FromHexError>::into(e))?;
                let client_state = Any::decode(client_state_bytes.as_slice())
                    .map_err(|e| Into::<DecodeError>::into(e))?;
                let consensus_state_bytes = consensus_state
                    .to_bytes()
                    .map_err(|e| Into::<FromHexError>::into(e))?;
                let consensus_state = Any::decode(consensus_state_bytes.as_slice())
                    .map_err(|e| Into::<DecodeError>::into(e))?;

                let signer = Self::to_signer(&signer)?;
                let msg = IbcMsgCreateClient {
                    client_state: client_state,
                    consensus_state: consensus_state,
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
                let header_bytes = header
                    .to_bytes()
                    .map_err(|e| Into::<FromHexError>::into(e))?;
                let header = Any::decode(header_bytes.as_slice())
                    .map_err(|e| Into::<DecodeError>::into(e))?;

                let signer = Self::to_signer(&signer)?;
                let msg = IbcMsgUpdateClient {
                    client_id: IbcClientId::from_str(&client_id)
                        .map_err(|error| ContractError::IbcValidationError { error: error })?,
                    header,
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
                    port_id_on_a: IbcPortId::from_str(&port_id_on_a)
                        .map_err(|error| ContractError::IbcValidationError { error: error })?,
                    chan_id_on_a: IbcChannelId::from_str(&chan_id_on_a)
                        .map_err(|error| ContractError::IbcValidationError { error: error })?,
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
                let packet_bytes = packet
                    .to_bytes()
                    .map_err(|e| Into::<ContractError>::into(e))?;
                let packet: RawPacket = Message::decode(packet_bytes.as_slice())
                    .map_err(|error| ContractError::IbcDecodeError { error: error })?;

                let data: Packet = Packet::try_from(packet)
                    .map_err(|error| ContractError::IbcPacketError { error: error })?;

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
                        error: DecodeError::new(error.to_string()),
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
        // Ok(Response::new())
    }
    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetCommitment { key } => {
                let key_bytes = key
                    .to_bytes()
                    .map_err(|e| Into::<ContractError>::into(e))
                    .unwrap();
                let res = self
                    .get_commitment(deps.storage, key_bytes)
                    .map_err(|_| ContractError::InvalidCommitmentKey)
                    .unwrap();
                to_binary(&hex::encode(res))
            }
            QueryMsg::GetClientRegistry { _type } => {
                let res = self
                    .get_client_from_registry(deps.storage, IbcClientType::new(_type.clone()))
                    .map_err(|_| ContractError::InvalidClientType { client_type: _type })
                    .unwrap();
                let addr = Addr::unchecked(res);
                to_binary(&addr)
            }
            QueryMsg::GetClientType { client_id } => {
                let res = self
                    .get_client_type(deps.storage, ClientId::from_str(&client_id).unwrap())
                    .map_err(|_| ContractError::InvalidClientId { client_id })
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetClientImplementation { client_id } => {
                let res = self
                    .get_client_implementations(
                        deps.storage,
                        ClientId::from_str(&client_id).unwrap(),
                    )
                    .map_err(|_| ContractError::InvalidClientId { client_id })
                    .unwrap();
                let addr = Addr::unchecked(res);
                to_binary(&addr)
            }
            QueryMsg::GetConsensusState { client_id, height } => {
                let raw_height: RawHeight = RawHeight::decode(height.to_bytes().unwrap().as_ref())
                    .map_err(|_| ClientError::InvalidHeight)
                    .unwrap();
                let height =
                    Height::new(raw_height.revision_number, raw_height.revision_height).unwrap();

                let res = self
                    .consensus_state_any(
                        deps.storage,
                        &IbcClientId::from_str(&client_id).unwrap(),
                        &height,
                    )
                    .map_err(|e| {
                        println!("{:?}", e);
                        ContractError::InvalidClientId { client_id }
                    })
                    .unwrap();

                to_binary(&hex::encode(res.encode_to_vec()))
            }
            QueryMsg::GetClientState { client_id } => {
                let res = self
                    .client_state_any(deps.storage, &IbcClientId::from_str(&client_id).unwrap())
                    .map_err(|_| ContractError::InvalidClientId { client_id })
                    .unwrap();

                to_binary(&hex::encode(res.encode_to_vec()))
            }
            QueryMsg::GetConnection { connection_id } => {
                let _connection_id = ConnectionId::from_str(&connection_id).unwrap();
                let res = self.get_connection(deps.storage, _connection_id).unwrap();
                let connection_end = ConnectionEnd::decode_vec(res.as_slice()).unwrap();
                let raw_connection_end: RawConnectionEnd = connection_end.into();
                to_binary(&hex::encode(raw_connection_end.encode_to_vec()))
            }
            QueryMsg::GetChannel {
                port_id,
                channel_id,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = ChannelId::from(IbcChannelId::from_str(&channel_id).unwrap());
                let res = self
                    .get_channel_end(deps.storage, _port_id.clone(), _channel_id.clone())
                    .unwrap();
                let raw: RawChannel = res.into();
                to_binary(&hex::encode(raw.encode_to_vec()))
            }
            QueryMsg::GetNextSequenceSend {
                port_id,
                channel_id,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = ChannelId::from(IbcChannelId::from_str(&channel_id).unwrap());
                let res = self
                    .get_next_sequence_send(deps.storage, _port_id.clone(), _channel_id.clone())
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetNextSequenceReceive {
                port_id,
                channel_id,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = ChannelId::from(IbcChannelId::from_str(&channel_id).unwrap());
                let res = self
                    .get_next_sequence_recv(deps.storage, _port_id.clone(), _channel_id.clone())
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetNextSequenceAcknowledgement {
                port_id,
                channel_id,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = ChannelId::from(IbcChannelId::from_str(&channel_id).unwrap());
                let res = self
                    .get_next_sequence_ack(deps.storage, _port_id.clone(), _channel_id.clone())
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetCapability { name } => {
                let res = self
                    .get_capability(deps.storage, name.to_bytes().unwrap())
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetExpectedTimePerBlock => {
                let res = self.get_expected_time_per_block(deps.storage).unwrap();
                to_binary(&res)
            }
            QueryMsg::GetNextClientSequence => {
                let res = self
                    .client_counter(deps.storage)
                    .map_err(|_| ContractError::InvalidNextClientSequence {})
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetNextConnectionSequence => {
                let res = self.connection_counter(deps.storage).unwrap();
                to_binary(&res)
            }
            QueryMsg::GetNextChannelSequence => {
                let res = self.channel_counter(deps.storage).unwrap();
                to_binary(&res)
            }
            QueryMsg::GetPacketReceipt {
                port_id,
                channel_id,
                sequence,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = ChannelId::from(IbcChannelId::from_str(&channel_id).unwrap());
                let _sequence = Sequence::from(sequence);
                let _res = self
                    .get_packet_receipt(deps.storage, &_port_id, &_channel_id, _sequence.clone())
                    .unwrap();
                to_binary(&true)
            }
            QueryMsg::GetPacketCommitment {
                port_id,
                channel_id,
                sequence,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = ChannelId::from(IbcChannelId::from_str(&channel_id).unwrap());
                let _sequence = Sequence::from(sequence);
                let res = self
                    .get_packet_commitment(deps.storage, &_port_id, &_channel_id, _sequence.clone())
                    .unwrap();
                to_binary(&hex::encode(res.into_vec()))
            }
            QueryMsg::GetPacketAcknowledgementCommitment {
                port_id,
                channel_id,
                sequence,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = ChannelId::from(IbcChannelId::from_str(&channel_id).unwrap());
                let _sequence = Sequence::from(sequence);
                let res = self
                    .get_packet_acknowledgement(
                        deps.storage,
                        &_port_id,
                        &_channel_id,
                        _sequence.clone(),
                    )
                    .unwrap();
                to_binary(&hex::encode(res.into_vec()))
            }
            QueryMsg::HasPacketReceipt {
                port_id,
                channel_id,
                sequence,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = ChannelId::from(IbcChannelId::from_str(&channel_id).unwrap());
                let _sequence = Sequence::from(sequence);
                let res = self
                    .get_packet_receipt(deps.storage, &_port_id, &_channel_id, _sequence.clone())
                    .unwrap();
                match res {
                    Receipt::Ok => to_binary(&true),
                }
            }
        }
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
        //  Ok(Response::new())
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
        let bytes = hex_str.to_bytes()?;
        let raw = <R as Message>::decode(bytes.as_slice())
            .map_err(|error| ContractError::IbcDecodeError { error: error })?;
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
        let bytes = str.to_bytes()?;
        let signer_string =
            String::from_utf8(bytes).map_err(|error| ContractError::IbcDecodeError {
                error: DecodeError::new(error.to_string()),
            })?;

        let signer =
            Signer::from_str(&signer_string).map_err(|error| ContractError::IbcDecodeError {
                error: DecodeError::new(error.to_string()),
            })?;
        Ok(signer)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::context::CwIbcCoreContext;
    use common::ibc::core::ics02_client::height::Height;
    use common::{
        constants::ICON_CONSENSUS_STATE_TYPE_URL,
        icon::icon::lightclient::v1::ConsensusState as RawConsensusState, traits::AnyTypes,
    };

    use cw_common::ibc_types::IbcClientType;
    use prost::Message;

    use super::{instantiate, query, InstantiateMsg, QueryMsg};

    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, OwnedDeps,
    };
    use cw_common::raw_types::{Any, RawHeight};
    use cw_common::{hex_string::HexString, ibc_types::IbcClientId};

    const SENDER: &str = "sender";

    fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info(SENDER, &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        deps
    }

    #[test]
    fn test_query_next_client_sequence() {
        let contract = CwIbcCoreContext::default();
        let mut deps = setup();
        let msg = QueryMsg::GetNextClientSequence;
        let result = query(deps.as_ref(), mock_env(), msg.clone()).unwrap();
        let result_parsed: u64 = from_binary(&result).unwrap();
        assert_eq!(0, result_parsed);

        contract
            .increase_client_counter(deps.as_mut().storage)
            .unwrap();
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let result_parsed: u64 = from_binary(&result).unwrap();
        assert_eq!(1, result_parsed);
    }

    #[test]
    fn test_query_get_client_registry() {
        let client_type_str = "test_client_type".to_string();
        let client = "test_client".to_string();
        let client_type = IbcClientType::new(client_type_str.clone());
        let contract = CwIbcCoreContext::default();
        let mut deps = setup();

        contract
            .store_client_into_registry(deps.as_mut().storage, client_type, client.clone())
            .unwrap();

        let msg = QueryMsg::GetClientRegistry {
            _type: client_type_str.clone(),
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let result_parsed: Addr = from_binary(&result).unwrap();
        assert_eq!(client, result_parsed.as_str());
    }

    #[test]
    fn test_query_get_consensus_state() {
        let contract = CwIbcCoreContext::default();
        let client_id = "test_client_1".to_string();
        let mut deps = setup();
        let commitment_root =
            "0x7702db70e830e07b4ff46313456fc86d677c7eeca0c011d7e7dcdd48d5aacfe2".to_string();
        let consensus_state = RawConsensusState {
            message_root: commitment_root.encode_to_vec(),
        };

        let height = Height::new(123, 456).unwrap();
        let raw_height: RawHeight = RawHeight::from(height);
        contract
            .store_consensus_state(
                deps.as_mut().storage,
                &IbcClientId::from_str(&client_id).unwrap(),
                height,
                consensus_state.to_any().encode_to_vec(),
            )
            .unwrap();

        let msg = QueryMsg::GetConsensusState {
            client_id,
            height: HexString::from_bytes(&raw_height.encode_to_vec()),
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let result_parsed: String = from_binary(&result).unwrap();
        let result_bytes = hex::decode(result_parsed).unwrap();

        let result_decoded = Any::decode(result_bytes.as_ref()).unwrap();
        println!("{:?}", result_decoded);
        assert_eq!(
            ICON_CONSENSUS_STATE_TYPE_URL.to_string(),
            result_decoded.type_url
        );
    }
}
