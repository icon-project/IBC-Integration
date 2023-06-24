use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum StorageKey {
    Owner,
    Admin,
    IbcConfig,
    IbcHost,
    TimeoutHeight,
    XCallHost,
    FeeHandler,
    Balance,
    Fee,
    ConfiguredNetworks,
    ChannelConfigs,
    ConnectionConfigs,
    NetworkFees,
    UnclaimedPacketFees,
    UnClaimedAckFees,
    IncomingPackets,
    OutGoingPackets,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::Owner => "owner",
            StorageKey::Admin => "admin",
            StorageKey::IbcConfig => "ibcconfig",
            StorageKey::IbcHost => "ibc_host",
            StorageKey::TimeoutHeight => "timeout_height",
            StorageKey::XCallHost => "xcall_host",
            StorageKey::FeeHandler => "feehandler",
            StorageKey::Balance => "balance",
            StorageKey::Fee => "fee",
            StorageKey::ConfiguredNetworks => "configured_networks",
            StorageKey::ChannelConfigs => "channel_configs",
            StorageKey::ConnectionConfigs => "connection_configs",
            StorageKey::NetworkFees => "network_fees",
            StorageKey::UnclaimedPacketFees => "unclaimed_packet_fees",
            StorageKey::IncomingPackets => "incoming_packets",
            StorageKey::OutGoingPackets => "outgoing_packets",
            StorageKey::UnClaimedAckFees => "unclaimed_ack_fees",
        }
    }
}
