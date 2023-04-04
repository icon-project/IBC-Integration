use super::*;

#[cw_serde]
pub enum LightClientConnectionMessage{
    OpenTry{
        proof_height: String,
        counterparty_prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        counterparty_chan_end_path: Vec<u8>,
        expected_counterparty_channel_end: Vec<u8>,
    }
}