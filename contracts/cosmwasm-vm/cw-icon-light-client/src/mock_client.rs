use crate::traits::AnyTypes;
use crate::traits::{ConsensusStateUpdate, IContext, ILightClient};
use crate::ContractError;
use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::icon::icon::types::v1::{BtpHeader, MerkleNode, SignedHeader};
use common::utils::{calculate_root, keccak256};
use prost::Message;

pub struct MockClient<'a> {
    context: &'a mut dyn IContext<Error = crate::ContractError>,
}

impl<'a> MockClient<'a> {
    pub fn new(context: &'a mut dyn IContext<Error = crate::ContractError>) -> Self {
        Self { context }
    }
}

impl ILightClient for MockClient<'_> {
    type Error = crate::ContractError;

    fn create_client(
        &mut self,
        client_id: &str,
        client_state: ClientState,
        consensus_state: ConsensusState,
    ) -> Result<(Vec<u8>, ConsensusStateUpdate), Self::Error> {
        self.context
            .insert_client_state(&client_id, client_state.clone())?;
        self.context.insert_consensus_state(
            &client_id,
            client_state.latest_height.into(),
            consensus_state.clone(),
        )?;

        Ok((
            client_state.get_keccak_hash().into(),
            ConsensusStateUpdate {
                consensus_state_commitment: consensus_state.get_keccak_hash(),
                height: client_state.latest_height,
            },
        ))
    }

    fn update_client(
        &mut self,
        client_id: &str,
        signed_header: SignedHeader,
    ) -> Result<(Vec<u8>, ConsensusStateUpdate), Self::Error> {
        let btp_header = signed_header.header.clone().unwrap();
        let mut state = self.context.get_client_state(client_id)?;
        let config = self.context.get_config()?;

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
