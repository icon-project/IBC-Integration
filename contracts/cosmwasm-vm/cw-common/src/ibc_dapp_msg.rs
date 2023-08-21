use crate::cw_types::{
    CwChannelCloseMsg, CwChannelConnectMsg, CwChannelOpenMsg, CwPacketAckMsg, CwPacketReceiveMsg,
    CwPacketTimeoutMsg,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_xcall_lib::network_address::NetId;

#[cw_serde]
pub enum ExecuteMsg {
    #[cfg(not(feature = "native_ibc"))]
    IbcChannelOpen { msg: CwChannelOpenMsg },

    #[cfg(not(feature = "native_ibc"))]
    IbcChannelConnect { msg: CwChannelConnectMsg },
    #[cfg(not(feature = "native_ibc"))]
    IbcChannelClose { msg: CwChannelCloseMsg },
    #[cfg(not(feature = "native_ibc"))]
    IbcPacketReceive { msg: CwPacketReceiveMsg },
    #[cfg(not(feature = "native_ibc"))]
    IbcPacketAck { msg: CwPacketAckMsg },
    #[cfg(not(feature = "native_ibc"))]
    IbcPacketTimeout { msg: CwPacketTimeoutMsg },
}
