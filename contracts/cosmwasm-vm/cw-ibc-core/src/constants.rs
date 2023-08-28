// Constants for Reply messages

pub const EXECUTE_CREATE_CLIENT: u64 = 21;
pub const EXECUTE_UPDATE_CLIENT: u64 = 22;
pub const EXECUTE_UPGRADE_CLIENT: u64 = 23;
pub const MISBEHAVIOUR: u64 = 24;

pub const EXECUTE_CONNECTION_OPENTRY: u64 = 31;
pub const EXECUTE_CONNECTION_OPENACK: u64 = 32;
pub const EXECUTE_CONNECTION_OPENCONFIRM: u64 = 33;

pub const EXECUTE_ON_CHANNEL_OPEN_INIT: u64 = 41;
pub const EXECUTE_ON_CHANNEL_OPEN_TRY: u64 = 422;

pub const EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE: u64 = 432;

pub const EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE: u64 = 442;

pub const EXECUTE_ON_CHANNEL_CLOSE_INIT: u64 = 45;

pub const EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE: u64 = 462;
pub const VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE: u64 = 542;
pub const VALIDATE_ON_PACKET_RECEIVE_ON_MODULE: u64 = 522;
pub const VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE: u64 = 532;

// Errors

pub const PACKET_ERROR: &str = "Packet Error";
pub const CHANNEL_ERROR: &str = "Channel Error";
pub const CONNECTION_ERROR: &str = "Connection Error";
pub const VALIDATION_ERROR: &str = "Validation Error";
pub const CLIENT_ERROR: &str = "Client Error";
pub const PORT_ERROR: &str = "Port Error";
