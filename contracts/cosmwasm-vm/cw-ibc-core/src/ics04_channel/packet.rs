use super::*;
use common::ibc::core::ics04_channel::msgs::timeout::MsgTimeout;
use common::ibc::core::ics04_channel::msgs::timeout_on_close::MsgTimeoutOnClose;

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
    /// This function validates a timeout packet based on its type.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract. It allows the
    /// contract to interact with the blockchain state and other contracts.
    /// * `info`: `info` is a struct that contains information about the message being processed, such
    /// as the sender's address, the amount of tokens being sent, and any attached data. It is of type
    /// `MessageInfo`.
    /// * `timeout_msg_type`: A variable of type `TimeoutMsgType` that represents the type of timeout
    /// message being validated. It can either be `Timeout` or `TimeoutOnClose`.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing the possible errors that can occur during
    /// contract execution.
    pub fn timeout_packet_validate(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        timeout_msg_type: TimeoutMsgType,
    ) -> Result<Response, ContractError> {
        match timeout_msg_type {
            TimeoutMsgType::Timeout(msg) => {
                self.timeout_packet_validate_to_light_client(deps, info, msg)
            }
            TimeoutMsgType::TimeoutOnClose(msg) => {
                self.timeout_on_close_packet_validate_to_light_client(deps, info, msg)
            }
        }
    }
}
