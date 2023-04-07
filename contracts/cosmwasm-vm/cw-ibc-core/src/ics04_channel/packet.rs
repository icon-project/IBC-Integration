use super::*;
use ibc::core::ics04_channel::msgs::timeout::MsgTimeout;
use ibc::core::ics04_channel::msgs::timeout_on_close::MsgTimeoutOnClose;

pub mod timeout;
pub use timeout::*;
pub mod functions;
pub use functions::*;
pub mod packet_types;
pub use packet_types::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn timeout_packet_validate(
        &self,
        module_id: ModuleId,
        timeout_msg_type: TimeoutMsgType,
    ) -> Result<Response, ContractError> {
        let s = match &timeout_msg_type {
            TimeoutMsgType::Timeout(msg) => (),
            TimeoutMsgType::TimeoutOnClose(msg) => (),
        };
        // timeout_packet_validate_to_light_client

        let (packet, signer) = match timeout_msg_type {
            TimeoutMsgType::Timeout(msg) => (msg.packet, msg.signer),
            TimeoutMsgType::TimeoutOnClose(msg) => (msg.packet, msg.signer),
        };

        Ok(Response::new())
    }
}
