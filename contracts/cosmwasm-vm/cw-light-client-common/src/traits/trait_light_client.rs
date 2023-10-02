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
pub trait ILightClient {
    type Error;
    /**
     * @dev createClient creates a new client with the given state.
     * If succeeded, it returns a commitment for the initial state.
     */
    fn create_client(
        &mut self,
        caller: Addr,
        client_id: &str,
        client_state: ClientState,
        consensus_state: ConsensusState,
    ) -> Result<ConsensusStateUpdate, Self::Error>;

    /**
     * @dev updateClient updates the client corresponding to `clientId`.
     * If succeeded, it returns a commitment for the updated state.
     * If there are no updates for consensus state, this function should returns an empty array as `updates`.
     *
     * NOTE: updateClient is intended to perform the followings:
     * 1. verify a given client message(e.g. header)
     * 2. check misbehaviour such like duplicate block height
     * 3. if misbehaviour is found, update state accordingly and return
     * 4. update state(s) with the client message
     * 5. persist the state(s) on the host
     */
    fn update_client(
        &mut self,
        caller: Addr,
        client_id: &str,
        header: SignedHeader,
    ) -> Result<ConsensusStateUpdate, Self::Error>;

    fn verify_header(
        &mut self,
        caller: &Addr,
        client_id: &str,
        header: &SignedHeader,
    ) -> Result<(), Self::Error>;
}