use crate::helpers::keccak256;
use crate::traits::{AnyTypes, IHeight};
use crate::traits::{ConsensusStateUpdate, IContext, ILightClient};
use crate::ContractError;
use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::icon::icon::lightclient::v1::Header;
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

impl AnyTypes for Header {
    fn get_type_url() -> String {
        HEADER_TYPE_URL.to_string()
    }
}

struct IconClient<'a> {
    store: &'a dyn IContext<Error = crate::ContractError>,
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
        let latest_height = client_state
            .latest_height
            .clone()
            .ok_or(ContractError::HeightNotSaved(client_id.to_string()))?;
        let height_no = <Height as IHeight>::to_uint128(&latest_height);
        self.store.insert_client_state(&client_id, client_state)?;
        self.store
            .insert_consensus_state(&client_id, height_no, consensus_state)?;

        Ok((
            keccak256(&client_state_bytes.encode_to_vec()).into(),
            ConsensusStateUpdate {
                consensus_state_commitment: keccak256(&consensus_state_bytes.encode_to_vec()),
                height: latest_height,
            },
        ))
    }

    fn get_timestamp_at_height(
        &self,
        client_id: &str,
        height: &Height,
    ) -> Result<u64, Self::Error> {
        let height_no = Height::to_uint128(height);
        let timestamp = self.store.get_timestamp_at_height(client_id, height_no)?;
        Ok(timestamp)
    }

    fn get_latest_height(&self, client_id: &str) -> Result<Height, Self::Error> {
        let state = self.store.get_client_state(client_id)?;

        let height = state
            .latest_height
            .ok_or(ContractError::HeightNotSaved(client_id.to_string()))?;
        Ok(height)
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
        let state = self.store.get_client_state(client_id)?;
        let any_state = state.to_any();
        Ok(any_state.encode_to_vec())
    }

    fn get_consensus_state(
        &self,
        client_id: &str,
        height: &Height,
    ) -> Result<Vec<u8>, Self::Error> {
        let height_no = Height::to_uint128(height);
        let state = self.store.get_consensus_state(client_id, height_no)?;
        let any_state = state.to_any();
        Ok(any_state.encode_to_vec())
    }
}
