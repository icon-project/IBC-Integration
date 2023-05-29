use dyn_clone::DynClone;

use crate::ibc::prelude::*;

use crate::ibc::core::ics24_host::identifier::ClientId;
use crate::ibc::Height;

pub trait Misbehaviour: DynClone + core::fmt::Debug + Send + Sync {
    /// The type of client (eg. Tendermint)
    fn client_id(&self) -> &ClientId;

    /// The height of the consensus state
    fn height(&self) -> Height;
}

// Implements `Clone` for `Box<dyn Misbehaviour>`
dyn_clone::clone_trait_object!(Misbehaviour);
