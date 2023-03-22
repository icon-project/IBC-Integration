use std::collections::HashMap;

use crate::traits::AnyTypes;
use crate::traits::{ConsensusStateUpdate, IContext, ILightClient};
use crate::ContractError;
use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::icon::icon::types::v1::BtpHeader;
use common::utils::keccak256;
use ibc_proto::{google::protobuf::Any, ibc::core::client::v1::Height};
use prost::Message;

const HEADER_TYPE_URL: &str = "/icon.lightclient.v1.Header";
const CLIENT_STATE_TYPE_URL: &str = "/icon.lightclient.v1.ClientState";
const CONSENSUS_STATE_TYPE_URL: &str = "/icon.lightclient.v1.ConsensusState";

impl AnyTypes for ClientState {
    fn get_type_url() -> String {
        CLIENT_STATE_TYPE_URL.to_string()
    }
}

impl AnyTypes for ConsensusState {
    fn get_type_url() -> String {
        CONSENSUS_STATE_TYPE_URL.to_string()
    }
}

struct IconClient<'a> {
    context: &'a dyn IContext<Error = crate::ContractError>,
}

impl IconClient<'_> {
    pub fn has_quorum_of(n_validators: u128, votes: u128) -> bool {
        votes * 3 > n_validators * 2
    }
    pub fn check_block_proof(
        &self,
        client_id: &str,
        header: &BtpHeader,
        signatures: Vec<Vec<u8>>,
    ) -> Result<(), ContractError> {
        let mut votes = 0_u128;
        let state = self.context.get_client_state(client_id).unwrap();
        let decision = header.get_network_section_root();
        let validators_map = common::utils::to_lookup(&state.validators);
        for (i, signature) in signatures.iter().enumerate() {
            let signer = self.context.recover_signer(decision.as_slice(), signature);
            if let Some(val) = signer {
                if let Some(expected) = validators_map.get(&val.to_vec()) {
                    votes = votes + 1;
                }
            }

            if Self::has_quorum_of(state.validators.len() as u128, votes) {
                break;
            }
        }
        if !Self::has_quorum_of(state.validators.len() as u128, votes) {
            return Err(ContractError::InSuffcientQuorum);
        }
        Ok(())
    }
}

impl ILightClient for IconClient<'_> {
    type Error = crate::ContractError;

    fn create_client(
        &self,
        client_id: &str,
        client_state_bytes: Any,
        consensus_state_bytes: Any,
    ) -> Result<(Vec<u8>, ConsensusStateUpdate), Self::Error> {
        let client_state = ClientState::from_any(client_state_bytes.clone())
            .map_err(|e| ContractError::DecodeError(e))?;
        let consensus_state = ConsensusState::from_any(consensus_state_bytes.clone())
            .map_err(|e| ContractError::DecodeError(e))?;

        self.context
            .insert_client_state(&client_id, client_state.clone())?;
        self.context.insert_consensus_state(
            &client_id,
            client_state.latest_height.into(),
            consensus_state,
        )?;

        Ok((
            keccak256(&client_state_bytes.encode_to_vec()).into(),
            ConsensusStateUpdate {
                consensus_state_commitment: keccak256(&consensus_state_bytes.encode_to_vec()),
                height: client_state.latest_height,
            },
        ))
    }

    fn get_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<u64, Self::Error> {
        let timestamp = self.context.get_timestamp_at_height(client_id, height)?;
        Ok(timestamp)
    }

    fn get_latest_height(&self, client_id: &str) -> Result<u64, Self::Error> {
        let state = self.context.get_client_state(client_id)?;

        Ok(state.latest_height)
    }

    fn update_client(
        &self,
        client_id: &str,
        header: Any,
    ) -> Result<(Vec<u8>, Vec<ConsensusStateUpdate>, bool), Self::Error> {
        todo!()
    }

    fn verify_membership(
        &self,
        client_id: &str,
        height: &Height,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &[u8],
        prefix: &[u8],
        path: &[u8],
        value: &[u8],
    ) -> Result<bool, Self::Error> {
        todo!()
    }

    fn verify_non_membership(
        &self,
        client_id: &str,
        height: &Height,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &[u8],
        prefix: &[u8],
        path: &[u8],
    ) -> Result<bool, Self::Error> {
        todo!()
    }

    fn get_client_state(&self, client_id: &str) -> Result<Vec<u8>, Self::Error> {
        let state = self.context.get_client_state(client_id)?;
        let any_state = state.to_any();
        Ok(any_state.encode_to_vec())
    }

    fn get_consensus_state(&self, client_id: &str, height: u64) -> Result<Vec<u8>, Self::Error> {
        let state = self.context.get_consensus_state(client_id, height)?;
        let any_state = state.to_any();
        Ok(any_state.encode_to_vec())
    }
}
