use super::{
    conn_types::{LightClientConnectionMessage, VerifyClientConsesnusState, VerifyConnectionState},
    *,
};

pub const EXECUTE_CONNECTION_OPENTRY: u64 = 31;

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
        self.store_connection(
            deps.storage,
            connection_identifier.clone(),
            connection_end.clone(),
        )
        .unwrap();

        let event = create_open_init_event(
            connection_identifier.connection_id().as_str(),
            client_id.as_str(),
            message.counterparty.client_id().as_str(),
        );
        let counter = match self.increase_connection_counter(deps.storage) {
            Ok(counter) => counter,
            Err(error) => return Err(error),
        };
        self.store_connection_to_client(deps.storage, client_id, connection_identifier.clone())?;
        self.store_connection(deps.storage, connection_identifier.clone(), connection_end)
            .unwrap();
        return Ok(Response::new().add_event(event));
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

    pub fn connection_open_try(
        &self,
        message: MsgConnectionOpenTry,
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        //TODO validate

        self.validate_self_client(message.client_state_of_b_on_a.clone())?;
        let host_height = self
            .host_height()
            .map_err(|_| ContractError::IbcConnectionError {
                error: ConnectionError::Other {
                    description: "failed to get host height".to_string(),
                },
            })?;
        if message.consensus_height_of_b_on_a > host_height {
            return Err(ContractError::IbcConnectionError {
                error: ConnectionError::InvalidConsensusHeight {
                    target_height: message.consensus_height_of_b_on_a,
                    current_height: host_height,
                }
                .into(),
            });
        }
        let prefix_on_a = message.counterparty.clone().prefix().clone();
        let prefix_on_b = self.commitment_prefix();
        let client_id_on_b = ClientId::from(message.client_id_on_b.clone());

        let client_address = self.get_client(deps.as_ref().storage, client_id_on_b)?;

        let client_consensus_state_path_on_b =
            self.consensus_state_path(&message.client_id_on_b, &message.consensus_height_of_b_on_a);
        let expected_conn_end_on_a = ConnectionEnd::new(
            State::Init,
            message.counterparty.client_id().clone(),
            Counterparty::new(message.client_id_on_b.clone(), None, prefix_on_b),
            message.versions_on_a.clone(),
            message.delay_period,
        );

        let consensus_state_of_a_on_b = self
            .consensus_state(
                deps.storage,
                &message.client_id_on_b,
                &message.proofs_height_on_a,
            )
            .map_err(|_| ContractError::IbcConnectionError {
                error: ConnectionError::Other {
                    description: "failed to fetch consensus state".to_string(),
                },
            })?;

        let connection_path = self.connection_path(&message.counterparty.connection_id.unwrap());
        let verify_connection_state = VerifyConnectionState::new(
            message.proofs_height_on_a.to_string(),
            to_vec(&prefix_on_a).map_err(|error| ContractError::Std(error))?,
            to_vec(&message.proof_conn_end_on_a).map_err(|error| ContractError::Std(error))?,
            consensus_state_of_a_on_b.root().as_bytes().to_vec(),
            connection_path,
            to_vec(&expected_conn_end_on_a).map_err(|error| ContractError::Std(error))?,
        );

        let client_state_path = self.client_state_path(&message.client_id_on_b);
        let verify_client_full_satate = VerifyClientFullState::new(
            message.proofs_height_on_a.to_string(),
            to_vec(&prefix_on_a.clone()).map_err(|error| ContractError::Std(error))?,
            to_vec(&message.proof_client_state_of_b_on_a)
                .map_err(|error| ContractError::Std(error))?,
            consensus_state_of_a_on_b.root().as_bytes().to_vec(),
            client_state_path,
            to_vec(&message.client_state_of_b_on_a).map_err(|error| ContractError::Std(error))?,
        );
        let conensus_state_path_on_a =
            self.consensus_state_path(&message.client_id_on_b, &message.consensus_height_of_b_on_a);
        let vefiry_client_consensus_state = VerifyClientConsesnusState::new(
            message.proofs_height_on_a.to_string(),
            to_vec(&prefix_on_a.clone()).map_err(|error| ContractError::Std(error))?,
            to_vec(&message.proof_consensus_state_of_b_on_a)
                .map_err(|error| ContractError::Std(error))?,
            consensus_state_of_a_on_b.root().as_bytes().to_vec(),
            conensus_state_path_on_a,
            client_consensus_state_path_on_b,
        );
        let client_message = LightClientConnectionMessage::OpenTry {
            verify_connection_state,
            verify_client_full_satate,
            vefiry_client_consensus_state,
        };

        let wasm_execute_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: client_address,
            msg: to_binary(&client_message).unwrap(),
            funds: info.funds,
        });

        let sub_message = SubMsg::reply_always(wasm_execute_message, EXECUTE_CONNECTION_OPENTRY);

        Ok(Response::new()
            .add_submessage(sub_message)
            .add_attribute("method", "connection_open_try"))
    }

    pub fn excute_connection_open_try(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => match result.data {
                Some(data) => {
                    let response: OpenTryResponse =
                        from_binary(&data).map_err(|error| ContractError::Std(error))?;

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
                                ConnectionId::from_str(&response.counterparty_connection_id)?;
                            Some(connection_id.connection_id().clone())
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
                        counterparty_conn_id,
                        counterparty_prefix,
                    );

                    let version: Version = serde_json_wasm::from_slice(&response.versions)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        })?;

                    let delay_period = Duration::from_secs(response.delay_period);

                    let client_id = ClientId::from_str(&response.client_id).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?;

                    let connection_id = self.generate_connection_idenfier(deps.storage)?;

                    let conn_end = ConnectionEnd::new(
                        State::TryOpen,
                        client_id.ibc_client_id().clone(),
                        counterparty,
                        vec![version],
                        delay_period,
                    );

                    let counterparty_client_id =
                        ClientId::from_str(&response.client_id).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: error.to_string(),
                            }
                        })?;
                    let counterparty_conn_id = ConnectionId::from_str(&response.conn_id).unwrap();
                    let event = create_open_try_event(
                        connection_id.clone(),
                        client_id.clone(),
                        counterparty_conn_id,
                        counterparty_client_id,
                    );
                    self.store_connection_to_client(
                        deps.storage,
                        client_id,
                        connection_id.clone(),
                    )?;
                    self.store_connection(deps.storage, connection_id.clone(), conn_end)
                        .unwrap();

                    Ok(Response::new()
                        .add_attribute("method", "excute_connection_open_try")
                        .add_attribute("connection_id", connection_id.connection_id().as_str())
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
#[cw_serde]
pub struct OpenTryResponse {
    conn_id: String,
    client_id: String,
    counterparty_client_id: String,
    counterparty_connection_id: String,
    counterparty_prefix: Vec<u8>,
    versions: Vec<u8>,
    delay_period: u64,
}

impl OpenTryResponse {
    pub fn new(
        conn_id: String,
        client_id: String,
        counterparty_client_id: String,
        counterparty_connection_id: String,
        counterparty_prefix: Vec<u8>,
        versions: Vec<u8>,
        delay_period: u64,
    ) -> Self {
        Self {
            conn_id,
            client_id,
            counterparty_client_id,
            counterparty_connection_id,
            counterparty_prefix,
            versions,
            delay_period,
        }
    }
}
