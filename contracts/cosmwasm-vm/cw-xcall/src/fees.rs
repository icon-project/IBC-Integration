use cw_xcall_lib::network_address::NetId;

use super::*;
/// This is an implementation of two methods for the `CwCallService` struct.

impl<'a> CwCallService<'a> {
    /// The `set_protocol_fee` function sets the protocol fee and the `get_protocol_fee` function
    /// retrieves the current protocol fee.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` or `Deps` object that provides access to the contract's
    /// dependencies such as storage, API, and other modules. `DepsMut` is used when the function needs
    /// to modify the state of the contract, while `Deps` is used
    /// * `info`: MessageInfo is a struct that contains information about the message being executed,
    /// such as the sender's address, the amount of coins being sent, and the gas limit. It is provided
    /// by the Cosmos SDK to the contract's entry points.
    /// * `value`: The `value` parameter in both functions represents the amount of protocol fee to be
    /// set or retrieved. It is of type `u128`, which means it can hold a large unsigned integer value.
    /// The protocol fee is a fee charged by the contract for executing certain operations or
    /// transactions on the blockchain.
    ///
    /// Returns:
    ///
    /// The `set_protocol_fee` function returns a `Result<Response, ContractError>` which contains a
    /// `Response` object with an added attribute "method" set to "set_protocolfee". The
    /// `get_protocol_fee` function returns a `u128` value which represents the current protocol fee.
    pub fn set_protocol_fee(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        value: u128,
    ) -> Result<Response, ContractError> {
        self.ensure_admin(deps.storage, info.sender)?;
        self.store_protocol_fee(deps.storage, value)?;

        Ok(Response::new().add_attribute("method", "set_protocolfee"))
    }


    pub fn get_fee(&self, deps: Deps,nid:NetId,rollback:bool,sources:Vec<String>)->Result<u128,ContractError>{
        let protocol_fee= self.get_protocol_fee(deps.storage);
        let mut sources=sources;
        if sources.is_empty(){
            let conn=self.get_default_connection(deps.storage, nid.clone())?;
            sources=vec![conn.to_string()];
        }
        let conn_fees=sources.into_iter().map(|s|{
            return self.query_connection_fee(deps, nid.clone(), rollback, &s)
        }).collect::<Result<Vec<u128>,ContractError>>()?;
        let conn_total:u128=conn_fees.iter().sum();

        Ok(protocol_fee+conn_total)

    }

    
}
