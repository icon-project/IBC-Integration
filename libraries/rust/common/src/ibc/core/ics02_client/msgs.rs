//! These are definitions of messages that a relayer submits to a chain. Specific implementations of
//! these messages can be found, for instance, in ICS 07 for Tendermint-specific chains. A chain
//! handles these messages in two layers: first with the general ICS 02 client handler, which
//! subsequently calls into the chain-specific (e.g., ICS 07) client handler. See:
//! <https://github.com/cosmos/ibc/tree/master/spec/core/ics-002-client-semantics#create>.

use crate::ibc::core::ics02_client::msgs::create_client::MsgCreateClient;
use crate::ibc::core::ics02_client::msgs::misbehaviour::MsgSubmitMisbehaviour;
use crate::ibc::core::ics02_client::msgs::update_client::MsgUpdateClient;
use crate::ibc::core::ics02_client::msgs::upgrade_client::MsgUpgradeClient;

pub mod create_client;
pub mod misbehaviour;
pub mod update_client;
pub mod upgrade_client;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ClientMsg {
    CreateClient(MsgCreateClient),
    UpdateClient(MsgUpdateClient),
    Misbehaviour(MsgSubmitMisbehaviour),
    UpgradeClient(MsgUpgradeClient),
}
