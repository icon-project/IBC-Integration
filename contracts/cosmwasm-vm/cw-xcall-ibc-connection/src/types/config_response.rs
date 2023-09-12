use cw_common::xcall_connection_msg::ConfigResponse;

use crate::types::channel_config::ChannelConfig;
use crate::state::IbcConfig;

pub fn to_config_response(ibc_config: IbcConfig, channel_config: ChannelConfig) -> ConfigResponse {
    ConfigResponse {
        channel_id: ibc_config.src_endpoint().channel_id.clone(),
        port: ibc_config.src_endpoint().port_id.clone(),
        destination_channel_id: ibc_config.dst_endpoint().channel_id.clone(),
        destination_port_id: ibc_config.dst_endpoint().port_id.clone(),
        light_client_id: channel_config.client_id,
        timeout_height: channel_config.timeout_height,
    }
}