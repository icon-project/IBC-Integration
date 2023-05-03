use cw_common::hex_string::HexString;
use common::icon::icon::lightclient::v1::{
    ClientState as RawClientState, ConsensusState as RawConsensusState,
};
use super::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-ibc-core";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwIbcCoreContext<'a> {
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
               
                let client_state=Self::from_raw::<RawClientState,ClientState>(&client_state)?;
                let consensus_state=Self::from_raw::<RawConsensusState,ConsensusState>(&consensus_state)?;

             
                let signer =
                    String::from_utf8(signer.to_bytes().unwrap()).map_err(|error| ContractError::IbcDecodeError {
                        error: error.to_string(),
                    })?;

                let signer =
                    Signer::from_str(&signer).map_err(|error| ContractError::IbcDecodeError {
                        error: error.to_string(),
                    })?;
                let msg = MsgCreateClient {
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
                let header = SignedHeader::try_from(header.to_bytes().unwrap()).map_err(|error| error)?;
                let signer =
                    String::from_utf8(signer.to_bytes().unwrap()).map_err(|error| ContractError::IbcDecodeError {
                        error: error.to_string(),
                    })?;

                let signer =
                    Signer::from_str(&signer).map_err(|error| ContractError::IbcDecodeError {
                        error: error.to_string(),
                    })?;
                let msg = MsgUpdateClient {
                    client_id: IbcClientId::from_str(&client_id).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    header: header.into(),
                    signer,
                };
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
                let signer =
                    String::from_utf8(signer.to_bytes().unwrap()).map_err(|error| ContractError::IbcDecodeError {
                        error: error.to_string(),
                    })?;

                let signer =
                    Signer::from_str(&signer).map_err(|error| ContractError::IbcDecodeError {
                        error: error.to_string(),
                    })?;
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
                let packet_bytes=packet.to_bytes().unwrap();
                let packet: RawPacket = Message::decode(packet_bytes.as_slice()).map_err(|error| {
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

    pub fn calculate_fee(&self, expected_gas: u64) -> u128 {
        let fee = expected_gas as u128 * self.gas_price();

        fee.checked_div(GAS_DENOMINATOR as u128).unwrap()
    }

    pub fn gas_price(&self) -> u128 {
        let price = GAS_NUMERATOR_DEFAULT * GAS_ADJUSTMENT_NUMERATOR_DEFAULT;

        price.checked_div(GAS_DENOMINATOR).unwrap();

        price as u128
    }
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

    pub fn from_raw<R: Message + std::default::Default + Clone, T: TryFrom<R>>(
        hex_str: &HexString,
    ) -> Result<T, ContractError>
    where
        <T as TryFrom<R>>::Error: std::fmt::Debug,
    {
        let bytes=hex_str.to_bytes().map_err(|e|ContractError::IbcDecodeError { error: e.to_string() })?;
        let raw = <R as Message>::decode(bytes.as_slice()).map_err(|error| ContractError::IbcDecodeError {
            error: error.to_string(),
        })?;
        let message = T::try_from(raw).map_err(|error| {
            let err = format!("Failed to convert to ibc type with error {:?}", error);
            ContractError::IbcRawConversionError { error: err }
        })?;
        Ok(message)
    }
}
