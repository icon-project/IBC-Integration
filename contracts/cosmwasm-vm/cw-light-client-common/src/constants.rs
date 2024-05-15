use common::icon::icon::lightclient::v1::TrustLevel;
use cw_storage_plus::{Item, Map};

use crate::traits::Config;
type ClientId = String;

pub const CLIENT_STATE_HASH: &str = "client_state_hash";
pub const CONSENSUS_STATE_HASH: &str = "consensus_state_hash";
pub const HEIGHT: &str = "height";
pub const TRUST_LEVEL: TrustLevel = TrustLevel {
    numerator: 2,
    denominator: 3,
};

pub const CLIENT_STATES: Map<String, Vec<u8>> = Map::new("CLIENT_STATES");
pub const CONSENSUS_STATES: Map<(ClientId, u64), Vec<u8>> = Map::new("CONSENSUS_STATES");
pub const PROCESSED_TIMES: Map<(ClientId, u64), u64> = Map::new("PROCESSED_TIMES");
pub const PROCESSED_HEIGHTS: Map<(ClientId, u64), u64> = Map::new("PROCESSED_HEIGHTS");

pub const CONFIG: Item<Config> = Item::new("CONFIG");
