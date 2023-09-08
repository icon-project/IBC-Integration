use std::str::from_utf8;

use super::*;

/// This is an implementation of several helper functions for working with IBC (Inter-Blockchain
/// Communication) modules in a Cosmos SDK-based blockchain application.
impl<'a> CwIbcCoreContext<'a> {
    /// This function looks up a module ID based on a given port ID.
    ///
    /// Arguments:
    ///
    /// * `store`: A mutable reference to a trait object of type `dyn Storage`. This is used to interact
    /// with the contract's storage.
    /// * `port_id`: A unique identifier for an IBC port.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either a `ModuleId` if the lookup is successful or a
    /// `ContractError` if there is an error.
    pub fn lookup_module_by_port(
        &self,
        store: &mut dyn Storage,
        port_id: &PortId,
    ) -> Result<ModuleId, ContractError> {
        match self.ibc_store().port_to_module().may_load(store, port_id) {
            Ok(result) => match result {
                Some(port_id) => Ok(port_id),
                None => Err(ContractError::IbcPortError {
                    error: PortError::UnknownPort {
                        port_id: port_id.clone(),
                    },
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This function stores a module ID by port ID in a storage object.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `Storage`. It is used to
    /// store the mapping between a `PortId` and a `ModuleId`.
    /// * `port_id`: `port_id` is an identifier for a port in the IBC (Inter-Blockchain Communication)
    /// protocol. It is used to uniquely identify a port and its associated module.
    /// * `module_id`: `module_id` is a unique identifier for a module in a blockchain network. It is
    /// used to reference and interact with the module's code and state. In the context of the code
    /// snippet you provided, `module_id` is being stored in the IBC (Inter-Blockchain Communication)
    /// store,
    ///
    /// Returns:
    ///
    /// a `Result` object with either an `Ok(())` value indicating success or a `ContractError` value
    /// indicating an error occurred.
    pub fn store_module_by_port(
        &self,
        store: &mut dyn Storage,
        port_id: &PortId,
        module_id: ModuleId,
    ) -> Result<(), ContractError> {
        Ok(self
            .ibc_store()
            .port_to_module()
            .save(store, port_id, &module_id)?)
    }

    /// This function binds a port to a given address and returns a response with relevant attributes.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the storage of the smart contract. The `dyn` keyword indicates that this is a
    /// dynamic dispatch trait object, which means that it can be used to call methods on any type that
    /// implements
    /// * `port_id`: The `port_id` parameter is a unique identifier for an IBC port. It is used to
    /// identify the port when sending and receiving packets over the IBC protocol.
    /// * `address`: The `address` parameter is a string representing the network address that the port
    /// will be bound to. This is typically an IP address or a domain name.
    ///
    /// Returns:
    ///
    /// a `Result` object that contains either a `Response` object or a `ContractError` object. If the
    /// function executes successfully, it will return a `Response` object with some attributes added to
    /// it. If there is an error, it will return a `ContractError` object.
    pub fn bind_port(
        &self,
        store: &mut dyn Storage,
        port_id: &IbcPortId,
        address: String,
    ) -> Result<Response, ContractError> {
        self.claim_capability(store, port_id.as_str().as_bytes().to_vec(), address.clone())?;

        self.store_module_by_port(store, port_id, ModuleId::from_str(&address).unwrap())
            .unwrap();
        Ok(Response::new()
            .add_attribute("method", "bind_port")
            .add_attribute("port_id", port_id.as_str())
            .add_attribute("address", address))
    }
    /// This function returns a vector of bytes representing the capability path of a channel given its port
    /// ID and channel ID.
    ///
    /// Arguments:
    ///
    /// * `port_id`: An identifier for a port on a blockchain network. It is used to uniquely identify the
    /// source or destination of an inter-blockchain communication (IBC) message.
    /// * `channel_id`: The `channel_id` parameter is a unique identifier for an IBC channel. It is used to
    /// reference a specific channel within a port.
    ///
    /// Returns:
    ///
    /// A vector of bytes representing the path of a channel capability, which is constructed by
    /// concatenating the strings "ports/", the provided `port_id`, "/channels/", and the provided
    /// `channel_id`.

    pub fn channel_capability_path(
        &self,
        port_id: &IbcPortId,
        channel_id: &IbcChannelId,
    ) -> Vec<u8> {
        let path = format!("ports/{port_id}/channels/{channel_id}");
        path.as_bytes().to_vec()
    }

    pub fn get_all_ports(&self, store: &dyn Storage) -> Result<Vec<String>, StdError> {
        return self
            .ibc_store()
            .capabilities()
            .keys(store, None, None, cosmwasm_std::Order::Ascending)
            .map(|k| {
                k.and_then(|key| {
                    return from_utf8(&key).map(|k| k.to_string()).map_err(|_e| {
                        StdError::ParseErr {
                            target_type: "string".to_owned(),
                            msg: "failed to parse key".to_owned(),
                        }
                    });
                })
            })
            .collect::<Result<Vec<String>, StdError>>();
    }
}
