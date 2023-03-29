use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    from_binary, to_binary, Attribute, CosmosMsg, Empty, MessageInfo, Reply, Response, SubMsg,
    WasmMsg,
};

use ibc::core::ics02_client::error::ClientError;

use super::*;

#[cw_serde]
pub struct CallBackData {
    client_type: String,
    consensus_height: String,
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
            .map_err(|error| return ContractError::IbcContextError { error })?;

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
        message: ibc::core::ics02_client::msgs::update_client::MsgUpdateClient,
    ) {
        todo!()
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
    ) -> Result<IbcClientId, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => {
                let call_backdata: CallBackData = from_binary(&result.data.unwrap()).unwrap();
                let client_counter = self.client_counter(deps.as_ref().storage)?;

                let client_type = ClientType::new(call_backdata.client_type);

                let client_id = ClientId::new(client_type, client_counter)?;

                Ok(client_id.ibc_client_id().clone())
            }
            cosmwasm_std::SubMsgResult::Err(error) => Err(ContractError::IbcClientError {
                error: ClientError::Other { description: error },
            }),
        }
    }
}
