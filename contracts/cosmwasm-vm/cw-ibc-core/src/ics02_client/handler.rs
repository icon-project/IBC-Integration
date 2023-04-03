use super::{
    events::{create_client_event, update_client_event, upgrade_client_event},
    *,
};
use cosmwasm_std::to_vec;

#[cw_serde]
pub struct CreateClientResponse {
    client_type: String,
    height: String,
    client_state_commitment: Vec<u8>,
    consensus_state_commitment: Vec<u8>,
}

impl CreateClientResponse {
    pub fn new(
        client_type: String,
        height: String,
        client_state_commitment: Vec<u8>,
        consensus_state_commitment: Vec<u8>,
    ) -> Self {
        Self {
            client_type,
            height,
            client_state_commitment,
            consensus_state_commitment,
        }
    }
    pub fn client_type(&self) -> ClientType {
        ClientType::new(self.client_type.to_owned())
    }

    pub fn height(&self) -> Height {
        Height::from_str(&self.height).unwrap()
    }

    pub fn client_state_commitment(&self) -> &[u8] {
        &self.client_state_commitment
    }
    pub fn consensus_state_commitment(&self) -> &[u8] {
        &self.consensus_state_commitment
    }
}

#[cw_serde]
pub enum LightClientMessage {
    CreateClient {
        client_id: String,
        client_state: Vec<u8>,
        consensus_state: Vec<u8>,
    },
    UpdateClient {
        client_id: String,
        header: Vec<u8>,
    },
    UpgradeClient {
        upgraded_client_state: Vec<u8>,
        upgraded_consensus_state: Vec<u8>,
        proof_upgrade_client: Vec<u8>,
        proof_upgrade_consensus_state: Vec<u8>,
    },
}
pub const EXECUTE_CREATE_CLIENT: u64 = 21;
pub const EXECUTE_UPDATE_CLIENT: u64 = 22;
pub const EXECUT_UPGRADE_CLIENT: u64 = 23;

#[cw_serde]
pub struct UpdateClientResponse {
    height: String,
    client_id: String,
    client_state_commitment: Vec<u8>,
    consensus_state_commitment: Vec<u8>,
}

impl UpdateClientResponse {
    pub fn new(
        height: String,
        client_id: String,
        client_state_commitment: Vec<u8>,
        consensus_state_commitment: Vec<u8>,
    ) -> Self {
        Self {
            height,
            client_id,
            client_state_commitment,
            consensus_state_commitment,
        }
    }
    pub fn height(&self) -> Height {
        Height::from_str(&self.height).unwrap()
    }

    pub fn client_state_commitment(&self) -> &[u8] {
        &self.client_state_commitment
    }
    pub fn consensus_state_commitment(&self) -> &[u8] {
        &self.consensus_state_commitment
    }
    pub fn client_id(&self) -> Result<ClientId, ContractError> {
        ClientId::from_str(&self.client_id).map_err(|error| ContractError::IbcDecodeError {
            error: error.to_string(),
        })
    }
}
#[cw_serde]
pub struct UpgradeClientResponse {
    client_id: String,
    height: String,
    client_state_commitment: Vec<u8>,
    consesnus_state_commitment: Vec<u8>,
}

impl UpgradeClientResponse {
    pub fn new(
        client_state_commitment: Vec<u8>,
        consesnus_state_commitment: Vec<u8>,
        client_id: String,
        height: String,
    ) -> Self {
        {
            Self {
                height,
                client_id,
                client_state_commitment,
                consesnus_state_commitment,
            }
        }
    }

    pub fn client_id(&self) -> ClientId {
        ClientId::from_str(&self.client_id).unwrap()
    }

    pub fn client_state_commitment(&self) -> &[u8] {
        &self.client_state_commitment
    }
    pub fn consesnus_state_commitment(&self) -> &[u8] {
        &self.consesnus_state_commitment
    }
    pub fn height(&self) -> Height {
        Height::from_str(&self.height).unwrap()
    }
}

impl<'a> IbcClient for CwIbcCoreContext<'a> {
    fn create_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: MsgCreateClient,
    ) -> Result<Response, ContractError> {
        let client_state = self
            .decode_client_state(message.client_state.clone())
            .map_err(|error| return error)?;
        let client_counter = self.client_counter(deps.as_ref().storage)?;

        let client_type = ClientType::from(client_state.client_type());

        let client_id = ClientId::new(client_type.clone(), client_counter)?;

        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type)?;

        let create_client_message = LightClientMessage::CreateClient {
            client_id: client_id.ibc_client_id().to_string(),
            client_state: message.client_state.value,
            consensus_state: message.consensus_state.value,
        };

        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&create_client_message).unwrap(),
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_always(create_client_message, EXECUTE_CREATE_CLIENT);

        Ok(Response::new()
            .add_submessage(sub_msg)
            .add_attribute("method", "create_client"))
    }

    fn update_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: MsgUpdateClient,
    ) -> Result<Response, ContractError> {
        let client_id = ClientId::from(message.client_id);

        let light_client_address =
            self.get_client_implementations(deps.as_ref().storage, client_id.clone())?;

        let message = LightClientMessage::UpdateClient {
            client_id: client_id.as_str().to_string().clone(),
            header: message.header.value,
        };

        let client_update_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&message).unwrap(),
            funds: info.funds,
        });
        let sub_msg: SubMsg = SubMsg::reply_always(client_update_message, EXECUTE_UPDATE_CLIENT);
        Ok(Response::new()
            .add_submessage(sub_msg)
            .add_attribute("method", "update_client")
            .add_attribute("client_id", client_id.as_str()))
    }

    fn upgrade_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: MsgUpgradeClient,
    ) -> Result<Response, ContractError> {
        let old_client_state = self.client_state(deps.as_ref().storage, &message.client_id)?;

        //Check Client Frozen
        if old_client_state.is_frozen() {
            return Err(ContractError::IbcClientError {
                error: ClientError::ClientFrozen {
                    client_id: message.client_id,
                },
            });
        }

        let old_consensus_state = self.consensus_state(
            deps.as_ref().storage,
            &message.client_id,
            &old_client_state.latest_height(),
        )?;

        let now = self.host_timestamp(deps.as_ref().storage)?;
        let duration = now
            .duration_since(&old_consensus_state.timestamp())
            .ok_or_else(|| ClientError::InvalidConsensusStateTimestamp {
                time1: old_consensus_state.timestamp(),
                time2: now,
            })
            .map_err(|error| ContractError::IbcClientError { error })?;

        // Check if the latest consensus state is within the trust period.
        if old_client_state.expired(duration) {
            return Err(ContractError::IbcClientError {
                error: ClientError::HeaderNotWithinTrustPeriod {
                    latest_time: old_consensus_state.timestamp(),
                    update_time: now,
                },
            });
        };

        // Validate the upgraded client state and consensus state and verify proofs against the root

        let wasm_exec_message = LightClientMessage::UpgradeClient {
            upgraded_client_state: message.client_state.value,
            upgraded_consensus_state: message.consensus_state.value,
            proof_upgrade_client: to_vec(&message.proof_upgrade_client).unwrap(),
            proof_upgrade_consensus_state: to_vec(&message.proof_upgrade_consensus_state).unwrap(),
        };

        let client_id = ClientId::from(message.client_id);

        let address = self.get_client_implementations(deps.storage, client_id.clone())?;

        let wasm_msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: address,
            msg: to_binary(&wasm_exec_message).unwrap(),
            funds: info.funds,
        });

        let sub_message = SubMsg::reply_always(wasm_msg, EXECUT_UPGRADE_CLIENT);

        Ok(Response::new()
            .add_submessage(sub_message)
            .add_attribute("method", "upgrade_client")
            .add_attribute("client_id", client_id.ibc_client_id().as_str()))
    }

    fn register_client(&self, deps: DepsMut, client_type: ClientType, light_client: Addr) {
        let light_client_address = light_client.to_string();

        self.check_client_registered(deps.as_ref().storage, client_type.clone())
            .unwrap();

        self.store_client_into_registry(deps.storage, client_type, light_client_address)
            .unwrap();
    }

    fn generate_client_identifier(
        &self,
        deps: DepsMut,
        client_type: ClientType,
    ) -> Result<ClientId, ContractError> {
        let client_seqence = self.client_counter(deps.as_ref().storage)?;
        let client_identifer = ClientId::new(client_type, client_seqence.try_into().unwrap())?;
        self.increase_client_counter(deps.storage)?;
        Ok(client_identifer)
    }

    fn execute_create_client_reply(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => {
                let call_backdata: CreateClientResponse =
                    from_binary(&result.data.unwrap()).unwrap();
                let client_counter = self.client_counter(deps.as_ref().storage)?;
                let client_type = ClientType::new(call_backdata.client_type.clone());
                let client_id = ClientId::new(client_type.clone(), client_counter)?;
                let light_client_address =
                    self.get_client_from_registry(deps.as_ref().storage, client_type.clone())?;

                self.store_client_type(deps.storage, client_id.clone(), client_type.clone())?;

                self.store_client_implementations(
                    deps.storage,
                    client_id.clone(),
                    light_client_address,
                )?;

                self.store_client_state(
                    deps.storage,
                    client_id.ibc_client_id(),
                    call_backdata.client_state_commitment.clone(),
                )?;

                self.store_consensus_state(
                    deps.storage,
                    client_id.ibc_client_id(),
                    call_backdata.height(),
                    call_backdata.consensus_state_commitment.clone(),
                )?;

                self.increase_client_counter(deps.storage)?;

                let event = create_client_event(
                    client_id.ibc_client_id().as_str(),
                    &client_type.client_type().as_str(),
                    &call_backdata.height().to_string(),
                );

                Ok(Response::new()
                    .add_event(event)
                    .add_attribute("method", "execute_create_client_reply")
                    .add_attribute("client_id", client_id.ibc_client_id().to_string()))
            }
            cosmwasm_std::SubMsgResult::Err(error) => Err(ContractError::IbcClientError {
                error: ClientError::Other { description: error },
            }),
        }
    }
    fn execute_update_client_reply(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => match result.data {
                Some(data) => {
                    let update_client_response: UpdateClientResponse =
                        from_binary(&data).map_err(|error| ContractError::Std(error))?;

                    let client_id = update_client_response.client_id()?;

                    let height = update_client_response.height();

                    self.store_client_state(
                        deps.storage,
                        client_id.ibc_client_id(),
                        update_client_response.client_state_commitment().to_vec(),
                    )?;

                    self.store_consensus_state(
                        deps.storage,
                        client_id.ibc_client_id(),
                        height,
                        update_client_response.consensus_state_commitment().to_vec(),
                    )?;

                    let client_type = ClientType::from(client_id.clone());

                    let event = update_client_event(
                        client_type.client_type(),
                        height,
                        vec![height],
                        client_id.ibc_client_id(),
                    );

                    Ok(Response::new()
                        .add_event(event)
                        .add_attribute("methods", "execute_update_client_reply")
                        .add_attribute("height", height))
                }
                None => Err(ContractError::IbcClientError {
                    error: ClientError::Other {
                        description: "UNKNOWN ERROR".to_string(),
                    },
                }),
            },
            cosmwasm_std::SubMsgResult::Err(error) => Err(ContractError::IbcClientError {
                error: ClientError::Other { description: error },
            }),
        }
    }
    fn execute_upgrade_client_reply(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => match result.data {
                Some(data) => {
                    let response: UpgradeClientResponse =
                        from_binary(&data).map_err(|error| ContractError::Std(error))?;
                    let client_id = ClientId::from_str(&response.client_id).map_err(|error| {
                        ContractError::IbcClientError {
                            error: ClientError::InvalidClientIdentifier(error),
                        }
                    })?;

                    self.store_client_state(
                        deps.storage,
                        client_id.ibc_client_id(),
                        response.client_state_commitment.clone(),
                    )?;

                    self.store_consensus_state(
                        deps.storage,
                        client_id.ibc_client_id(),
                        response.height(),
                        response.consesnus_state_commitment.clone(),
                    )?;

                    let client_type = ClientType::from(client_id.clone());

                    let event = upgrade_client_event(
                        client_type.client_type(),
                        response.height(),
                        client_id.ibc_client_id().clone(),
                    );

                    Ok(Response::new()
                        .add_event(event)
                        .add_attribute("method", "execute_upgrade_client_reply")
                        .add_attribute("client_id", client_id.ibc_client_id().as_str()))
                }
                None => Err(ContractError::IbcClientError {
                    error: ClientError::Other {
                        description: "UNKNOWN ERROR".to_string(),
                    },
                }),
            },
            cosmwasm_std::SubMsgResult::Err(error) => Err(ContractError::IbcClientError {
                error: ClientError::Other { description: error },
            }),
        }
    }
}
