use cosmwasm_std::StdError;
use prost::DecodeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("{0}")]
    DecodeError(#[from] DecodeError),
    #[error("Timestamp not found for {client_id:?} at height {height:?}")]
    TimestampNotFound { height: u64, client_id: String },
    #[error("Client state not found for client_id:{0}")]
    ClientStateNotFound(String),
    #[error("Height not found in client state for client_id:{0}")]
    HeightNotSaved(String),
    #[error("Consensusstate not found for {client_id:?} at height {height:?}")]
    ConsensusStateNotFound { height: u64, client_id: String },
    #[error("Failed to save client state")]
    FailedToSaveClientState,

    #[error("Failed to save consensus state")]
    FailedToSaveConsensusState,
    #[error("Insufficient validator signatures supplied")]
    InSuffcientQuorum,
    #[error("Clientstate already exists for {0}")]
    ClientStateAlreadyExists(String),
    #[error("Config not found or initialized")]
    ConfigNotFound,
    #[error("Failed to save {update_height:?} client is at {saved_height:?}")]
    InvalidHeightUpdate {
        saved_height: u64,
        update_height: u64,
    },
    #[error("Invalid header update {0}")]
    InvalidHeaderUpdate(String),
    #[error("Invalid message root {0}")]
    InvalidMessageRoot(String),
    #[error("Failed to save processed time")]
    FailedToSaveProcessedTime,
}
