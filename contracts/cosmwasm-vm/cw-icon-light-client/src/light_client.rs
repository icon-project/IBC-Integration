use crate::constants::TRUST_LEVEL;
use crate::traits::{ConsensusStateUpdate, IContext, ILightClient};
use crate::ContractError;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::icon::icon::lightclient::v1::{ClientState, TrustLevel};
use common::icon::icon::types::v1::{BtpHeader, MerkleNode, SignedHeader};
use common::traits::AnyTypes;
use common::utils::{calculate_root, keccak256};
use cosmwasm_std::Addr;
use cw_common::hex_string::HexString;
use debug_print::debug_println;
use prost::Message;

pub struct IconClient<'a> {
    context: &'a mut dyn IContext<Error = crate::ContractError>,
}

impl<'a> IconClient<'a> {
    pub fn new(context: &'a mut dyn IContext<Error = crate::ContractError>) -> Self {
        Self { context }
    }
    pub fn has_quorum_of(n_validators: u64, votes: u64, trust_level: &TrustLevel) -> bool {
        votes * trust_level.denominator > n_validators * trust_level.numerator
    }
    pub fn check_block_proof(
        &self,
        client_id: &str,
        header: &BtpHeader,
        signatures: &Vec<Vec<u8>>,
        validators: &Vec<Vec<u8>>,
    ) -> Result<bool, ContractError> {
        let mut votes = u64::default();
        let state = self.context.get_client_state(client_id)?;
        let trust_level: &TrustLevel = &TRUST_LEVEL;
        let decision = header
            .get_network_type_section_decision_hash(&state.src_network_id, state.network_type_id);
        debug_println!(
            "network type section decision hash {}",
            hex::encode(decision)
        );
        let validators_map = common::utils::to_lookup(validators);

        let num_validators = validators.len() as u64;

        for signature in signatures {
            let signer = self
                .context
                .recover_icon_signer(decision.as_slice(), signature);
            if let Some(val) = signer {
                if validators_map.contains_key(&val) {
                    votes += 1;
                }
            }

            if Self::has_quorum_of(num_validators, votes, trust_level) {
                break;
            }
        }
        if !Self::has_quorum_of(num_validators, votes, trust_level) {
            debug_println!("Insuffcient Quorom detected");
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
        &mut self,
        caller: Addr,
        client_id: &str,
        client_state: ClientState,
        consensus_state: ConsensusState,
    ) -> Result<ConsensusStateUpdate, Self::Error> {
        self.context.ensure_ibc_host(caller)?;
        let exists = self.context.get_client_state(client_id).is_ok();
        if exists {
            return Err(ContractError::ClientStateAlreadyExists(
                client_id.to_string(),
            ));
        }
        self.context
            .insert_client_state(client_id, client_state.clone())?;
        self.context.insert_consensus_state(
            client_id,
            client_state.latest_height,
            consensus_state.clone(),
        )?;

        Ok(ConsensusStateUpdate {
            consensus_state_commitment: consensus_state.get_keccak_hash(),
            client_state_commitment: client_state.get_keccak_hash(),
            client_state_bytes: client_state.encode_to_vec(),
            consensus_state_bytes: consensus_state.encode_to_vec(),
            height: client_state.latest_height,
        })
    }

    fn update_client(
        &mut self,
        caller: Addr,
        client_id: &str,
        signed_header: SignedHeader,
    ) -> Result<ConsensusStateUpdate, Self::Error> {
        self.context.ensure_ibc_host(caller)?;
        let btp_header = signed_header.header.clone().unwrap();

        let mut state = self.context.get_client_state(client_id)?;

        if signed_header.trusted_height > btp_header.main_height {
            return Err(ContractError::UpdateBlockOlderThanTrustedHeight);
        }

        let trusted_consensus_state = self
            .context
            .get_consensus_state(client_id, signed_header.trusted_height)?;

        let current_proof_context_hash =
            btp_header.get_next_proof_context_hash(&signed_header.current_validators);

        if current_proof_context_hash != trusted_consensus_state.next_proof_context_hash {
            return Err(ContractError::InvalidProofContextHash);
        }

        if (btp_header.main_height - signed_header.trusted_height) > state.trusting_period {
            return Err(ContractError::TrustingPeriodElapsed {
                trusted_height: signed_header.trusted_height,
                update_height: btp_header.main_height,
            });
        }

        if btp_header.main_height < state.latest_height
            && (state.latest_height - btp_header.main_height) > state.trusting_period
        {
            return Err(ContractError::UpdateBlockTooOld);
        }

        if state.network_id != btp_header.network_id {
            return Err(ContractError::InvalidHeaderUpdate(
                "network id mismatch".to_string(),
            ));
        }

        let _valid = self.check_block_proof(
            client_id,
            &btp_header,
            &signed_header.signatures,
            &signed_header.current_validators,
        )?;

        if state.latest_height < btp_header.main_height {
            state.latest_height = btp_header.main_height;
        }

        let consensus_state = btp_header.to_consensus_state();
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
        let commitment = keccak256(&consensus_state.encode_to_vec());

        Ok(ConsensusStateUpdate {
            consensus_state_commitment: commitment,
            client_state_commitment: keccak256(&state.encode_to_vec()),
            client_state_bytes: state.encode_to_vec(),
            consensus_state_bytes: consensus_state.encode_to_vec(),
            height: btp_header.main_height,
        })
    }

    fn verify_membership(
        &self,
        client_id: &str,
        height: u64,
        _delay_time_period: u64,
        _delay_block_period: u64,
        proof: &Vec<MerkleNode>,
        path: &[u8],
        value: &[u8],
    ) -> Result<bool, Self::Error> {
        debug_println!(
            "[LightClient]: Path Bytes  {:?}",
            HexString::from_bytes(path)
        );
        debug_println!(
            "[LightClient]: Value Bytes  {:?}",
            HexString::from_bytes(value)
        );
        let path = keccak256(path).to_vec();
        debug_println!("[LightClient]: client id is: {:?}", client_id);

        let state = self.context.get_client_state(client_id)?;

        if state.frozen_height != 0 && height > state.frozen_height {
            return Err(ContractError::ClientStateFrozen(state.frozen_height));
        }

        let mut value_hash = value.to_vec();
        if !value.is_empty() {
            value_hash = keccak256(value).to_vec();
        }

        // let _ =
        //     self.validate_delay_args(client_id, height, delay_time_period, delay_block_period)?;
        let consensus_state: ConsensusState =
            self.context.get_consensus_state(client_id, height)?;
        debug_println!(
            "[LightClient]: Path Hash {:?}",
            HexString::from_bytes(&path)
        );
        debug_println!(
            "[LightClient]: Value Hash {:?}",
            HexString::from_bytes(&value_hash)
        );
        let leaf = keccak256(&[path, value_hash].concat());
        debug_println!(
            "[LightClient]: Leaf Value {:?}",
            HexString::from_bytes(&leaf)
        );

        let message_root = calculate_root(leaf, proof);
        debug_println!(
            "[LightClient]: Stored Message Root {:?} ",
            hex::encode(consensus_state.message_root.clone())
        );
        debug_println!(
            "[LightClient]: Calculated Message Root : {:?}",
            HexString::from_bytes(&message_root)
        );
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
        self.verify_membership(
            client_id,
            height,
            delay_time_period,
            delay_block_period,
            proof,
            path,
            &[],
        )
    }
}
