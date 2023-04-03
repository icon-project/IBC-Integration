use super::{events::create_client_event, *};

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
        client_state: Vec<u8>,
        consensus_state: Vec<u8>,
    },
}
pub const EXECUTE_CREATE_CLIENT: u64 = 21;

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

        let client_type = ClientType::from(client_state.client_type());

        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type)?;

        let create_client_message = LightClientMessage::CreateClient {
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
        message: ibc::core::ics02_client::msgs::upgrade_client::MsgUpgradeClient,
    ) {
        todo!()
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
}
