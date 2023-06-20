use std::str::from_utf8;

use cw_common::{client_msg::VerifyConnectionPayload, from_binary_response, hex_string::HexString};
use debug_print::debug_println;
use prost::DecodeError;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    /// This method initializes a new connection between two clients in an IBC protocol implementation.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct, which provides access to the contract's dependencies such
    /// as storage, querier, and API. It is used to interact with the blockchain and other contracts.
    /// * `message`: The `message` parameter is of type `MsgConnectionOpenInit` and contains the
    /// information needed to initialize a new connection. It includes the client ID of the initiating
    /// party (`client_id_on_a`), the counterparty information (`counterparty`), the version of the
    /// connection protocol to be used (`
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can occur
    /// during contract execution.
    pub fn connection_open_init(
        &self,
        deps: DepsMut,
        message: MsgConnectionOpenInit,
    ) -> Result<Response, ContractError> {
        let client_id = message.client_id_on_a.clone();
        // self.check_for_connection(deps.as_ref().storage, client_id.clone())?;

        let connection_identifier = self.generate_connection_idenfier(deps.storage)?;

        self.client_state(deps.storage, &message.client_id_on_a)?;

        let lightclient_address = self.get_client(deps.as_ref().storage, client_id.clone())?;

        let query_message = cw_common::client_msg::QueryMsg::GetClientState {
            client_id: client_id.as_str().to_string(),
        };

        let query = QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: lightclient_address,
            msg: to_binary(&query_message).map_err(ContractError::Std)?,
        });

        let response: Vec<u8> = deps.querier.query(&query).map_err(ContractError::Std)?;

        if response.is_empty() {
            return Err(ClientError::ClientNotFound {
                client_id: message.client_id_on_a,
            })
            .map_err(Into::<ContractError>::into);
        }

        let versions = match message.version {
            Some(version) => {
                if self.get_compatible_versions().contains(&version) {
                    vec![version]
                } else {
                    return Err(ConnectionError::EmptyVersions)
                        .map_err(Into::<ContractError>::into);
                }
            }
            None => self.get_compatible_versions(),
        };

        let connection_end: ConnectionEnd = ConnectionEnd::new(
            State::Init,
            message.client_id_on_a,
            message.counterparty.clone(),
            versions,
            message.delay_period,
        );

        self.update_connection_commitment(
            deps.storage,
            connection_identifier.clone(),
            connection_end.clone(),
        )?;
        self.store_connection_to_client(
            deps.storage,
            client_id.clone(),
            connection_identifier.clone(),
        )?;
        self.store_connection(deps.storage, connection_identifier.clone(), connection_end)?;

        let event = create_open_init_event(
            connection_identifier.as_str(),
            client_id.as_str(),
            message.counterparty.client_id().as_str(),
        );

        return Ok(Response::new()
            .add_event(event)
            .add_attribute("method", "connection_open_init")
            .add_attribute("connection_id", connection_identifier.as_str()));
    }

    /// This method generates a unique connection identifier by incrementing a counter and returning a
    /// new ConnectionId.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is
    /// likely a storage implementation that allows the contract to persist data on the blockchain. The
    /// `generate_connection_identifier` function uses this storage to retrieve and update a counter
    /// value, which is used to generate a unique `
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing a `ConnectionId` if the function executes
    /// successfully, or a `ContractError` if an error occurs.
    pub fn generate_connection_idenfier(
        &self,
        store: &mut dyn Storage,
    ) -> Result<ConnectionId, ContractError> {
        let counter = self.connection_counter(store)?;
        let connection_id = ConnectionId::new(counter);
        self.increase_connection_counter(store)?;

        Ok(connection_id)
    }
    pub fn get_compatible_versions(&self) -> Vec<Version> {
        vec![Version::default()]
    }
    /// This method handles the processing of a connection open acknowledgement message in a IBC contract.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier. It is used to interact with the blockchain and
    /// other contracts.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// sender, such as their address and the amount of funds they sent with the message.
    /// * `msg`: The `msg` parameter is of type `MsgConnectionOpenAck` and contains the information
    /// needed to acknowledge the opening of a connection between two chains in the IBC protocol. It
    /// includes the connection ID on chain A, the consensus height of chain A on chain B, the client
    /// state of chain A
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing possible errors that can occur during the
    /// execution of the function.
    pub fn connection_open_ack(
        &self,

        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        msg: MsgConnectionOpenAck,
    ) -> Result<Response, ContractError> {
        debug_println!("[ConnOpenAck]: Connection Open Ack");
        let host_height = self
            .host_height(&env)
            .map_err(|_| ConnectionError::Other {
                description: "failed to get host height".to_string(),
            })
            .map_err(Into::<ContractError>::into)?;
        debug_println!("[ConnOpenAck]: Host Height {:?}", host_height);
        if msg.consensus_height_of_a_on_b > host_height {
            return Err(ConnectionError::InvalidConsensusHeight {
                target_height: msg.consensus_height_of_a_on_b,
                current_height: host_height,
            })
            .map_err(Into::<ContractError>::into);
        }
        debug_println!("[ConnOpenAck]:Consensus Height Valid");

        self.validate_self_client(msg.client_state_of_a_on_b.clone())?;
        debug_println!("[ConnOpenAck]: Self Client Valid");
        let conn_end_on_a = self.connection_end(deps.storage, msg.conn_id_on_a.clone())?;
        let client_id_on_a = conn_end_on_a.client_id();
        let client_id_on_b = conn_end_on_a.counterparty().client_id();

        if !(conn_end_on_a.state_matches(&State::Init)
            && conn_end_on_a.versions().contains(&msg.version))
        {
            return Err(ConnectionError::ConnectionMismatch {
                connection_id: msg.conn_id_on_a,
            })
            .map_err(Into::<ContractError>::into);
        }

        debug_println!("[ConnOpenAck]: State Matched");

        let client_cons_state_path_on_a =
            self.consensus_state(deps.storage, client_id_on_a, &msg.proofs_height_on_b)?;

        let consensus_state_of_b_on_a = self
            .consensus_state(deps.storage, client_id_on_a, &msg.proofs_height_on_b)
            .map_err(|_| ConnectionError::Other {
                description: "failed to fetch consensus state".to_string(),
            })
            .map_err(Into::<ContractError>::into)?;
        let prefix_on_a = self.commitment_prefix(deps.as_ref(), &env);
        let prefix_on_b = conn_end_on_a.counterparty().prefix();
        let client_address = self.get_client(deps.as_ref().storage, client_id_on_a.clone())?;

        let expected_conn_end_on_b: ConnectionEnd = ConnectionEnd::new(
            State::TryOpen,
            client_id_on_b.clone(),
            Counterparty::new(
                client_id_on_a.clone(),
                Some(msg.conn_id_on_a.clone()),
                prefix_on_a,
            ),
            vec![msg.version.clone()],
            conn_end_on_a.delay_period(),
        );
        debug_println!(
            "[ConnOpenAck]: Expected conn end {:?}",
            expected_conn_end_on_b
        );
        let connection_path = commitment::connection_path(&msg.conn_id_on_b);
        let verify_connection_state = VerifyConnectionState::new(
            msg.proofs_height_on_b.to_string(),
            to_vec(&prefix_on_b)?,
            msg.proof_conn_end_on_b.into(),
            consensus_state_of_b_on_a.root().as_bytes().to_vec(),
            connection_path,
            expected_conn_end_on_b.encode_vec().unwrap(),
        );

        let client_state_path = commitment::client_state_path(client_id_on_b);
        let verify_client_full_state = VerifyClientFullState::new(
            msg.proofs_height_on_b.to_string(),
            to_vec(&prefix_on_b)?,
            msg.proof_client_state_of_a_on_b.into(),
            consensus_state_of_b_on_a.root().as_bytes().to_vec(),
            client_state_path,
            msg.client_state_of_a_on_b.value.clone(),
        );

        let consensus_state_path_on_b =
            commitment::consensus_state_path(client_id_on_b, &msg.consensus_height_of_a_on_b);
        let verify_client_consensus_state = VerifyClientConsensusState::new(
            msg.proofs_height_on_b.to_string(),
            to_vec(&prefix_on_b)?,
            msg.proof_consensus_state_of_a_on_b.into(),
            consensus_state_of_b_on_a.root().as_bytes().to_vec(),
            consensus_state_path_on_b,
            client_cons_state_path_on_a.clone().as_bytes(),
        );
        let payload = VerifyConnectionPayload::<OpenAckResponse> {
            client_id: client_id_on_a.to_string(),
            verify_connection_state,
            verify_client_full_state,
            verify_client_consensus_state,
            expected_response: OpenAckResponse {
                conn_id: msg.conn_id_on_a.to_string(),
                version: serde_json_wasm::to_vec(&msg.version).unwrap(),
                counterparty_client_id: client_id_on_b.clone().to_string(),
                counterparty_connection_id: msg.conn_id_on_b.to_string(),
                counterparty_prefix: prefix_on_b.as_bytes().to_vec(),
            },
        };
        let client_message =
            crate::ics04_channel::LightClientMessage::VerifyConnectionOpenAck(payload);

        let wasm_execute_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: client_address,
            msg: to_binary(&client_message)?,
            funds: info.funds,
        });
        debug_println!("[ConnOpenAck]: Validating On Light Client");

        let sub_message = SubMsg::reply_always(wasm_execute_message, EXECUTE_CONNECTION_OPENACK);

        Ok(Response::new()
            .add_submessage(sub_message)
            .add_attribute("method", "connection_open_ack"))
    }

    /// This method executes the opening acknowledgement of an IBC connection and updates the
    /// connection state accordingly.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, querier, and API interfaces.
    /// * `message`: `message` is a `Reply` struct that contains the result of a submessage sent by the
    /// contract. It is used to extract the data returned by the submessage and process it accordingly.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to be
    /// returned by the contract and `ContractError` is an enum representing the possible errors that can
    /// occur during the execution of the function.
    pub fn execute_connection_open_ack(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        debug_println!("[ConnOpenAckReply]: Open Ack Reply");
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => match result.data {
                Some(data) => {
                    let response: OpenAckResponse =
                        from_binary_response(&data).map_err(ContractError::Std)?;
                    debug_println!("[ConnOpenAckReply]: Response Decoded {:?}", response);

                    let connection_id =
                        IbcConnectionId::from_str(&response.conn_id).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: DecodeError::new(error.to_string()),
                            }
                        })?;

                    let version: Version =
                        serde_json_wasm::from_slice(&response.version).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: DecodeError::new(error.to_string()),
                            }
                        })?;

                    let mut conn_end = self.connection_end(deps.storage, connection_id.clone())?;

                    if !conn_end.state_matches(&State::Init) {
                        return Err(ConnectionError::ConnectionMismatch { connection_id })
                            .map_err(Into::<ContractError>::into);
                    }
                    debug_println!("[ConnOpenAckReply]: Conn end state matches");
                    let counter_party_client_id =
                        ClientId::from_str(&response.counterparty_client_id).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: DecodeError::new(error.to_string()),
                            }
                        })?;

                    let counterparty_conn_id = match response.counterparty_connection_id.is_empty()
                    {
                        true => None,
                        false => {
                            let connection_id =
                                IbcConnectionId::from_str(&response.counterparty_connection_id)
                                    .unwrap();
                            Some(connection_id)
                        }
                    };

                    let counterparty_prefix =
                        CommitmentPrefix::try_from(response.counterparty_prefix)
                            .map_err(|error| ConnectionError::Other {
                                description: error.to_string(),
                            })
                            .map_err(Into::<ContractError>::into)?;

                    let counterparty = Counterparty::new(
                        counter_party_client_id,
                        counterparty_conn_id.clone(),
                        counterparty_prefix,
                    );

                    conn_end.set_state(State::Open);
                    conn_end.set_version(version);
                    conn_end.set_counterparty(counterparty.clone());

                    let counter_conn_id = counterparty_conn_id.unwrap();

                    let event = create_open_ack_event(
                        connection_id.clone(),
                        conn_end.client_id().clone(),
                        counter_conn_id,
                        counterparty.client_id().clone(),
                    );

                    self.store_connection(deps.storage, connection_id.clone(), conn_end.clone())
                        .unwrap();
                    debug_println!("[ConnOpenAckReply]: Connection Stored");

                    self.update_connection_commitment(
                        deps.storage,
                        connection_id.clone(),
                        conn_end,
                    )
                    .unwrap();

                    Ok(Response::new()
                        .add_attribute("method", "execute_connection_open_ack")
                        .add_attribute("connection_id", connection_id.as_str())
                        .add_event(event))
                }
                None => Err(ConnectionError::Other {
                    description: "UNKNOWN ERROR".to_string(),
                })
                .map_err(Into::<ContractError>::into),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ConnectionError::Other { description: error })
                    .map_err(Into::<ContractError>::into)
            }
        }
    }
    /// This method handles the opening of a connection open try between two IBC clients.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier. It is used to interact with the blockchain and
    /// other contracts.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// sender, such as their address and the amount of funds they sent with the message.
    /// * `message`: `message` is a `MsgConnectionOpenTry` struct which contains the information needed
    /// to try opening a connection between two chains in the IBC protocol. It includes the client ID of
    /// the counterparty chain, the connection ID of the counterparty chain, the prefix of the
    /// counterparty chain, the
    ///
    /// Returns:
    ///
    /// This function returns a `Result<Response, ContractError>` where `Response` is a struct
    /// representing the response to a contract execution and `ContractError` is an enum representing the
    /// possible errors that can occur during contract execution.
    pub fn connection_open_try(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        message: MsgConnectionOpenTry,
    ) -> Result<Response, ContractError> {
        //TODO validate

        self.validate_self_client(message.client_state_of_b_on_a.clone())?;
        let host_height = self
            .host_height(&env)
            .map_err(|_| ConnectionError::Other {
                description: "failed to get host height".to_string(),
            })
            .map_err(Into::<ContractError>::into)?;
        if message.consensus_height_of_b_on_a > host_height {
            return Err(ConnectionError::InvalidConsensusHeight {
                target_height: message.consensus_height_of_b_on_a,
                current_height: host_height,
            })
            .map_err(Into::<ContractError>::into);
        }
        let prefix_on_a = message.counterparty.prefix().clone();
        let prefix_on_b = self.commitment_prefix(deps.as_ref(), &env);
        let client_id_on_b = message.client_id_on_b.clone();

        debug_println!(
            "prefix_on_b is {:?}",
            from_utf8(&prefix_on_b.clone().into_vec()).unwrap()
        );

        let client_address = self.get_client(deps.as_ref().storage, client_id_on_b.clone())?;

        // no idea what is this  is this suppose to be like this ?????
        let client_consensus_state_path_on_b = commitment::consensus_state_path(
            &message.client_id_on_b,
            &message.consensus_height_of_b_on_a,
        );
        let expected_conn_end_on_a = ConnectionEnd::new(
            State::Init,
            message.counterparty.client_id().clone(),
            Counterparty::new(message.client_id_on_b.clone(), None, prefix_on_b),
            message.versions_on_a.clone(),
            message.delay_period,
        );

        debug_println!("expected_connection_end {:?}", expected_conn_end_on_a);
        let actual_byte = hex::decode("0a0f30372d74656e6465726d696e742d3012230a0131120d4f524445525f4f524445524544120f4f524445525f554e4f524445524544180122400a0c69636f6e636c69656e742d301a300a2e034a9dcdcdcdaa5f2948b317265cc19b96f135b34de0ca4f65d9597546ae33d07a000b636f6d6d69746d656e7473").unwrap();
        let actual_connection = ConnectionEnd::decode(actual_byte.as_slice()).unwrap();

        debug_println!("actual connection {:?}", actual_connection);

        let consensus_state_of_a_on_b = self
            .consensus_state(
                deps.storage,
                &message.client_id_on_b,
                &message.proofs_height_on_a,
            )
            .map_err(|_| ConnectionError::Other {
                description: "failed to fetch consensus state".to_string(),
            })
            .map_err(Into::<ContractError>::into)?;

        let connection_path =
            commitment::connection_path(&message.counterparty.connection_id.clone().unwrap());
        debug_println!(
            "[ConnOpenTry]: connkey: {:?}",
            HexString::from_bytes(&connection_path)
        );
        debug_println!(
            "[ConnOpenTry]: root: {:?} ",
            HexString::from_bytes(consensus_state_of_a_on_b.root().as_bytes())
        );

        debug_println!(
            "[ConnOpenTry]: expected counterpart connection_end:{:?}",
            HexString::from_bytes(&expected_conn_end_on_a.encode_vec().unwrap())
        );

        let verify_connection_state = VerifyConnectionState::new(
            message.proofs_height_on_a.to_string(),
            to_vec(&prefix_on_a).map_err(ContractError::Std)?,
            message.proof_conn_end_on_a.into(),
            consensus_state_of_a_on_b.root().as_bytes().to_vec(),
            connection_path,
            expected_conn_end_on_a.encode_vec().unwrap().to_vec(),
        );

        // this is verifying tendermint client state and shouldn't have icon-client as an argument
        debug_println!(
            "[ConnOpenTry]: payload client state path {:?}",
            &message.counterparty.client_id()
        );
        let client_state_path = commitment::client_state_path(message.counterparty.client_id());
        debug_println!(
            "[ConnOpenTry]: the clientstate value is  {:?}",
            message.client_state_of_b_on_a.value.clone()
        );
        let verify_client_full_state = VerifyClientFullState::new(
            message.proofs_height_on_a.to_string(),
            to_vec(&prefix_on_a).map_err(ContractError::Std)?,
            message.proof_client_state_of_b_on_a.into(),
            consensus_state_of_a_on_b.root().as_bytes().to_vec(),
            client_state_path,
            message.client_state_of_b_on_a.value.clone().to_vec(),
        );

        let consensus_state_path_on_a = commitment::consensus_state_path(
            &message.client_id_on_b,
            &message.consensus_height_of_b_on_a,
        );
        let verify_client_consensus_state = VerifyClientConsensusState::new(
            message.proofs_height_on_a.to_string(),
            to_vec(&prefix_on_a).map_err(ContractError::Std)?,
            message.proof_consensus_state_of_b_on_a.into(),
            consensus_state_of_a_on_b.root().as_bytes().to_vec(),
            consensus_state_path_on_a,
            client_consensus_state_path_on_b,
        );

        let payload = VerifyConnectionPayload::<OpenTryResponse> {
            client_id: client_id_on_b.to_string(),
            verify_connection_state,
            verify_client_full_state,
            verify_client_consensus_state,
            expected_response: OpenTryResponse {
                conn_id: "".to_string(),
                client_id: client_id_on_b.to_string(),
                counterparty_client_id: message.counterparty.client_id().clone().to_string(),

                counterparty_connection_id: message
                    .counterparty
                    .connection_id()
                    .map(|c| c.to_string())
                    .unwrap_or("".to_string()),
                counterparty_prefix: message.counterparty.prefix().as_bytes().to_vec(),
                versions: serde_json_wasm::to_vec(&message.versions_on_a.clone()).unwrap(),
                delay_period: message.delay_period.as_secs(),
            },
        };

        let client_message =
            crate::ics04_channel::LightClientMessage::VerifyConnectionOpenTry(payload);

        let wasm_execute_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: client_address,
            msg: to_binary(&client_message).unwrap(),
            funds: info.funds,
        });

        let sub_message = SubMsg::reply_always(wasm_execute_message, EXECUTE_CONNECTION_OPENTRY);

        Ok(Response::new()
            .add_submessage(sub_message)
            .add_attribute("method", "connection_open_try"))
    }

    /// The below code is implementing a function `execute_connection_open_try` that handles the result
    /// of a submessage sent to open a connection in an IBC (Inter-Blockchain Communication) protocol.
    /// It extracts the relevant information from the submessage result, such as the counterparty client
    /// ID, connection ID, and commitment prefix, and uses them to create a new connection end. It then
    /// stores the connection end in the contract's storage and returns a response with relevant
    /// attributes and events. If there is an error in the submessage result, it returns an error with a
    /// description of the error.
    pub fn execute_connection_open_try(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        debug_println!("[ConnOpenTryReply]:open try repluy ");
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => match result.data {
                Some(data) => {
                    debug_println!("[ConnOpenTryReply]: Res is ok ");

                    let response: OpenTryResponse =
                        from_binary_response(&data).map_err(ContractError::Std)?;

                    debug_println!("[ConnOpenTryReply]: Response parsed");

                    let counter_party_client_id =
                        ClientId::from_str(&response.counterparty_client_id).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: DecodeError::new(error.to_string()),
                            }
                        })?;

                    debug_println!(
                        "[ConnOpenTryReply]: counter_party_client_id id {:?}",
                        counter_party_client_id
                    );

                    let counterparty_conn_id = match response.counterparty_connection_id.is_empty()
                    {
                        true => None,
                        false => {
                            let connection_id =
                                ConnectionId::from_str(&response.counterparty_connection_id)?;
                            Some(connection_id)
                        }
                    };

                    debug_println!(
                        "[ConnOpenTryReply]: counterparty conn id  {:?}",
                        counterparty_conn_id
                    );

                    let counterparty_prefix =
                        CommitmentPrefix::try_from(response.counterparty_prefix)
                            .map_err(|error| ConnectionError::Other {
                                description: error.to_string(),
                            })
                            .map_err(Into::<ContractError>::into)?;

                    debug_println!(
                        "[ConnOpenTryReply]: counterparty_prefix {:?}",
                        counterparty_prefix
                    );

                    let counterparty = Counterparty::new(
                        counter_party_client_id,
                        counterparty_conn_id.clone(),
                        counterparty_prefix,
                    );

                    debug_println!(
                        "[ConnOpenTryReply]: respnose version {:?}",
                        HexString::from_bytes(&response.versions)
                    );
                    let version: Vec<Version> = serde_json_wasm::from_slice(&response.versions)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: DecodeError::new(error.to_string()),
                        })?;

                    debug_println!("[ConnOpenTryReply]: version decode{:?}", version);

                    let delay_period = Duration::from_secs(response.delay_period);

                    let client_id = ClientId::from_str(&response.client_id).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: DecodeError::new(error.to_string()),
                        }
                    })?;

                    debug_println!("[ConnOpenTryReply]: client id is{:?}", client_id);

                    let connection_id = self.generate_connection_idenfier(deps.storage)?;

                    debug_println!("[ConnOpenTryReply]: connection id is{:?}", connection_id);

                    let conn_end = ConnectionEnd::new(
                        State::TryOpen,
                        client_id.clone(),
                        counterparty,
                        version,
                        delay_period,
                    );

                    debug_println!("[ConnOpenTryReply]: conn end{:?}", conn_end);

                    let counterparty_client_id =
                        ClientId::from_str(&response.counterparty_client_id).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: DecodeError::new(error.to_string()),
                            }
                        })?;
                    debug_println!(
                        "[ConnOpenTryReply]: counterparty client id {:?}",
                        counterparty_client_id
                    );
                    debug_println!(
                        "[ConnOpenTryReply]: Response connection id  {:?}",
                        &response.counterparty_connection_id
                    );

                    let event = create_open_try_event(
                        connection_id.clone(),
                        client_id.clone(),
                        counterparty_conn_id,
                        counterparty_client_id,
                    );
                    self.store_connection_to_client(
                        deps.storage,
                        client_id,
                        connection_id.clone(),
                    )?;
                    self.store_connection(deps.storage, connection_id.clone(), conn_end.clone())
                        .unwrap();

                    self.update_connection_commitment(
                        deps.storage,
                        connection_id.clone(),
                        conn_end,
                    )
                    .unwrap();

                    Ok(Response::new()
                        .add_attribute("method", "execute_connection_open_try")
                        .add_attribute("connection_id", connection_id.as_str())
                        .add_event(event))
                }
                None => Err(ConnectionError::Other {
                    description: "UNKNOWN ERROR".to_string(),
                })
                .map_err(Into::<ContractError>::into),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ConnectionError::Other { description: error })
                    .map_err(Into::<ContractError>::into)
            }
        }
    }
    /// This function handles the confirmation of an open connection between two parties in an IBC
    /// protocol implementation.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// sender, such as the sender's address and the amount of funds sent with the message.
    /// * `msg`: `msg` is a `MsgConnectionOpenConfirm` struct which contains the following fields:
    ///
    /// Returns:
    ///
    /// A `Result<Response, ContractError>` is being returned.
    pub fn connection_open_confirm(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: MsgConnectionOpenConfirm,
    ) -> Result<Response, ContractError> {
        debug_println!("[ConnOpenConfirm]: Connection Open Confirm");
        let conn_end_on_b = self.connection_end(deps.storage, msg.conn_id_on_b.clone())?;
        debug_println!("[ConnOpenConfirm]: Our Connection {:?}", conn_end_on_b);
        let client_id_on_b = conn_end_on_b.client_id();
        let client_id_on_a = conn_end_on_b.counterparty().client_id();
        debug_println!("");
        if !conn_end_on_b.state_matches(&State::TryOpen) {
            return Err(ConnectionError::ConnectionMismatch {
                connection_id: msg.conn_id_on_b,
            })
            .map_err(Into::<ContractError>::into);
        }

        debug_println!("Connection State Matched");

        // let _client_state_of_a_on_b = self
        //     .client_state(deps.storage, client_id_on_b)
        //     .map_err(|_| ConnectionError::Other {
        //         description: "failed to fetch client state".to_string(),
        //     })
        //     .map_err(|e| Into::<ContractError>::into(e))?;
        debug_println!("Client State Decoded");

        let _client_cons_state_path_on_b =
            self.consensus_state(deps.storage, client_id_on_b, &msg.proof_height_on_a)?;
        debug_println!("[ConnOpenConfirm]: Consensus State Path Decoded");
        let consensus_state_of_a_on_b = self
            .consensus_state(deps.storage, client_id_on_b, &msg.proof_height_on_a)
            .map_err(|_| ConnectionError::Other {
                description: "failed to fetch consensus state".to_string(),
            })
            .map_err(Into::<ContractError>::into)?;
        debug_println!("Consensus State Decoded");

        let prefix_on_a = conn_end_on_b.counterparty().prefix();
        let prefix_on_b = self.commitment_prefix(deps.as_ref(), &env);

        let client_address = self.get_client(deps.as_ref().storage, client_id_on_b.clone())?;

        let expected_conn_end_on_a = ConnectionEnd::new(
            State::Open,
            client_id_on_a.clone(),
            Counterparty::new(
                client_id_on_b.clone(),
                Some(msg.conn_id_on_b.clone()),
                prefix_on_b.clone(),
            ),
            conn_end_on_b.versions().to_vec(),
            conn_end_on_b.delay_period(),
        );

        let connection_path =
            commitment::connection_path(conn_end_on_b.counterparty().connection_id().unwrap());
        let verify_connection_state = VerifyConnectionState::new(
            msg.proof_height_on_a.to_string(),
            to_vec(&prefix_on_a).map_err(ContractError::Std)?,
            msg.proof_conn_end_on_a.into(),
            consensus_state_of_a_on_b.root().as_bytes().to_vec(),
            connection_path,
            expected_conn_end_on_a.encode_vec().unwrap(),
        );

        debug_println!("Verify Connection State {:?}", verify_connection_state);
        let client_message = crate::ics04_channel::LightClientMessage::VerifyOpenConfirm {
            client_id: client_id_on_b.to_string(),
            verify_connection_state,
            expected_response: OpenConfirmResponse {
                conn_id: msg.conn_id_on_b.clone().to_string(),
                counterparty_client_id: client_id_on_a.to_string(),
                counterparty_connection_id: msg.conn_id_on_b.clone().to_string(),
                counterparty_prefix: prefix_on_b.as_bytes().to_vec(),
            },
        };

        let wasm_execute_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: client_address,
            msg: to_binary(&client_message).unwrap(),
            funds: info.funds,
        });

        let sub_message =
            SubMsg::reply_always(wasm_execute_message, EXECUTE_CONNECTION_OPENCONFIRM);
        debug_println!("[ConnOpenConfirm]: Verify to light client");
        Ok(Response::new()
            .add_submessage(sub_message)
            .add_attribute("method", "connection_open_confirm"))
    }

    /// This method executes the opening confirmation of an IBC connection and returns a response or an
    /// error.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, querier, and API interfaces. The `DepsMut`
    /// object is used to interact with these dependencies and perform operations such as reading and
    /// writing data
    /// * `message`: `message` is a `Reply` struct that contains the result of a submessage sent by the
    /// contract. Specifically, it contains the result of a `connection openconfirm` submessage, which
    /// confirms the opening of a connection between two IBC-enabled blockchains. The function extracts
    /// relevant information from the
    ///
    /// Returns:
    ///
    /// This function returns a `Result<Response, ContractError>` where `Response` and `ContractError`
    /// are defined in the `cosmwasm_std` and `ibc` crates respectively.
    pub fn execute_connection_openconfirm(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        debug_println!("[ConnOpenConfirmReply]: OpenConfirm Reply Received");
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(result) => match result.data {
                Some(data) => {
                    let response: OpenConfirmResponse =
                        from_binary_response(&data).map_err(ContractError::Std)?;
                    debug_println!(
                        "[ConnOpenConfirmReply]: OpenConfirm Response Decoded {:?}",
                        response
                    );

                    let connection_id =
                        IbcConnectionId::from_str(&response.conn_id).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: DecodeError::new(error.to_string()),
                            }
                        })?;
                    debug_println!(
                        "[ConnOpenConfirmReply]: Parsed Connection Id {:?}",
                        connection_id
                    );
                    let mut conn_end = self.connection_end(deps.storage, connection_id.clone())?;
                    debug_println!(
                        "[ConnOpenConfirmReply]: Stored Connection End {:?}",
                        conn_end
                    );

                    if !conn_end.state_matches(&State::TryOpen) {
                        return Err(ConnectionError::ConnectionMismatch { connection_id })
                            .map_err(Into::<ContractError>::into);
                    }
                    debug_println!("[ConnOpenConfirmReply]: Stored Connection State Matched");
                    let counter_party_client_id =
                        ClientId::from_str(&response.counterparty_client_id).map_err(|error| {
                            ContractError::IbcDecodeError {
                                error: DecodeError::new(error.to_string()),
                            }
                        })?;
                    debug_println!(
                        "[ConnOpenConfirmReply]: CounterParty ClientId {:?}",
                        counter_party_client_id
                    );
                    let counterparty_conn_id = match response.counterparty_connection_id.is_empty()
                    {
                        true => None,
                        false => {
                            let connection_id =
                                IbcConnectionId::from_str(&response.counterparty_connection_id)
                                    .unwrap();
                            Some(connection_id)
                        }
                    };
                    debug_println!(
                        "[ConnOpenConfirmReply]: CounterParty ConnId {:?}",
                        counterparty_conn_id
                    );
                    let counterparty_prefix =
                        CommitmentPrefix::try_from(response.counterparty_prefix)
                            .map_err(|error| ConnectionError::Other {
                                description: error.to_string(),
                            })
                            .map_err(Into::<ContractError>::into)?;

                    let counterparty = Counterparty::new(
                        counter_party_client_id,
                        counterparty_conn_id.clone(),
                        counterparty_prefix,
                    );
                    debug_println!("[ConnOpenConfirmReply]: CounterParty  {:?}", counterparty);
                    conn_end.set_state(State::Open);

                    let counter_conn_id = counterparty_conn_id.unwrap();
                    debug_println!(
                        "[ConnOpenConfirmReply]: CounterParty ConnId {:?}",
                        counter_conn_id
                    );

                    let event = create_open_confirm_event(
                        connection_id.clone(),
                        conn_end.client_id().clone(),
                        counter_conn_id,
                        counterparty.client_id().clone(),
                    );

                    self.store_connection(deps.storage, connection_id.clone(), conn_end.clone())
                        .unwrap();
                    debug_println!("[ConnOpenConfirmReply]: Connection Stored");

                    self.update_connection_commitment(
                        deps.storage,
                        connection_id.clone(),
                        conn_end,
                    )
                    .unwrap();

                    debug_println!("[ConnOpenConfirmReply]: Commitment Stored Stored");

                    Ok(Response::new()
                        .add_attribute("method", "execute_connection_open_confirm")
                        .add_attribute("connection_id", connection_id.as_str())
                        .add_event(event))
                }
                None => Err(ConnectionError::Other {
                    description: "UNKNOWN ERROR".to_string(),
                })
                .map_err(Into::<ContractError>::into),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ConnectionError::Other { description: error })
                    .map_err(Into::<ContractError>::into)
            }
        }
    }
}
