use std::marker::PhantomData;

use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;

use common::icon::icon::types::v1::MerkleNode;
use common::icon::icon::types::v1::SignedHeader;
use common::utils::calculate_root;
use common::utils::keccak256;
use cosmwasm_std::Addr;

use cosmwasm_std::Api;

use cosmwasm_std::Deps;
use cosmwasm_std::Order;
use cosmwasm_std::StdResult;
use cosmwasm_std::Storage;
use cw_common::cw_println;
use cw_common::hex_string::HexString;
use cw_storage_plus::Bound;
use serde::Deserialize;
use serde::Serialize;

use crate::constants::CLIENT_STATES;
use crate::constants::CONFIG;
use crate::constants::CONSENSUS_STATES;
use crate::constants::PROCESSED_HEIGHTS;
use crate::constants::PROCESSED_TIMES;
use crate::ContractError;
use common::traits::AnyTypes;
use prost::Message;
use super::*;
pub trait IContext {
    fn get_client_state(&self, client_id: &str) -> Result<ClientState, ContractError>;

    fn insert_client_state(
        &mut self,
        client_id: &str,
        state: ClientState,
    ) -> Result<(), ContractError>;

    fn get_consensus_state(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<ConsensusState, ContractError>;
    fn insert_consensus_state(
        &mut self,
        client_id: &str,
        height: u64,
        state: ConsensusState,
    ) -> Result<(), ContractError>;

    fn get_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<u64, ContractError>;
    fn insert_timestamp_at_height(
        &mut self,
        client_id: &str,
        height: u64,
    ) -> Result<(), ContractError>;
    fn insert_blocknumber_at_height(
        &mut self,
        client_id: &str,
        height: u64,
    ) -> Result<(), ContractError>;

    fn recover_signer(&self, msg: &[u8], signature: &[u8]) -> Option<[u8; 20]> {
        if signature.len() != 65 {
            return None;
        }
        let mut rs = [0u8; 64];
        rs[..].copy_from_slice(&signature[..64]);
        let v = signature[64];
        let pubkey = self.api().secp256k1_recover_pubkey(msg, &rs, v).unwrap();
        let pubkey_hash = keccak256(&pubkey[1..]);
        let address: Option<[u8; 20]> = pubkey_hash.as_slice()[12..].try_into().ok();
        address
    }

    fn recover_icon_signer(&self, msg: &[u8], signature: &[u8]) -> Option<Vec<u8>> {
        self.recover_signer(msg, signature)
            .map(|addr| addr.to_vec())
    }

    fn get_config(&self) -> Result<Config, ContractError>;

    fn insert_config(&mut self, config: &Config) -> Result<(), ContractError>;

    fn get_current_block_time(&self) -> u64;
    fn get_current_block_height(&self) -> u64;
    fn get_processed_time_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, ContractError>;
    fn get_processed_block_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, ContractError>;

    fn ensure_owner(&self, caller: Addr) -> Result<(), ContractError>;
    fn ensure_ibc_host(&self, caller: &Addr) -> Result<(), ContractError>;
    fn api(&self) -> &dyn Api;
}
