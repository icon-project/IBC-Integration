use super::*;
use super::conn1_types::LightClientConnectionMessage;

pub const EXECUTE_CONNECTION_OPENACK: u64 = 32;

impl<'a> CwIbcCoreContext<'a> {
    pub fn connection_open_init(
        &self,
        deps: DepsMut,
        message: MsgConnectionOpenInit,
    ) -> Result<Response, ContractError> {
        let connection_identifier = self.generate_connection_idenfier(deps.storage)?;

        self.client_state(deps.storage, &message.client_id_on_a.clone())?;

        let client_id = ClientId::from(message.client_id_on_a.clone());

        self.check_for_connection(deps.as_ref().storage, client_id.clone())?;

        let versions = match message.version {
            Some(version) => {
                if self.get_compatible_versions().contains(&version) {
                    vec![version]
                } else {
                    return Err(ContractError::IbcConnectionError {
                        error: (ConnectionError::EmptyVersions),
                    });
                }
            }
            None => self.get_compatible_versions(),
        };

        let connection_end = ConnectionEnd::new(
            State::Init,
            message.client_id_on_a,
            message.counterparty.clone(),
            versions,
            message.delay_period,
        );

        self.update_connection_commitment(
            deps.storage,
            connection_identifier.clone(),
            connection_end.clone(),
        )?;
        self.store_connection_to_client(
            deps.storage,
            client_id.clone(),
            connection_identifier.clone(),
        )?;
        self.store_connection(deps.storage, connection_identifier.clone(), connection_end)
            .unwrap();

        let event = create_open_init_event(
            connection_identifier.connection_id().as_str(),
            client_id.as_str(),
            message.counterparty.client_id().as_str(),
        );

        return Ok(Response::new()
            .add_attribute("method", "connection_open_init")
            .add_attribute("connection_id", connection_identifier.as_str())
            .add_event(event));
    }

    pub fn generate_connection_idenfier(
        &self,
        store: &mut dyn Storage,
    ) -> Result<ConnectionId, ContractError> {
        let counter = self.connection_counter(store)?;

        let connection_id = ConnectionId::new(counter);

        self.increase_connection_counter(store)?;

        Ok(connection_id)
    }
    pub fn get_compatible_versions(&self) -> Vec<Version> {
        vec![Version::default()]
    }

    pub fn connection_open_ack(
        &self,
        info: MessageInfo,
        deps: DepsMut,
        msg: MsgConnectionOpenAck,
    ) -> Result<Response, ContractError> {
        let host_height = self
            .host_height()
            .map_err(|_| ContractError::IbcConnectionError {
                error: ConnectionError::Other {
                    description: "failed to get host height".to_string(),
                },
            })?;
        if msg.consensus_height_of_a_on_b > host_height {
            return Err(ContractError::IbcConnectionError {
                error: ConnectionError::InvalidConsensusHeight {
                    target_height: msg.consensus_height_of_a_on_b,
                    current_height: host_height,
                }
                .into(),
            });
        }

        self.validate_self_client(msg.client_state_of_a_on_b.clone())?;
        let conn_end_on_a = self.connection_end(deps.storage, msg.conn_id_on_a.clone().into())?;
        let client_id_on_a = conn_end_on_a.client_id();
        let client_id_on_b = conn_end_on_a.counterparty().client_id();

        if !(conn_end_on_a.state_matches(&State::Init)
            && conn_end_on_a.versions().contains(&msg.version))
        {
            return Err(ContractError::IbcConnectionError {
                error: ConnectionError::ConnectionMismatch {
                    connection_id: msg.conn_id_on_a.clone(),
                },
            });
        }

        let client_cons_state_path_on_a =
            self.consensus_state(deps.storage, client_id_on_a, &msg.proofs_height_on_b)?;
        let consensus_state_of_b_on_a = self
            .consensus_state(deps.storage, client_id_on_a, &msg.proofs_height_on_b)
            .map_err(|_| ContractError::IbcConnectionError {
                error: ConnectionError::Other {
                    description: "failed to fetch consennsus state".to_string(),
                },
            })?;
        let prefix_on_a = self.commitment_prefix();
        let prefix_on_b = conn_end_on_a.counterparty().prefix();
        let client_address =
            self.get_client(deps.as_ref().storage, client_id_on_a.clone().into())?;

        let expected_conn_end_on_b = ConnectionEnd::new(
            State::TryOpen,
            client_id_on_b.clone(),
            Counterparty::new(
                client_id_on_a.clone().into(),
                Some(msg.conn_id_on_a.clone()),
                prefix_on_a,
            ),
            vec![msg.version.clone()],
            conn_end_on_a.delay_period(),
        );
        let connection_path = self.connection_path(&msg.conn_id_on_b);
        let verify_connection_state = VerifyConnectionState::new(
            msg.proofs_height_on_b.to_string(),
            to_vec(&prefix_on_b).map_err(|error| ContractError::Std(error))?,
            to_vec(&msg.proof_conn_end_on_b).map_err(|error| ContractError::Std(error))?,
            consensus_state_of_b_on_a.root().as_bytes().to_vec(),
            connection_path,
            to_vec(&expected_conn_end_on_b).map_err(|error| ContractError::Std(error))?,
        );

        let client_state_path = self.client_state_path(client_id_on_a);
        let verify_client_full_satate = VerifyClientFullState::new(
            msg.proofs_height_on_b.to_string(),
            to_vec(&prefix_on_b).map_err(|error| ContractError::Std(error))?,
            to_vec(&msg.proof_client_state_of_a_on_b).map_err(|error| ContractError::Std(error))?,
            consensus_state_of_b_on_a.root().as_bytes().to_vec(),
            client_state_path,
            to_vec(&msg.client_state_of_a_on_b.clone())
                .map_err(|error| ContractError::Std(error))?,
        );

        let consensus_state_path_on_b =
            self.consensus_state_path(client_id_on_b, &msg.consensus_height_of_a_on_b);
        let vefiry_client_consensus_state = VerifyClientConsesnusState::new(
            msg.proofs_height_on_b.to_string(),
            to_vec(&prefix_on_b).map_err(|error| ContractError::Std(error))?,
            to_vec(&msg.proof_consensus_state_of_a_on_b)
                .map_err(|error| ContractError::Std(error))?,
            consensus_state_of_b_on_a.root().as_bytes().to_vec(),
            consensus_state_path_on_b,
            to_vec(&client_cons_state_path_on_a.clone())
                .map_err(|error| ContractError::Std(error))?,
        );
        let client_message = LightClientConnectionMessage::OpenAck {
            verify_connection_state,
            verify_client_full_satate,
            vefiry_client_consensus_state,
        };

        let wasm_execute_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: client_address.to_string(),
            msg: to_binary(&client_message).unwrap(),
            funds: info.funds,
        });

        let sub_message = SubMsg::reply_always(wasm_execute_message, EXECUTE_CONNECTION_OPENACK);

        Ok(Response::new()
            .add_submessage(sub_message)
            .add_attribute("method", "connection_open_ack"))
    }

    pub fn excute_connection_open_ack(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => match result.data {
                Some(data) => {
                    let response: OpenAckResponse =
                        from_binary(&data).map_err(|error| ContractError::Std(error))?;

                    let connection_id =
                        IbcConnectionId::from_str(&response.conn_id).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: error.to_string(),
                            }
                        })?;

                    let version: Version =
                        serde_json_wasm::from_slice(&response.version).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: error.to_string(),
                            }
                        })?;

                    let mut conn_end =
                        self.connection_end(deps.storage, connection_id.clone().into())?;

                    if !conn_end.state_matches(&State::Init) {
                        return Err(ContractError::IbcConnectionError {
                            error: ConnectionError::ConnectionMismatch {
                                connection_id: connection_id.clone(),
                            },
                        });
                    }
                    let counter_party_client_id =
                        ClientId::from_str(&response.counterparty_client_id).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: error.to_string(),
                            }
                        })?;

                    let counterparty_conn_id = match response.counterparty_connection_id.is_empty()
                    {
                        true => None,
                        false => {
                            let connection_id =
                                IbcConnectionId::from_str(&response.counterparty_connection_id)
                                    .unwrap();
                            Some(connection_id.clone())
                        }
                    };

                    let counterparty_prefix = CommitmentPrefix::try_from(
                        response.counterparty_prefix,
                    )
                    .map_err(|error| ContractError::IbcConnectionError {
                        error: ConnectionError::Other {
                            description: error.to_string(),
                        },
                    })?;

                    let counterparty = Counterparty::new(
                        counter_party_client_id.ibc_client_id().clone(),
                        counterparty_conn_id.clone(),
                        counterparty_prefix,
                    );

                    conn_end.set_state(State::Open);
                    conn_end.set_version(version.clone());
                    conn_end.set_counterparty(counterparty.clone());

                    let counter_conn_id = ConnectionId::from(counterparty_conn_id.unwrap());

                    let event = create_open_ack_event(
                        connection_id.clone().into(),
                        conn_end.client_id().clone().into(),
                        counter_conn_id,
                        counterparty.client_id().clone().into(),
                    );

                    self.store_connection(deps.storage, connection_id.clone().into(), conn_end)
                        .unwrap();

                    Ok(Response::new()
                        .add_attribute("method", "excute_connection_open_try")
                        .add_attribute("connection_id", connection_id.as_str())
                        .add_event(event))
                }
                None => Err(ContractError::IbcConnectionError {
                    error: ConnectionError::Other {
                        description: "UNKNOWN ERROR".to_string(),
                    },
                }),
            },
            cosmwasm_std::SubMsgResult::Err(error) => Err(ContractError::IbcConnectionError {
                error: ConnectionError::Other { description: error },
            }),
        }
    }
}
