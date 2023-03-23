use std::collections::HashMap;

use crate::traits::AnyTypes;
use crate::traits::{ConsensusStateUpdate, IContext, ILightClient};
use crate::ContractError;
use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::icon::icon::types::v1::{BtpHeader, MerkleNode, SignedHeader};
use common::utils::{calculate_root, keccak256};
use ibc_proto::{google::protobuf::Any, ibc::core::client::v1::Height};
use prost::Message;

const HEADER_TYPE_URL: &str = "/icon.lightclient.v1.SignedHeader";
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

impl AnyTypes for SignedHeader {
    fn get_type_url() -> String {
        HEADER_TYPE_URL.to_string()
    }
}

pub struct IconClient<'a> {
    context: &'a dyn IContext<Error = crate::ContractError>,
}

impl<'a> IconClient<'a> {
    pub fn new(context: &'a dyn IContext<Error = crate::ContractError>) -> Self {
        Self { context }
    }
    pub fn has_quorum_of(n_validators: u128, votes: u128) -> bool {
        votes * 3 > n_validators * 2
    }
    pub fn check_block_proof(
        &self,
        client_id: &str,
        header: &BtpHeader,
        signatures: &Vec<Vec<u8>>,
    ) -> Result<bool, ContractError> {
        let mut votes = 0_u128;
        let state = self.context.get_client_state(client_id)?;
        let config = self.context.get_config()?;
        let decision = header
            .get_network_type_section_decision_hash(&config.src_network_id, config.network_type_id);
        let validators_map = common::utils::to_lookup(&state.validators);
        for (i, signature) in signatures.iter().enumerate() {
            let signer = self.context.recover_signer(decision.as_slice(), signature);
            if let Some(val) = signer {
                if let Some(_expected) = validators_map.get(&val.to_vec()) {
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
        Ok(true)
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

    

    fn update_client(
        &self,
        client_id: &str,
        signed_header_bytes: Any,
    ) -> Result<(Vec<u8>, ConsensusStateUpdate), Self::Error> {
        let signed_header = SignedHeader::from_any(signed_header_bytes)
            .map_err(|e| ContractError::DecodeError(e))?;
        let btp_header = signed_header.header.clone().unwrap();
        let mut state = self.context.get_client_state(client_id)?;
        let config = self.context.get_config()?;
        if state.latest_height != (btp_header.main_height - 1) {
            return Err(ContractError::InvalidHeightUpdate {
                saved_height: state.latest_height,
                update_height: btp_header.main_height,
            });
        }

        if state.network_section_hash != btp_header.prev_network_section_hash {
            return Err(ContractError::InvalidHeaderUpdate(
                "network section mismatch".to_string(),
            ));
        }

        if config.network_id != btp_header.network_id {
            return Err(ContractError::InvalidHeaderUpdate(
                "network id mismatch".to_string(),
            ));
        }

        let _valid = self.check_block_proof(client_id, &btp_header, &signed_header.signatures)?;

        state.validators = btp_header.next_validators.clone();
        state.latest_height = btp_header.main_height.into();
        state.network_section_hash = btp_header.get_network_section_hash().to_vec();
        let consensus_state = ConsensusState {
            message_root: btp_header.message_root,
        };
        self.context.insert_client_state(client_id, state.clone())?;
        self.context.insert_consensus_state(
            client_id,
            btp_header.main_height,
            consensus_state.clone(),
        )?;
        self.context
            .insert_timestamp_at_height(client_id, btp_header.main_height)?;
        let commitment = keccak256(&consensus_state.to_any().encode_to_vec());

        Ok((
            keccak256(&state.to_any().encode_to_vec()).to_vec(),
            ConsensusStateUpdate {
                consensus_state_commitment: commitment,
                height: btp_header.main_height,
            },
        ))
    }

    fn verify_membership(
        &self,
        client_id: &str,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &Vec<MerkleNode>,
       
        value: &[u8],
    ) -> Result<bool, Self::Error> {
        let leaf = keccak256(value);
        let message_root = calculate_root(leaf, proof);
        let consensus_state = self.context.get_consensus_state(&client_id, height)?;
        if consensus_state.message_root != message_root {
            return Err(ContractError::InvalidMessageRoot(hex::encode(message_root)));
        }

        Ok(true)
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

   
}
