use crate::traits::AnyTypes;
use crate::traits::{ConsensusStateUpdate, IContext, ILightClient};
use crate::ContractError;
use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::icon::icon::types::v1::{BtpHeader, MerkleNode, SignedHeader};
use common::utils::{calculate_root, keccak256};
use ibc_proto::google::protobuf::Any;
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
        for (_i, signature) in signatures.iter().enumerate() {
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

    fn validate_delay_args(
        &self,
        client_id: &str,
        height: u64,
        delay_time: u64,
        delay_block: u64,
    ) -> Result<(), ContractError> {
        let processed_time = self
            .context
            .get_processed_time_at_height(client_id, height)?;
        let processed_height = self
            .context
            .get_processed_block_at_height(client_id, height)?;
        let current_time = self.context.get_current_block_time();
        let current_height = self.context.get_current_block_height();
        if !current_time >= (processed_time + delay_time) {
            return Err(ContractError::NotEnoughtTimeElapsed);
        }

        if !current_height >= (processed_height + delay_block) {
            return Err(ContractError::NotEnoughtBlocksElapsed);
        }

        Ok(())
    }
}

impl ILightClient for IconClient<'_> {
    type Error = crate::ContractError;

    fn create_client(
        &self,
        client_id: &str,
        trusting_period: u64,
        max_clock_drift: u64,
        btp_header: BtpHeader,
    ) -> Result<(Vec<u8>, ConsensusStateUpdate), Self::Error> {
        let client_state = ClientState {
            frozen_height: 0,
            latest_height: btp_header.main_height,
            trusting_period,
            max_clock_drift,
            network_section_hash: btp_header.get_network_section_hash().try_into().unwrap(),
            validators: btp_header.next_validators,
        };
        let consensus_state = ConsensusState {
            message_root: btp_header.message_root,
        };

        let client_state_bytes = client_state.to_any().encode_to_vec();
        let consensus_state_bytes = consensus_state.to_any().encode_to_vec();

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

        if (btp_header.main_height - state.latest_height) < state.trusting_period {
            return Err(ContractError::TrustingPeriodElapsed {
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
        self.context
            .insert_blocknumber_at_height(client_id, btp_header.main_height)?;
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
        path: &[u8],
        value: &[u8],
    ) -> Result<bool, Self::Error> {
        let state = self.context.get_client_state(client_id)?;
        if state.frozen_height != 0 && height > state.frozen_height {
            return Err(ContractError::ClientStateFrozen(state.frozen_height));
        }

        let _ =
            self.validate_delay_args(client_id, height, delay_time_period, delay_block_period)?;
        let consensus_state = self.context.get_consensus_state(&client_id, height)?;
        let leaf = keccak256(&[path, value].concat());
        let message_root = calculate_root(leaf, proof);
        if consensus_state.message_root != message_root {
            return Err(ContractError::InvalidMessageRoot(hex::encode(message_root)));
        }

        Ok(true)
    }

    fn verify_non_membership(
        &self,
        client_id: &str,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &Vec<MerkleNode>,
        path: &[u8],
    ) -> Result<bool, Self::Error> {
        return self.verify_membership(
            client_id,
            height,
            delay_time_period,
            delay_block_period,
            proof,
            &[],
            path,
        );
    }
}
