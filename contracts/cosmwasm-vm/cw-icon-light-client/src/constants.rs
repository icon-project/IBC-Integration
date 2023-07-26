use common::icon::icon::lightclient::v1::TrustLevel;

pub const CLIENT_STATE_HASH: &str = "client_state_hash";
pub const CONSENSUS_STATE_HASH: &str = "consensus_state_hash";
pub const HEIGHT: &str = "height";
pub const TRUST_LEVEL: TrustLevel = TrustLevel {
    numerator: 2,
    denominator: 3,
};
