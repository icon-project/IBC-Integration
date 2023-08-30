use crate::{ContractError, EXECUTE_UPDATE_CLIENT};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Binary, CosmosMsg, Deps, SubMsg};
use cw_common::client_msg::{LightClientPacketMessage, VerifyConnectionState};

use cw_common::ibc_types::IbcClientId;
use cw_common::raw_types::Any;
use cw_common::types::{VerifyChannelState, VerifyPacketAcknowledgement, VerifyPacketData};
use cw_common::{client_msg::VerifyConnectionPayload, query_helpers::build_smart_query};
use prost::Message;

#[cw_serde]
pub struct LightClient {
    address: String,
}

impl LightClient {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn update_client(
        &self,
        client_id: &IbcClientId,
        header: &Any,
    ) -> Result<SubMsg, ContractError> {
        let exec_message = cw_common::client_msg::ExecuteMsg::UpdateClient {
            client_id: client_id.as_str().to_string(),
            signed_header: header.encode_to_vec(),
        };
        let client_update_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: self.address.clone(),
            msg: to_binary(&exec_message).map_err(ContractError::Std)?,
            funds: vec![],
        });
        let sub_msg: SubMsg =
            SubMsg::reply_on_success(client_update_message, EXECUTE_UPDATE_CLIENT);
        Ok(sub_msg)
    }

    pub fn verify_connection_open_ack(
        &self,
        deps: Deps,
        payload: VerifyConnectionPayload,
    ) -> Result<(), ContractError> {
        let msg = to_binary(&cw_common::client_msg::QueryMsg::VerifyConnectionOpenAck(
            payload,
        ))
        .map_err(ContractError::Std)?;
        let query = build_smart_query(self.address.clone(), msg);
        let result: bool = deps.querier.query(&query).unwrap_or(false);
        self.to_validation_result(result, "verify connection open ack")
    }

    pub fn verify_connection_open_try(
        &self,
        deps: Deps,
        payload: VerifyConnectionPayload,
    ) -> Result<(), ContractError> {
        let msg = to_binary(&cw_common::client_msg::QueryMsg::VerifyConnectionOpenTry(
            payload,
        ))
        .map_err(ContractError::Std)?;
        let query = build_smart_query(self.address.clone(), msg);
        let result: bool = deps.querier.query(&query).unwrap_or(false);
        self.to_validation_result(result, "verify connection open try")
    }

    pub fn verify_connection_open_confirm(
        &self,
        deps: Deps,
        verify_connection_state: VerifyConnectionState,
        client_id: &IbcClientId,
    ) -> Result<(), ContractError> {
        let msg = to_binary(&cw_common::client_msg::QueryMsg::VerifyOpenConfirm {
            client_id: client_id.to_string(),
            verify_connection_state,
        })
        .map_err(ContractError::Std)?;
        let query = build_smart_query(self.address.clone(), msg);
        let result: bool = deps.querier.query(&query).unwrap_or(false);
        self.to_validation_result(result, "verify connection open confirm")
    }

    pub fn verify_packet_acknowledge(
        &self,
        deps: Deps,
        verify_packet_acknowledge: VerifyPacketAcknowledgement,
        client_id: &IbcClientId,
    ) -> Result<(), ContractError> {
        let msg = to_binary(
            &cw_common::client_msg::QueryMsg::VerifyPacketAcknowledgement {
                client_id: client_id.to_string(),
                verify_packet_acknowledge,
            },
        )
        .map_err(ContractError::Std)?;
        let query = build_smart_query(self.address.clone(), msg);
        let result: bool = deps.querier.query(&query).unwrap_or(false);
        self.to_validation_result(result, "verify packet ack")
    }

    pub fn verify_packet_data(
        &self,
        deps: Deps,
        verify_packet_data: VerifyPacketData,
        client_id: &IbcClientId,
    ) -> Result<(), ContractError> {
        let msg = to_binary(&cw_common::client_msg::QueryMsg::VerifyPacketData {
            client_id: client_id.to_string(),
            verify_packet_data,
        })
        .map_err(ContractError::Std)?;
        let query = build_smart_query(self.address.clone(), msg);
        let result: bool = deps.querier.query(&query).unwrap_or(false);
        self.to_validation_result(result, "verify packet data")
    }

    pub fn verify_timeout_on_close(
        &self,
        deps: Deps,
        client_id: &IbcClientId,
        verify_channel_state: VerifyChannelState,
        next_seq_recv_verification_result: LightClientPacketMessage,
    ) -> Result<(), ContractError> {
        let msg = to_binary(&cw_common::client_msg::QueryMsg::TimeoutOnCLose {
            client_id: client_id.to_string(),
            verify_channel_state,
            next_seq_recv_verification_result,
        })
        .map_err(ContractError::Std)?;
        let query = build_smart_query(self.address.clone(), msg);
        let result: bool = deps.querier.query(&query).unwrap_or(false);
        self.to_validation_result(result, "verify timeout on close")
    }

    pub fn verify_timeout(
        &self,
        deps: Deps,
        client_id: &IbcClientId,

        next_seq_recv_verification_result: LightClientPacketMessage,
    ) -> Result<(), ContractError> {
        let msg = to_binary(&cw_common::client_msg::QueryMsg::PacketTimeout {
            client_id: client_id.to_string(),
            next_seq_recv_verification_result,
        })
        .map_err(ContractError::Std)?;
        let query = build_smart_query(self.address.clone(), msg);
        let result: bool = deps.querier.query(&query).unwrap_or(false);
        self.to_validation_result(result, "verify timeout on close")
    }

    pub fn verify_channel(
        &self,
        deps: Deps,

        verify_channel_state: VerifyChannelState,
    ) -> Result<(), ContractError> {
        let msg = to_binary(&cw_common::client_msg::QueryMsg::VerifyChannel {
            verify_channel_state,
        })
        .map_err(ContractError::Std)?;
        let query = build_smart_query(self.address.clone(), msg);
        let result: bool = deps.querier.query(&query).unwrap_or(false);
        self.to_validation_result(result, "verify channel state")
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn get_client_state(
        &self,
        deps: Deps,
        client_id: &IbcClientId,
    ) -> Result<Vec<u8>, ContractError> {
        let query_message = cw_common::client_msg::QueryMsg::GetClientState {
            client_id: client_id.as_str().to_string(),
        };
        let msg = to_binary(&query_message).map_err(ContractError::Std)?;

        let query = build_smart_query(self.address.clone(), msg);

        let response: Vec<u8> = deps.querier.query(&query).map_err(ContractError::Std)?;
        Ok(response)
    }

    fn to_validation_result(&self, result: bool, msg: &str) -> Result<(), ContractError> {
        match result {
            true => Ok(()),
            false => Err(ContractError::LightClientValidationFailed(msg.to_string())),
        }
    }
    pub fn get_consensus_state(
        &self,
        deps: Deps,
        client_id: &IbcClientId,
        height: u64,
    ) -> Result<Vec<u8>, ContractError> {
        let query_message = cw_common::client_msg::QueryMsg::GetConsensusState {
            client_id: client_id.as_str().to_string(),
            height,
        };
        let msg = to_binary(&query_message).map_err(ContractError::Std)?;

        let query = build_smart_query(self.address.clone(), msg);

        let response: Vec<u8> = deps.querier.query(&query).map_err(ContractError::Std)?;
        Ok(response)
    }
    pub fn get_timestamp_at_height_query(
        &self,

        client_id: &IbcClientId,
        height: u64,
    ) -> Result<Binary, ContractError> {
        let query_message = cw_common::client_msg::QueryMsg::GetTimestampAtHeight {
            client_id: client_id.as_str().to_string(),
            height,
        };
        let msg = to_binary(&query_message).map_err(ContractError::Std)?;
        Ok(msg)
    }

    pub fn get_timestamp_at_height(
        &self,
        deps: Deps,
        client_id: &IbcClientId,
        height: u64,
    ) -> Result<u64, ContractError> {
        let msg = self.get_timestamp_at_height_query(client_id, height)?;
        let query = build_smart_query(self.address.clone(), msg);

        let response: u64 = deps.querier.query(&query).map_err(ContractError::Std)?;
        Ok(response)
    }

    pub fn get_previous_consensus_state(
        &self,
        deps: Deps,
        client_id: &IbcClientId,
        height: u64,
    ) -> Result<Vec<u64>, ContractError> {
        let query_message = cw_common::client_msg::QueryMsg::GetPreviousConsensusState {
            client_id: client_id.as_str().to_string(),
            height,
        };
        let msg = to_binary(&query_message).map_err(ContractError::Std)?;

        let query = build_smart_query(self.address.clone(), msg);

        let response: Vec<u64> = deps.querier.query(&query).map_err(ContractError::Std)?;

        Ok(response)
    }
}
