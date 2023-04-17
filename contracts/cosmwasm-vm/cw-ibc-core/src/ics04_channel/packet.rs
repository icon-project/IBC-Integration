use super::*;
use ibc::core::ics04_channel::msgs::timeout::MsgTimeout;
use ibc::core::ics04_channel::msgs::timeout_on_close::MsgTimeoutOnClose;

pub mod timeout;
pub use timeout::*;
pub mod timeout_on_close;
pub use timeout_on_close::*;
pub mod send_packet;
pub use send_packet::*;
pub mod receive_packet;
pub use receive_packet::*;
pub mod acknowledgement;
pub use acknowledgement::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn timeout_packet_validate(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        timeout_msg_type: TimeoutMsgType,
    ) -> Result<Response, ContractError> {
        match &timeout_msg_type {
            TimeoutMsgType::Timeout(msg) => {
                self.timeout_packet_validate_to_light_client(deps, info, msg)
            }
            TimeoutMsgType::TimeoutOnClose(msg) => {
                self.timeout_on_close_packet_validate_to_light_client(deps, info, msg)
            }
        }
    }
}
