use std::marker::PhantomData;

use crate::{
    constants::{CLIENT_STATES, CONFIG, CONSENSUS_STATES, PROCESSED_HEIGHTS, PROCESSED_TIMES},
    traits::Config,
    ContractError,
};
use common::{
    icon::icon::{
        lightclient::v1::{ClientState, ConsensusState},
        types::v1::MerkleNode,
    },
    traits::AnyTypes,
    utils::{calculate_root, keccak256},
};
use cosmwasm_std::{Deps, Order, StdResult, Storage};
use cw_common::{cw_println, hex_string::HexString};
use cw_light_client_common::traits::IQueryHandler;
use cw_storage_plus::Bound;

use prost::Message;

pub struct QueryHandler;
impl IQueryHandler for QueryHandler {}
#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::MockStorage;

    use crate::{constants::CONSENSUS_STATES, query_handler::IQueryHandler};

    use super::QueryHandler;

    #[test]
    fn test_previous_consensus() {
        let mut store = MockStorage::new();
        CONSENSUS_STATES
            .save(&mut store, ("test".to_string(), 100), &vec![1, 2, 4, 5])
            .unwrap();
        CONSENSUS_STATES
            .save(&mut store, ("test".to_string(), 80), &vec![1, 2, 4, 5])
            .unwrap();
        CONSENSUS_STATES
            .save(&mut store, ("test".to_string(), 70), &vec![1, 2, 4, 5])
            .unwrap();

        let result = QueryHandler::get_previous_consensus(&store, 110, "test".to_string()).unwrap();

        println!("{result:?}");
    }
}
