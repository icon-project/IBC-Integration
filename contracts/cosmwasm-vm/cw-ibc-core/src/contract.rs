use std::env;

use super::*;

use cosmwasm_std::to_binary;

use cw_common::hex_string::HexString;
use cw_common::query_helpers::build_smart_query;
use cw_common::raw_types::channel::RawMsgChannelCloseInit;
use cw_common::raw_types::channel::{
    RawChannel, RawMessageAcknowledgement, RawMessageRecvPacket, RawMessageTimeout,
    RawMessageTimeoutOnclose, RawMsgChannelCloseConfirm, RawMsgChannelOpenAck,
    RawMsgChannelOpenConfirm, RawMsgChannelOpenInit, RawMsgChannelOpenTry, RawPacket,
};
use cw_common::raw_types::client::{RawMsgCreateClient, RawMsgUpdateClient};
use cw_common::raw_types::connection::*;
use cw_common::raw_types::Protobuf;

use cw_common::{cw_println, to_checked_address};

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
        cw_println!(deps, "{:?}", info.funds);

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
                self.check_sender_is_owner(deps.as_ref().storage, info.sender)?;
                let client_type = IbcClientType::new(client_type);
                let client_address = to_checked_address(deps.as_ref(), client_address.as_ref());
                self.register_client(deps, client_type, client_address)
            }
            CoreExecuteMsg::CreateClient { msg } => {
                cw_println!(deps, "[IBCCore] CreateClient Called");
                let message: RawMsgCreateClient = Self::raw_from_hex(&msg)?;
                self.create_client(deps, info, env, message)
            }
            CoreExecuteMsg::UpdateClient { msg } => {
                cw_println!(deps, "[IBCCore] UpdateClient Called");
                let message: RawMsgUpdateClient = Self::raw_from_hex(&msg)?;
                self.update_client(deps, info, message)
            }
            CoreExecuteMsg::UpgradeClient {} => {
                unimplemented!()
            }
            CoreExecuteMsg::ClientMisbehaviour {} => {
                unimplemented!()
            }
            CoreExecuteMsg::ConnectionOpenInit { msg } => {
                cw_println!(deps, "[IBCCore] Connection Open Init Called");
                let message: RawMsgConnectionOpenInit = Self::raw_from_hex(&msg)?;
                self.connection_open_init(deps, message)
            }
            CoreExecuteMsg::ConnectionOpenTry { msg } => {
                cw_println!(deps, "[IBCCore] Connection Open Try Called");
                let message: RawMsgConnectionOpenTry = Self::raw_from_hex(&msg)?;
                self.connection_open_try(deps, info, env, message)
            }
            CoreExecuteMsg::ConnectionOpenAck { msg } => {
                cw_println!(deps, "[IBCCore] Connection Open Ack Called");
                let message: RawMsgConnectionOpenAck = Self::raw_from_hex(&msg)?;
                self.connection_open_ack(deps, info, env, message)
            }
            CoreExecuteMsg::ConnectionOpenConfirm { msg } => {
                cw_println!(deps, "[IBCCore] Connection Open Confirm Called");
                let message: RawMsgConnectionOpenConfirm = Self::raw_from_hex(&msg)?;
                self.connection_open_confirm(deps, env, info, message)
            }
            CoreExecuteMsg::ChannelOpenInit { msg } => {
                cw_println!(deps, "[IBCCore] Channel Open Init Called");
                let message = Self::raw_from_hex::<RawMsgChannelOpenInit>(&msg)?;

                self.validate_channel_open_init(deps, info, &message)
            }
            CoreExecuteMsg::ChannelOpenTry { msg } => {
                cw_println!(deps, "[IBCCore] Channel Open Try Called");
                let message: RawMsgChannelOpenTry = Self::raw_from_hex(&msg)?;
                self.validate_channel_open_try(deps, info, &message)
            }
            CoreExecuteMsg::ChannelOpenAck { msg } => {
                cw_println!(deps, "[IBCCore] Channel Open Ack Called");
                let message: RawMsgChannelOpenAck = Self::raw_from_hex(&msg)?;
                self.validate_channel_open_ack(deps, info, &message)
            }
            CoreExecuteMsg::ChannelOpenConfirm { msg } => {
                cw_println!(deps, "[IBCCore] Channel Open Confirm Called");
                let message: RawMsgChannelOpenConfirm = Self::raw_from_hex(&msg)?;
                self.validate_channel_open_confirm(deps, info, &message)
            }
            CoreExecuteMsg::ChannelCloseInit { msg } => {
                cw_println!(deps, "[IBCCore] Channel Close Init Called");
                let message: RawMsgChannelCloseInit = Self::raw_from_hex(&msg)?;
                self.validate_channel_close_init(deps, info, &message)
            }
            CoreExecuteMsg::ChannelCloseConfirm { msg } => {
                cw_println!(deps, "[IBCCore] Channel Close Confirm Called");
                let message: RawMsgChannelCloseConfirm = Self::raw_from_hex(&msg)?;
                self.validate_channel_close_confirm(deps, info, &message)
            }
            CoreExecuteMsg::SendPacket { packet } => {
                cw_println!(deps, "[IBCCore] Send Packet Called");
                let packet_bytes = packet.to_bytes().map_err(Into::<ContractError>::into)?;
                let packet: RawPacket = Message::decode(packet_bytes.as_slice())
                    .map_err(|error| ContractError::IbcDecodeError { error })?;

                self.send_packet(deps, &env, info, packet)
            }
            CoreExecuteMsg::ReceivePacket { msg } => {
                cw_println!(deps, "[IBCCore] Receive Packet Called");
                let message = Self::raw_from_hex::<RawMessageRecvPacket>(&msg)?;

                self.validate_receive_packet(deps, info, env, &message)
            }
            CoreExecuteMsg::AcknowledgementPacket { msg } => {
                cw_println!(deps, "[IBCCore] Acknowledgement Packet Called");
                let message = Self::raw_from_hex::<RawMessageAcknowledgement>(&msg)?;
                self.acknowledgement_packet_validate(deps, info, env, &message)
            }
            CoreExecuteMsg::TimeoutPacket { msg } => {
                cw_println!(deps, "[IBCCore] Timeout Packet Called");
                let message = Self::raw_from_hex::<RawMessageTimeout>(&msg)?;
                self.timeout_packet_validate(
                    deps,
                    env,
                    info,
                    cw_common::types::TimeoutMsgType::Timeout(message),
                )
            }
            CoreExecuteMsg::TimeoutOnClose { msg } => {
                cw_println!(deps, "[IBCCore] Timeout On Close Called");
                let message = Self::raw_from_hex::<RawMessageTimeoutOnclose>(&msg)?;
                self.timeout_packet_validate(
                    deps,
                    env,
                    info,
                    cw_common::types::TimeoutMsgType::TimeoutOnClose(message),
                )
            }
            CoreExecuteMsg::BindPort { port_id, address } => {
                cw_println!(deps, "[IBCCore] Bind Port Called");
                let port_id = IbcPortId::from_str(&port_id).map_err(|error| {
                    ContractError::IbcDecodeError {
                        error: DecodeError::new(error.to_string()),
                    }
                })?;
                let checked_address = to_checked_address(deps.as_ref(), &address).to_string();
                self.bind_port(deps.storage, &port_id, checked_address)
            }
            CoreExecuteMsg::SetExpectedTimePerBlock { block_time } => {
                self.check_sender_is_owner(deps.as_ref().storage, info.sender)?;
                self.set_expected_time_per_block(deps.storage, block_time)?;
                Ok(Response::new()
                    .add_attribute("method", "set_expected_time_per_block")
                    .add_attribute("time", block_time.to_string()))
            }
            CoreExecuteMsg::WriteAcknowledgement {
                packet,
                acknowledgement,
            } => {
                cw_println!(deps, "[IBCCore] Write Acknowledgement Called");
                let ack = acknowledgement.to_bytes()?;
                self.write_acknowledgement(deps, info, packet, ack)
            }
        }
        // Ok(Response::new())
    }
    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetCommitment { key } => {
                let key_bytes = key.to_bytes().map_err(Into::<ContractError>::into).unwrap();
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
                    .get_client_type(deps.storage, &ClientId::from_str(&client_id).unwrap())
                    .map_err(|_| ContractError::InvalidClientId { client_id })
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetClientImplementation { client_id } => {
                let res = self
                    .get_client_implementations(
                        deps.storage,
                        &ClientId::from_str(&client_id).unwrap(),
                    )
                    .map_err(|_| ContractError::InvalidClientId { client_id })
                    .unwrap();
                let addr = Addr::unchecked(res.get_address());
                to_binary(&addr)
            }
            QueryMsg::GetConsensusState { client_id } => {
                let res = self
                    .consensus_state_any(deps, &IbcClientId::from_str(&client_id).unwrap())
                    .map_err(|_e| {
                        cw_println!(deps, "{_e:?}");
                        ContractError::InvalidClientId { client_id }
                    })
                    .unwrap();

                to_binary(&hex::encode(res.encode_to_vec()))
            }
            QueryMsg::GetConsensusStateByHeight { client_id, height } => {
                let client_val = IbcClientId::from_str(&client_id).unwrap();
                let client = self.get_light_client(deps.storage, &client_val).unwrap();
                let res = client
                    .get_consensus_state(deps, &client_val, height)
                    .unwrap();
                let state = res.as_bytes();
                to_binary(&hex::encode(state))
            }
            QueryMsg::GetClientState { client_id } => {
                let res = self
                    .client_state_any(deps, &IbcClientId::from_str(&client_id).unwrap())
                    .map_err(|_| ContractError::InvalidClientId { client_id })
                    .unwrap();

                to_binary(&hex::encode(res.encode_to_vec()))
            }
            QueryMsg::GetConnection { connection_id } => {
                let connection_id = ConnectionId::from_str(&connection_id).unwrap();
                let res = self.get_connection(deps.storage, &connection_id).unwrap();
                let connection_end = ConnectionEnd::decode_vec(res.as_slice()).unwrap();
                let raw_connection_end: RawConnectionEnd = connection_end.into();
                to_binary(&hex::encode(raw_connection_end.encode_to_vec()))
            }
            QueryMsg::GetChannel {
                port_id,
                channel_id,
            } => {
                let port_id = PortId::from_str(&port_id).unwrap();
                let channel_id = IbcChannelId::from_str(&channel_id).unwrap();
                let res = self
                    .get_channel_end(deps.storage, &port_id, &channel_id)
                    .unwrap();
                let raw: RawChannel = res.into();
                to_binary(&hex::encode(raw.encode_to_vec()))
            }
            QueryMsg::GetNextSequenceSend {
                port_id,
                channel_id,
            } => {
                let port_id = PortId::from_str(&port_id).unwrap();
                let channel_id = IbcChannelId::from_str(&channel_id).unwrap();
                let res = self
                    .get_next_sequence_send(deps.storage, &port_id, &channel_id)
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetNextSequenceReceive {
                port_id,
                channel_id,
            } => {
                let port_id = PortId::from_str(&port_id).unwrap();
                let channel_id = IbcChannelId::from_str(&channel_id).unwrap();
                let res = self
                    .get_next_sequence_recv(deps.storage, &port_id, &channel_id)
                    .unwrap();
                let sequence: u64 = res.into();
                to_binary(&sequence)
            }
            QueryMsg::GetNextSequenceAcknowledgement {
                port_id,
                channel_id,
            } => {
                let port_id = PortId::from_str(&port_id).unwrap();
                let channel_id = IbcChannelId::from_str(&channel_id).unwrap();
                let res = self
                    .get_next_sequence_ack(deps.storage, &port_id, &channel_id)
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetCapability { name } => {
                let res = self
                    .get_capability(deps.storage, name.as_bytes().to_vec())
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetExpectedTimePerBlock {} => {
                let res = self.get_expected_time_per_block(deps.storage).unwrap();
                to_binary(&res)
            }
            QueryMsg::GetNextClientSequence {} => {
                let res = self
                    .client_counter(deps.storage)
                    .map_err(|_| ContractError::InvalidNextClientSequence {})
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetNextConnectionSequence {} => {
                let res = self.connection_counter(deps.storage).unwrap();
                to_binary(&res)
            }
            QueryMsg::GetNextChannelSequence {} => {
                let res = self.channel_counter(deps.storage).unwrap();
                to_binary(&res)
            }
            QueryMsg::GetPacketReceipt {
                port_id,
                channel_id,
                sequence,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = IbcChannelId::from_str(&channel_id).unwrap();
                let _sequence = Sequence::from(sequence);
                let _res = self
                    .get_packet_receipt(deps.storage, &_port_id, &_channel_id, _sequence)
                    .unwrap();
                to_binary(&true)
            }
            QueryMsg::GetPacketCommitment {
                port_id,
                channel_id,
                sequence,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = IbcChannelId::from_str(&channel_id).unwrap();
                let _sequence = Sequence::from(sequence);
                let res = self
                    .get_packet_commitment(deps.storage, &_port_id, &_channel_id, _sequence)
                    .unwrap();
                to_binary(&hex::encode(res.into_vec()))
            }
            QueryMsg::GetPacketAcknowledgementCommitment {
                port_id,
                channel_id,
                sequence,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = IbcChannelId::from_str(&channel_id).unwrap();
                let _sequence = Sequence::from(sequence);
                let res = self
                    .get_packet_acknowledgement(deps.storage, &_port_id, &_channel_id, _sequence)
                    .unwrap();
                to_binary(&hex::encode(res.into_vec()))
            }
            QueryMsg::HasPacketReceipt {
                port_id,
                channel_id,
                sequence,
            } => {
                let _port_id = PortId::from_str(&port_id).unwrap();
                let _channel_id = IbcChannelId::from_str(&channel_id).unwrap();
                let _sequence = Sequence::from(sequence);
                let res = self.get_packet_receipt(deps.storage, &_port_id, &_channel_id, _sequence);
                to_binary(&res.is_ok())
            }
            QueryMsg::GetAllPorts {} => {
                let ports = self.get_all_ports(deps.storage).unwrap();
                to_binary(&ports)
            }

            QueryMsg::GetCommitmentPrefix {} => {
                let prefix = self.commitment_prefix(deps, &_env);
                to_binary(&hex::encode(prefix.into_vec()))
            }
            QueryMsg::GetLatestHeight { client_id } => {
                let msg = to_binary(&cw_common::client_msg::QueryMsg::GetLatestHeight {
                    client_id: client_id.clone(),
                })?;
                let client = self
                    .get_client_implementations(
                        deps.storage,
                        &IbcClientId::from_str(&client_id).unwrap(),
                    )
                    .unwrap();
                let query = build_smart_query(client.get_address(), msg);
                let height: u64 = deps.querier.query(&query).unwrap();

                to_binary(&height)
            }
            QueryMsg::GetPacketHeights {
                port_id,
                channel_id,
                start_sequence,
                end_sequence,
            } => {
                let port_id = IbcPortId::from_str(&port_id).unwrap();
                let channel_id = IbcChannelId::from_str(&channel_id).unwrap();
                let heights = self
                    .ibc_store()
                    .get_packet_heights(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        start_sequence,
                        end_sequence,
                    )
                    .unwrap();
                to_binary(&heights)
            }

            QueryMsg::GetMissingPacketReceipts {
                port_id,
                channel_id,
                start_sequence,
                end_sequence,
            } => {
                let port_id = IbcPortId::from_str(&port_id).unwrap();
                let channel_id = IbcChannelId::from_str(&channel_id).unwrap();
                let missing = self
                    .ibc_store()
                    .get_missing_packet_receipts(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        start_sequence,
                        end_sequence,
                    )
                    .unwrap();
                to_binary(&missing)
            }
            QueryMsg::GetPreviousConsensusStateHeight { client_id, height } => {
                let client_val = IbcClientId::from_str(&client_id).unwrap();
                let client = self.get_light_client(deps.storage, &client_val).unwrap();
                let res = client
                    .get_previous_consensus_state(deps, &client_val, height)
                    .unwrap();
                to_binary(&res)
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
        env: Env,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.id {
            EXECUTE_UPDATE_CLIENT => self.execute_update_client_reply(deps, env, message),
            EXECUTE_UPGRADE_CLIENT => self.execute_upgrade_client_reply(deps, env, message),
            MISBEHAVIOUR => self.execute_misbehaviour_reply(deps, env, message),
            VALIDATE_ON_PACKET_RECEIVE_ON_MODULE => self.execute_receive_packet(deps, message),

            _ => Err(ContractError::ReplyError {
                code: message.id,
                msg: "InvalidReplyID".to_string(),
            }),
        }
    }

    pub fn migrate(
        &self,
        deps: DepsMut,
        _env: Env,
        msg: MigrateMsg,
    ) -> Result<Response, ContractError> {
        if msg.clear_store {
            let store = CwIbcStore::default();
            store.clear_storage(deps.storage);
        }
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(Response::default().add_attribute("migrate", "successful"))
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
        let raw = Self::raw_from_hex::<R>(hex_str)?;
        let message = T::try_from(raw).map_err(|error| {
            let err = format!("Failed to convert to ibc type with error {error:?}");
            ContractError::IbcRawConversionError { error: err }
        })?;
        Ok(message)
    }

    pub fn raw_from_hex<R: Message + std::default::Default + Clone>(
        hex_str: &HexString,
    ) -> Result<R, ContractError> {
        let bytes = hex_str.to_bytes()?;
        let raw = <R as Message>::decode(bytes.as_slice())
            .map_err(|error| ContractError::IbcDecodeError { error })?;
        Ok(raw)
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

    use crate::context::CwIbcCoreContext;
    use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};

    use crate::msg::MigrateMsg;
    use cw2::{get_contract_version, ContractVersion};
    use cw_common::ibc_types::IbcClientType;

    use super::{instantiate, query, InstantiateMsg, QueryMsg};

    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, OwnedDeps,
    };

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
        let msg = QueryMsg::GetNextClientSequence {};
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
            _type: client_type_str,
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let result_parsed: Addr = from_binary(&result).unwrap();
        assert_eq!(client, result_parsed.as_str());
    }

    #[test]
    fn test_migrate() {
        let mut mock_deps = mock_dependencies();
        let env = mock_env();

        let contract = CwIbcCoreContext::default();
        let result = contract.migrate(mock_deps.as_mut(), env, MigrateMsg { clear_store: false });
        assert!(result.is_ok());
        let expected = ContractVersion {
            contract: CONTRACT_NAME.to_string(),
            version: CONTRACT_VERSION.to_string(),
        };
        let version = get_contract_version(&mock_deps.storage).unwrap();
        println!("{version:?}");
        assert_eq!(expected, version);
    }
}
