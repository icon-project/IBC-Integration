use crate::{
    conversions::to_ibc_client_id, light_client::light_client::LightClient, EXECUTE_CREATE_CLIENT,
    EXECUTE_UPDATE_CLIENT, EXECUTE_UPGRADE_CLIENT, MISBEHAVIOUR,
};

use super::{events::client_misbehaviour_event, *};

use cosmwasm_std::Env;
use cw_common::{
    client_msg::ExecuteMsg as LightClientMessage,
    from_binary_response,
    raw_types::client::{
        RawMsgCreateClient, RawMsgSubmitMisbehaviour, RawMsgUpdateClient, RawMsgUpgradeClient,
    },
};

use cw_common::cw_println;
use prost::{DecodeError, Message};

impl<'a> IbcClient for CwIbcCoreContext<'a> {
    /// This method creates a new client and sends a message to a light client contract.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides access to the contract's dependencies
    /// such as storage, API, and querier. It is used to interact with the blockchain and other
    /// contracts.
    /// * `info`: `info` is a struct that contains information about the message sender, such as their
    /// address, the amount of funds they sent with the message, and the gas limit for executing the
    /// message. It is of type `MessageInfo`.
    /// * `message`: `message` is a struct of type `MsgCreateClient` which contains the necessary
    /// information to create a new client. It includes the client state and consensus state as well as
    /// other metadata such as the signer and fee.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing the possible errors that can occur during
    /// contract execution.
    fn create_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        message: RawMsgCreateClient,
    ) -> Result<Response, ContractError> {
        let client_state_any = message.client_state.ok_or(ContractError::IbcClientError {
            error: ClientError::MissingRawClientState,
        })?;
        let consensus_state_any = message
            .consensus_state
            .ok_or(ContractError::IbcClientError {
                error: ClientError::MissingRawConsensusState,
            })?;

        let client_state = self.decode_client_state(client_state_any.clone())?;
        let consensus_state = self.decode_consensus_state(consensus_state_any.clone())?;
        let client_type = client_state.client_type();
        let height = client_state.latest_height();
        let client_id = self.generate_client_identifier(deps.storage, client_type.clone())?;
        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type.clone())?;

        self.store_client_type(deps.storage, &client_id, client_type.clone())?;

        self.store_client_implementations(
            deps.storage,
            &client_id,
            LightClient::new(light_client_address),
        )?;

        self.store_client_commitment(deps.storage, &env, &client_id, client_state.hash())?;

        self.store_consensus_commitment(deps.storage, &client_id, height, consensus_state.hash())?;

        let event = create_client_event(
            client_id.as_str(),
            client_type.as_str(),
            &height.to_string(),
        );

        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type)?;

        let create_client_message = LightClientMessage::CreateClient {
            client_id: client_id.to_string(),
            client_state: client_state_any.encode_to_vec(),
            consensus_state: consensus_state_any.encode_to_vec(),
        };

        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&create_client_message).map_err(ContractError::Std)?,
            funds: info.funds,
        });
        let sub_msg = SubMsg {
            id: EXECUTE_CREATE_CLIENT,
            msg: create_client_message,
            gas_limit: None,
            reply_on: cosmwasm_std::ReplyOn::Never,
        };

        Ok(Response::new()
            .add_submessage(sub_msg)
            .add_event(event)
            .add_attribute("client_id", client_id.to_string())
            .add_attribute("method", "create_client"))
    }

    /// This method updates a client's information by sending a message to the client's address.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the sender
    /// of the message, such as the address and the amount of funds sent with the message.
    /// * `message`: The `message` parameter is of type `MsgUpdateClient` and contains the information
    /// needed to update a client in the Cosmos SDK. It includes the ID of the client to be updated and
    /// a signed header.
    ///
    /// Returns:
    ///
    /// This function returns a `Result<Response, ContractError>` where `Response` is a struct
    /// representing the response to a message and `ContractError` is an enum representing the possible
    /// errors that can occur during contract execution.
    fn update_client(
        &self,
        deps: DepsMut,
        _info: MessageInfo,
        message: RawMsgUpdateClient,
    ) -> Result<Response, ContractError> {
        let client_id = to_ibc_client_id(&message.client_id)?;
        let header = message.client_message.ok_or(ContractError::IbcClientError {
            error: ClientError::MissingRawHeader,
        })?;

        let client = self.get_light_client(deps.as_ref().storage, &client_id)?;
        let client_state = self.client_state(deps.as_ref(), &client_id)?;

        if client_state.is_frozen() {
            return Err(ClientError::ClientFrozen {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }

        self.store_callback_data(deps.storage, EXECUTE_UPDATE_CLIENT, &client_id)?;

        let sub_msg: SubMsg = client.update_client(&client_id, &header)?;
        cw_println!(
            deps,
            "Called Update Client On Lightclient for client id:{}",
            &message.client_id
        );
        Ok(Response::new()
            .add_submessage(sub_msg)
            .add_attribute("method", "update_client")
            .add_attribute("client_id", client_id.as_str()))
    }

    /// This method upgrades a client's state and consensus state and verifies proofs against the
    /// root.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier.
    /// * `info`: `info` is a `MessageInfo` struct that contains information about the message sender,
    /// such as their address and the amount of funds they sent with the message.
    /// * `message`: `message` is a struct of type `MsgUpgradeClient` which contains the necessary
    /// information to upgrade a client. It includes the `client_id` of the client to be upgraded, the
    /// new `client_state` and `consensus_state`, and the proofs for the upgrade.
    ///
    /// Returns:
    ///
    /// a `Result` object that contains either a `Response` or a `ContractError`.
    fn upgrade_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        message: RawMsgUpgradeClient,
    ) -> Result<Response, ContractError> {
        let client_id = to_ibc_client_id(&message.client_id)?;
        let old_client_state = self.client_state(deps.as_ref(), &client_id)?;

        let new_client_state = message.client_state.ok_or(ContractError::IbcClientError {
            error: ClientError::MissingRawClientState,
        })?;
        let new_consensus_state = message
            .consensus_state
            .ok_or(ContractError::IbcClientError {
                error: ClientError::MissingRawConsensusState,
            })?;

        if old_client_state.is_frozen() {
            return Err(ClientError::ClientFrozen {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }

        let old_consensus_state =
            self.consensus_state(deps.as_ref(), &client_id, &old_client_state.latest_height())?;

        let now = self.host_timestamp(&env)?;
        let duration = now
            .duration_since(&old_consensus_state.timestamp())
            .ok_or_else(|| ClientError::InvalidConsensusStateTimestamp {
                time1: old_consensus_state.timestamp(),
                time2: now,
            })
            .map_err(Into::<ContractError>::into)?;

        if old_client_state.expired(duration) {
            return Err(ClientError::HeaderNotWithinTrustPeriod {
                latest_time: old_consensus_state.timestamp(),
                update_time: now,
            })
            .map_err(Into::<ContractError>::into);
        };

        let wasm_exec_message = LightClientMessage::UpgradeClient {
            upgraded_client_state: new_client_state.value,
            upgraded_consensus_state: new_consensus_state.value,
            proof_upgrade_client: to_vec(&message.proof_upgrade_client)
                .map_err(ContractError::Std)?,
            proof_upgrade_consensus_state: to_vec(&message.proof_upgrade_consensus_state)
                .map_err(ContractError::Std)?,
        };

        let client = self.get_light_client(deps.storage, &client_id)?;

        let wasm_msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: client.get_address(),
            msg: to_binary(&wasm_exec_message).map_err(ContractError::Std)?,
            funds: info.funds,
        });

        let sub_message = SubMsg::reply_on_success(wasm_msg, EXECUTE_UPGRADE_CLIENT);

        Ok(Response::new()
            .add_submessage(sub_message)
            .add_attribute("method", "upgrade_client")
            .add_attribute("client_id", client_id.as_str()))
    }

    /// This method registers a light client with a given client type and stores it in the registry.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier. It is used to interact with the blockchain state
    /// and other modules.
    /// * `client_type`: The type of client being registered, which is of the enum type `ClientType`. It
    /// could be one of the following values: `Tendermint`, `CosmosSDK`, `Substrate`, `Solana`,
    /// `Ethereum`, `Bitcoin`, `Other`.
    /// * `light_client`: `light_client` is an `Addr` type parameter that represents the address of the
    /// light client being registered. It is used to identify the client and store it into the registry.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>`. If the function executes successfully, it will return an
    /// `Ok` variant containing a `Response` object with some attributes added to it. If there is an
    /// error, it will return an `Err` variant containing a `ContractError` object.
    fn register_client(
        &self,
        deps: DepsMut,
        client_type: IbcClientType,
        light_client: Addr,
    ) -> Result<Response, ContractError> {
        let light_client_address = light_client.to_string();

        self.check_client_registered(deps.as_ref().storage, client_type.clone())?;

        self.store_client_into_registry(deps.storage, client_type.clone(), light_client_address)?;

        Ok(Response::new()
            .add_attribute("method", "register_client")
            .add_attribute("client_type", client_type.as_str()))
    }

    /// This function generates a unique client identifier based on the client type and a client
    /// seq_on_a number stored in a storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is
    /// likely an interface for a storage system that the function uses to store and retrieve data. The
    /// specific implementation of this trait is not known from the code snippet provided.
    /// * `client_type`: The `client_type` parameter is a value of the `ClientType` enum that represents
    /// the type of client for which the identifier is being generated. The `ClientType` enum could have
    /// different variants such as `Individual`, `Corporate`, `Government`, etc.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing a `ClientId` if the function executes successfully,
    /// or a `ContractError` if an error occurs.
    fn generate_client_identifier(
        &self,
        store: &mut dyn Storage,
        client_type: IbcClientType,
    ) -> Result<ClientId, ContractError> {
        let client_seq_on_a = self.client_counter(store)?;
        let client_identifier = ClientId::new(client_type, client_seq_on_a)?;
        self.increase_client_counter(store)?;
        Ok(client_identifier)
    }

    /// The above code is implementing the `execute_update_client_reply` function for a Rust-based smart
    /// contract. This function is responsible for handling the result of a sub-message that updates the
    /// client state and consensus state of an IBC client. The function first checks if the sub-message
    /// was successful or not. If it was successful, it extracts the updated client and consensus states
    /// from the sub-message result and stores them in the contract's storage. It then generates an
    /// update client event and returns a response with this event and some additional attributes. If
    /// the sub-message was not successful, the function returns an error with a
    fn execute_update_client_reply(
        &self,
        deps: DepsMut,
        env: Env,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => match result.data {
                Some(data) => {
                    let update_client_response: UpdateClientResponse = from_binary_response(&data)?;
                    cw_println!(deps, "Received Client Update Callback with data");
                    let client_id: ClientId =
                        self.get_callback_data(deps.as_ref().storage, EXECUTE_UPDATE_CLIENT)?;
                    self.clear_callback_data(deps.storage, EXECUTE_UPDATE_CLIENT);
                    let height = update_client_response.height();

                    self.store_client_commitment(
                        deps.storage,
                        &env,
                        &client_id,
                        update_client_response.client_state_commitment.to_vec(),
                    )?;

                    self.store_consensus_commitment(
                        deps.storage,
                        &client_id,
                        height,
                        update_client_response.consensus_state_commitment.to_vec(),
                    )?;

                    let client_type = IbcClientType::from(client_id.clone());

                    let event = update_client_event(client_type, height, vec![height], &client_id);

                    Ok(Response::new()
                        .add_event(event)
                        .add_attribute("methods", "execute_update_client_reply")
                        .add_attribute("height", height))
                }
                None => Err(ClientError::Other {
                    description: "UNKNOWN ERROR".to_string(),
                })
                .map_err(Into::<ContractError>::into),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ClientError::Other { description: error }).map_err(Into::<ContractError>::into)
            }
        }
    }
    /// This function executes an upgrade client reply and stores the client and consensus state
    /// commitments.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract, which includes
    /// access to the storage, API, and other modules needed to execute the contract's logic.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module. It is used to handle the response from the sub-message and update the
    /// client and consensus state accordingly.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response from
    /// the contract and `ContractError` is an enum representing the possible errors that can occur
    /// during the execution of the function.
    fn execute_upgrade_client_reply(
        &self,
        deps: DepsMut,
        env: Env,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => match result.data {
                Some(data) => {
                    let response: UpgradeClientResponse =
                        from_binary(&data).map_err(ContractError::Std)?;
                    let client_id = response.client_id().map_err(ContractError::from)?;

                    self.store_client_commitment(
                        deps.storage,
                        &env,
                        &client_id,
                        response.client_state_commitment().to_vec(),
                    )?;

                    self.store_consensus_commitment(
                        deps.storage,
                        &client_id,
                        response.height(),
                        response.consensus_state_commitment().to_vec(),
                    )?;

                    let client_type = IbcClientType::from(client_id.clone());

                    let event =
                        upgrade_client_event(client_type, response.height(), client_id.clone());

                    Ok(Response::new()
                        .add_event(event)
                        .add_attribute("method", "execute_upgrade_client_reply")
                        .add_attribute("client_id", client_id.as_str()))
                }
                None => Err(ClientError::Other {
                    description: "Invalid Response Data".to_string(),
                })
                .map_err(Into::<ContractError>::into),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ClientError::Other { description: error }).map_err(Into::<ContractError>::into)
            }
        }
    }

    /// This function handles the submission of misbehaviour by a client.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides mutable access to the contract's
    /// dependencies such as the storage, API, and querier. It is used to interact with the blockchain
    /// and other contracts.
    /// * `info`: `info` is a parameter of type `MessageInfo` which contains information about the
    /// message being processed, such as the sender address, the amount of funds sent with the message,
    /// and the gas limit. It is used in this function to retrieve the funds sent with the message and
    /// pass them along to
    /// * `message`: `message` is a `MsgSubmitMisbehaviour` struct which contains the information about
    /// the misbehaviour being submitted by the client. It includes the client ID and the misbehaviour
    /// data.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing possible errors that can occur during
    /// contract execution.
    fn misbehaviour(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: RawMsgSubmitMisbehaviour,
    ) -> Result<Response, ContractError> {
        let client_id = to_ibc_client_id(&message.client_id)?;

        let client_state = self.client_state(deps.as_ref(), &client_id)?;

        if client_state.is_frozen() {
            return Err(ClientError::ClientFrozen {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }
        let client = self.get_light_client(deps.as_ref().storage, &client_id)?;

        let clinet_message = LightClientMessage::Misbehaviour {
            client_id: client_id.to_string(),
            misbehaviour: message.misbehaviour.unwrap().encode_to_vec(),
        };

        let wasm_exec_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: client.get_address(),
            msg: to_binary(&clinet_message)?,
            funds: info.funds,
        });

        let sub_message = SubMsg::reply_on_success(wasm_exec_message, MISBEHAVIOUR);

        Ok(Response::new()
            .add_submessage(sub_message)
            .add_attribute("method", "misbehaviour"))
    }

    /// This function handles the execution of a misbehaviour reply.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract, which includes
    /// access to the storage, API, and other modules needed to execute the contract's logic.
    /// * `message`: `message` is a `Reply` struct that contains the result of a previously executed
    /// submessage. It is used to extract the data from the submessage result and handle any errors that
    /// may have occurred.
    ///
    /// Returns:
    ///
    /// This function returns a `Result<Response, ContractError>` where `Response` is a struct
    /// representing the response from the contract and `ContractError` is an enum representing the
    /// possible errors that can occur during the execution of the function.
    fn execute_misbehaviour_reply(
        &self,
        deps: DepsMut,
        env: Env,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => match result.data {
                Some(response) => {
                    let misbehaviour_response = from_binary::<MisbehaviourResponse>(&response)?;

                    let client_id = misbehaviour_response
                        .client_id()
                        .map_err(ContractError::from)?;

                    let client_type =
                        IbcClientType::try_from(client_id.clone()).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: DecodeError::new(error.to_string()),
                            }
                        })?;

                    let event = client_misbehaviour_event(client_id.as_str(), client_type.as_str());

                    self.store_client_commitment(
                        deps.storage,
                        &env,
                        &client_id,
                        misbehaviour_response.client_state_commitment,
                    )?;

                    Ok(Response::new()
                        .add_event(event)
                        .add_attribute("method", "execute_misbheaviour_reply")
                        .add_attribute("client_id", client_id.as_str()))
                }
                None => Err(ClientError::Other {
                    description: "Invalid Response Data".to_string(),
                })
                .map_err(Into::<ContractError>::into),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ClientError::Other { description: error }).map_err(Into::<ContractError>::into)
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, to_binary};
    use cw_common::client_response::CreateClientResponse;
    #[test]
    fn test_binary_conversion() {
        let res: CreateClientResponse = CreateClientResponse::default();
        let bytes = to_binary(&res).unwrap();
        println!("{}", hex::encode(bytes.0.clone()));
        let decoded: CreateClientResponse = from_binary(&bytes).unwrap();
        assert_eq!(res, decoded)
    }
}
