use crate::traits::ExecuteChannel;

use super::*;

// version info for migration info
#[allow(dead_code)]
const CONTRACT_NAME: &str = "crates.io:cw-ibc-core";
#[allow(dead_code)]
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(unused_variables)]
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

        Ok(Response::new().add_attribute("method", "instantiate"))
    }

    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::RegisterClient {
                client_type,
                client_address,
            } => {
                let client_type = ClientType::new(client_type);
                self.register_client(deps, client_type, client_address)
            }
            ExecuteMsg::CreateClient {
                client_state,
                consensus_state,
                signer,
            } => {
                let msg = MsgCreateClient {
                    client_state,
                    consensus_state,
                    signer,
                };
                self.create_client(deps, info, msg)
            }
            ExecuteMsg::UpdateClient {
                client_id,
                header,
                signer,
            } => {
                let msg = MsgUpdateClient {
                    client_id: IbcClientId::from_str(&client_id).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    header,
                    signer,
                };
                self.update_client(deps, info, msg)
            }
            ExecuteMsg::UpgradeClient {} => todo!(),
            ExecuteMsg::ConnectionOpenInit {
                client_id_on_a,
                counterparty,
                version,
                delay_period,
                signer,
            } => {
                let msg = MsgConnectionOpenInit {
                    client_id_on_a: IbcClientId::from_str(&client_id_on_a).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    counterparty,
                    version,
                    delay_period,
                    signer,
                };

                self.connection_open_init(deps, msg)
            }
            ExecuteMsg::ConnectionOpenTry { msg } => {
                let message : ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry= msg.try_into().unwrap();

                self.connection_open_try(deps, info, message)
            }
            ExecuteMsg::ConnectionOpenAck {
                conn_id_on_a,
                conn_id_on_b,
                client_state_of_a_on_b,
                proof_conn_end_on_b,
                proof_client_state_of_a_on_b,
                proof_consensus_state_of_a_on_b,
                proofs_height_on_b,
                consensus_height_of_a_on_b,
                version,
                signer,
            } => {
                let message = MsgConnectionOpenAck {
                    conn_id_on_a: IbcConnectionId::from_str(conn_id_on_a.as_str()).map_err(
                        |error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        },
                    )?,
                    conn_id_on_b: IbcConnectionId::from_str(conn_id_on_b.as_str()).map_err(
                        |error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        },
                    )?,
                    client_state_of_a_on_b,
                    proof_conn_end_on_b: CommitmentProofBytes::try_from(proof_conn_end_on_b)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        })?,
                    proof_client_state_of_a_on_b: CommitmentProofBytes::try_from(
                        proof_client_state_of_a_on_b,
                    )
                    .map_err(|error| ContractError::IbcDecodeError {
                        error: error.to_string(),
                    })?,
                    proof_consensus_state_of_a_on_b: CommitmentProofBytes::try_from(
                        proof_consensus_state_of_a_on_b,
                    )
                    .map_err(|error| ContractError::IbcDecodeError {
                        error: error.to_string(),
                    })?,
                    proofs_height_on_b,
                    consensus_height_of_a_on_b,
                    version,
                    signer,
                };

                self.connection_open_ack(deps, info, message)
            }
            ExecuteMsg::ConnectionOpenConfirm {
                conn_id_on_b,
                proof_conn_end_on_a,
                proof_height_on_a,
                signer,
            } => {
                let message = MsgConnectionOpenConfirm {
                    conn_id_on_b: IbcConnectionId::from_str(conn_id_on_b.as_str()).map_err(
                        |error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        },
                    )?,
                    proof_conn_end_on_a: CommitmentProofBytes::try_from(proof_conn_end_on_a)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        })?,
                    proof_height_on_a,
                    signer,
                };
                self.connection_open_confirm(deps, info, message)
            }
            ExecuteMsg::ChannelOpenInit {
                port_id_on_a,
                connection_hops_on_a,
                port_id_on_b,
                ordering,
                signer,
                version_proposal,
            } => {
                let message = MsgChannelOpenInit {
                    port_id_on_a: IbcPortId::from_str(&port_id_on_a).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    connection_hops_on_a,
                    port_id_on_b: IbcPortId::from_str(&port_id_on_b).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    ordering,
                    signer,
                    version_proposal,
                };
                self.validate_channel_open_init(deps, info, &message)
            }
            ExecuteMsg::ChannelOpenTry { msg } => {
                let message: MsgChannelOpenTry = MsgChannelOpenTry::try_from(msg)
                    .map_err(|error| ContractError::IbcChannelError { error })?;
                self.validate_channel_open_try(deps, info, &message)
            }
            ExecuteMsg::ChannelOpenAck {
                port_id_on_a,
                chan_id_on_a,
                chan_id_on_b,
                version_on_b,
                proof_chan_end_on_b,
                proof_height_on_b,
                signer,
            } => {
                let message = MsgChannelOpenAck {
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
                    chan_id_on_b: IbcChannelId::from_str(&chan_id_on_b).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    version_on_b,
                    proof_chan_end_on_b: CommitmentProofBytes::try_from(proof_chan_end_on_b)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        })?,
                    proof_height_on_b,
                    signer,
                };
                self.validate_channel_open_ack(deps, info, &message)
            }
            ExecuteMsg::ChannelOpenConfirm {
                port_id_on_b,
                chan_id_on_b,
                proof_chan_end_on_a,
                proof_height_on_a,
                signer,
            } => {
                let message = MsgChannelOpenConfirm {
                    port_id_on_b: IbcPortId::from_str(&port_id_on_b).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    chan_id_on_b: IbcChannelId::from_str(&chan_id_on_b).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    proof_chan_end_on_a: CommitmentProofBytes::try_from(proof_chan_end_on_a)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        })?,
                    proof_height_on_a,
                    signer,
                };
                self.validate_channel_open_confirm(deps, info, &message)
            }
            ExecuteMsg::ChannelCloseInit {
                port_id_on_a,
                chan_id_on_a,
                signer,
            } => {
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
            ExecuteMsg::ChannelCloseConfirm {
                port_id_on_b,
                chan_id_on_b,
                proof_chan_end_on_a,
                proof_height_on_a,
                signer,
            } => {
                let message = MsgChannelCloseConfirm {
                    port_id_on_b: IbcPortId::from_str(&port_id_on_b).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    chan_id_on_b: IbcChannelId::from_str(&chan_id_on_b).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    proof_chan_end_on_a: CommitmentProofBytes::try_from(proof_chan_end_on_a)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        })?,
                    proof_height_on_a,
                    signer,
                };

                self.validate_channel_close_confirm(deps, info, &message)
            }
            ExecuteMsg::SendPacket { packet } => self.send_packet(deps, packet),
            ExecuteMsg::ReceivePacket {
                packet,
                proof_commitment_on_a,
                proof_height_on_a,
                signer,
            } => {
                let message = MsgRecvPacket {
                    packet,
                    proof_commitment_on_a: CommitmentProofBytes::try_from(proof_commitment_on_a)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        })?,
                    proof_height_on_a,
                    signer,
                };

                self.validate_receive_packet(deps, info, &message)
            }
            ExecuteMsg::AcknowledgementPacket {
                packet,
                acknowledgement,
                proof_acked_on_b,
                proof_height_on_b,
                signer,
            } => {
                let message = MsgAcknowledgement {
                    packet,
                    acknowledgement,
                    proof_acked_on_b: CommitmentProofBytes::try_from(proof_acked_on_b).map_err(
                        |error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        },
                    )?,
                    proof_height_on_b,
                    signer,
                };

                self.acknowledgement_packet_validate(deps, info, &message)
            }
            ExecuteMsg::RequestTimeout {} => todo!(),
            ExecuteMsg::Timeout {
                packet,
                next_seq_recv_on_b,
                proof_unreceived_on_b,
                proof_height_on_b,
                signer,
            } => {
                let message = MsgTimeout {
                    packet,
                    next_seq_recv_on_b: next_seq_recv_on_b.into(),
                    proof_unreceived_on_b: CommitmentProofBytes::try_from(proof_unreceived_on_b)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        })?,
                    proof_height_on_b,
                    signer,
                };

                self.timeout_packet_validate(
                    deps,
                    info,
                    cw_common::types::TimeoutMsgType::Timeout(message),
                )
            }
            ExecuteMsg::TimeoutOnClose {
                packet,
                next_seq_recv_on_b,
                proof_unreceived_on_b,
                proof_close_on_b,
                proof_height_on_b,
                signer,
            } => {
                let message = MsgTimeoutOnClose {
                    packet,
                    next_seq_recv_on_b: next_seq_recv_on_b.into(),
                    proof_unreceived_on_b: CommitmentProofBytes::try_from(proof_unreceived_on_b)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        })?,
                    proof_close_on_b: CommitmentProofBytes::try_from(proof_close_on_b).map_err(
                        |error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        },
                    )?,
                    proof_height_on_b,
                    signer,
                };
                self.timeout_packet_validate(
                    deps,
                    info,
                    cw_common::types::TimeoutMsgType::TimeoutOnClose(message),
                )
            }
            ExecuteMsg::BindPort { port_id, address } => {
                let port_id = IbcPortId::from_str(&port_id).map_err(|error| {
                    ContractError::IbcDecodeError {
                        error: error.to_string(),
                    }
                })?;
                self.bind_port(deps.storage, &port_id, address)
            }
            ExecuteMsg::SetExpectedTimePerBlock { block_time } => {
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
        env: Env,
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
                // self.execute_open_try_from_light_client(deps, info, message)
                todo!()
            }
            EXECUTE_ON_CHANNEL_OPEN_ACK_ON_LIGHT_CLIENT => {
                // self.execute_open_ack_from_light_client_reply(deps, info, message)
                todo!()
            }
            EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE => self.execute_channel_open_ack(deps, message),
            EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_LIGHT_CLIENT => {
                // self.execute_open_confirm_from_light_client_reply(deps, info, message)
                todo!()
            }
            EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE => {
                self.execute_channel_open_confirm(deps, message)
            }
            EXECUTE_ON_CHANNEL_CLOSE_INIT => self.execute_channel_close_init(deps, message),
            EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_LIGHT_CLIENT => {
                // self.execute_close_confirm_from_light_client_reply(deps, info, message)
                todo!()
            }
            EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE => {
                self.execute_channel_close_confirm(deps, message)
            }

            VALIDATE_ON_PACKET_TIMEOUT_ON_LIGHT_CLIENT => {
                // self.timeout_packet_validate_reply_from_light_client(deps, info, message)
                todo!()
            }
            VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE => self.execute_timeout_packet(deps, message),
            VALIDATE_ON_PACKET_RECEIVE_ON_LIGHT_CLIENT => {
                // self.receive_packet_validate_reply_from_light_client(deps, info, message)
                todo!()
            }
            VALIDATE_ON_PACKET_RECEIVE_ON_MODULE => self.execute_receive_packet(deps, message),
            VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_LIGHT_CLIENT => {
                todo!()
                //self.acknowledgement_packet_validate_reply_from_light_client(deps, info, message)
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
}
