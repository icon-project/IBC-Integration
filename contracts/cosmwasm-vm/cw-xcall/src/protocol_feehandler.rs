use cosmwasm_std::{Deps, DepsMut};

use crate::{types::{address::Address}, state::CwCallservice};


impl <'a> CwCallservice <'a>{
pub fn fee(&self, deps: DepsMut, address : Address){
self.ensure_admin_call_or_not(deps.storage);
let f = CwCallservice::new().fee_handler().save(deps.as_ref(), address);
if let Some(address) = address{
    
}

}
}