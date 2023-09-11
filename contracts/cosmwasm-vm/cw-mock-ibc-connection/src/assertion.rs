use cosmwasm_std::{ensure_eq, Addr, Storage};

use crate::{error::ContractError, state::CwIbcConnection, types::LOG_PREFIX};

impl<'a> CwIbcConnection<'a> {
    pub fn ensure_xcall_handler(
        &self,
        store: &dyn Storage,
        address: Addr,
    ) -> Result<(), ContractError> {
        let ibc_host = self.get_xcall_host(store)?;

        if ibc_host != address {
            println!("{LOG_PREFIX} Invalid Xcall Handler ");
            return Err(ContractError::OnlyXcallHandler {});
        }
        Ok(())
    }

    pub fn ensure_admin(&self, store: &dyn Storage, address: Addr) -> Result<(), ContractError> {
        let admin = self.query_admin(store)?;
        ensure_eq!(admin, address, ContractError::OnlyAdmin);

        Ok(())
    }
}
