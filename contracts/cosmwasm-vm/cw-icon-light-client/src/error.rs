use cosmwasm_std::StdError;
use cw_common::errors::CwErrors;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("{0}")]
    DecodeError(String),
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
    #[error("Trusting Period elapsed. Height: {update_height:?} client is at {saved_height:?}")]
    TrustingPeriodElapsed {
        saved_height: u64,
        update_height: u64,
    },
    #[error("Invalid header update {0}")]
    InvalidHeaderUpdate(String),
    #[error("Invalid message root {0}")]
    InvalidMessageRoot(String),
    #[error("Failed to save processed time")]
    FailedToSaveProcessedTime,
    #[error("Processed time not found for {client_id:?} at height {height:?}")]
    ProcessedTimeNotFound { client_id: String, height: u64 },
    #[error("Processed height not found for {client_id:?} at height {height:?}")]
    ProcessedHeightNotFound { client_id: String, height: u64 },
    #[error("Too early to process by time elapsed")]
    NotEnoughtTimeElapsed,
    #[error("Too early to process by block elapsed")]
    NotEnoughtBlocksElapsed,
    #[error("Failed to init contract")]
    FailedToInitContract,
    #[error("Failed to save config")]
    FailedToSaveConfig,
    #[error("Client state frozen at {0}")]
    ClientStateFrozen(u64),
    #[error("Failed Parsing Height {0}")]
    FailedToParseHeight(String),

    #[error("Invalid Client Id {0}")]
    InvalidClientId(String),

    #[error("Failed To Create ClientId")]
    FailedToCreateClientId(String),
}

impl From<CwErrors> for ContractError {
    fn from(value: CwErrors) -> Self {
        match value {
            CwErrors::FailedToCreateClientId {
                client_type:_,
                counter:_,
                validation_error,
            } => ContractError::FailedToCreateClientId(validation_error.to_string()),
            CwErrors::InvalidClientId(e, err) => ContractError::InvalidClientId(e),
            CwErrors::DecodeError { error } => ContractError::DecodeError(error),
            CwErrors::FailedToConvertToPacketDataResponse(e) => ContractError::Std(e),
        }
    }
}
