use std::{str::from_utf8, time::Duration};

use cw_common::{
    client_msg::VerifyConnectionPayload,
    hex_string::HexString,
    raw_types::connection::{RawMsgConnectionOpenInit, RawMsgConnectionOpenTry},
};
use debug_print::debug_println;

use crate::conversions::{
    to_ibc_client_id, to_ibc_counterparty, to_ibc_height, to_ibc_version, to_ibc_versions,
};

use super::{event::create_connection_event, *};

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
        message: RawMsgConnectionOpenInit,
    ) -> Result<Response, ContractError> {
        let client_id = to_ibc_client_id(&message.client_id)?;

        let connection_identifier = self.generate_connection_idenfier(deps.storage)?;

        self.client_state(deps.storage, &client_id)?;

        let client = self.get_client(deps.as_ref().storage, client_id.clone())?;

        let response: Vec<u8> = client.get_client_state(deps.as_ref(), &client_id)?;

        let delay_period = Duration::from_nanos(message.delay_period);
        let ibc_version = to_ibc_version(message.version)?;
        let ibc_counterparty = to_ibc_counterparty(message.counterparty)?;

        if response.is_empty() {
            return Err(ClientError::ClientNotFound { client_id })
                .map_err(Into::<ContractError>::into);
        }

        let versions = match ibc_version {
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
            client_id.clone(),
            ibc_counterparty.clone(),
            versions,
            delay_period,
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

        let event = create_connection_event(
            IbcEventType::OpenInitConnection,
            &connection_identifier,
            &client_id,
            ibc_counterparty.client_id(),
            None,
        )?;

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
        _info: MessageInfo,
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
            return Err(ContractError::IbcConnectionError {
                error: ConnectionError::ConnectionMismatch {
                    connection_id: msg.conn_id_on_a,
                },
            });
        }

        debug_println!("[ConnOpenAck]: State Matched");

        let client_cons_state_path_on_a =
            self.consensus_state(deps.storage, client_id_on_a, &msg.proofs_height_on_b)?;

        let consensus_state_of_b_on_a = self
            .consensus_state(deps.storage, client_id_on_a, &msg.proofs_height_on_b)
            .map_err(|_| ContractError::IbcConnectionError {
                error: ConnectionError::Other {
                    description: "failed to fetch consensus state".to_string(),
                },
            })?;
        let prefix_on_a = self.commitment_prefix(deps.as_ref(), &env);
        let prefix_on_b = conn_end_on_a.counterparty().prefix();
        let client = self.get_client(deps.as_ref().storage, client_id_on_a.clone())?;

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
        let payload = VerifyConnectionPayload {
            client_id: client_id_on_a.to_string(),
            verify_connection_state,
            verify_client_full_state,
            verify_client_consensus_state,
        };

        client.verify_connection_open_ack(deps.as_ref(), payload)?;

        let connection_id = msg.conn_id_on_a.clone();

        let version: Version = msg.version.clone();

        let mut conn_end = self.connection_end(deps.storage, connection_id.clone())?;

        if !conn_end.state_matches(&State::Init) {
            return Err(ConnectionError::ConnectionMismatch { connection_id })
                .map_err(Into::<ContractError>::into);
        }
        debug_println!("[ConnOpenAckReply]: Conn end state matches");
        let counter_party_client_id = client_id_on_b.clone();

        let counterparty_conn_id = msg.conn_id_on_b;

        let counterparty_prefix = prefix_on_b.clone();

        let counterparty = Counterparty::new(
            counter_party_client_id,
            Some(counterparty_conn_id.clone()),
            counterparty_prefix,
        );

        conn_end.set_state(State::Open);
        conn_end.set_version(version);
        conn_end.set_counterparty(counterparty.clone());

        let event = create_connection_event(
            IbcEventType::OpenAckConnection,
            &connection_id,
            &conn_end.client_id().clone(),
            counterparty.client_id(),
            Some(counterparty_conn_id),
        )?;

        self.store_connection(deps.storage, connection_id.clone(), conn_end.clone())?;
        debug_println!("[ConnOpenAckReply]: Connection Stored");

        self.update_connection_commitment(deps.storage, connection_id.clone(), conn_end)?;

        Ok(Response::new()
            .add_attribute("method", "execute_connection_open_ack")
            .add_attribute("connection_id", connection_id.as_str())
            .add_event(event))
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
        _info: MessageInfo,
        env: Env,
        message: RawMsgConnectionOpenTry,
    ) -> Result<Response, ContractError> {
        let message_client_state =
            message
                .client_state
                .ok_or(ContractError::IbcConnectionError {
                    error: ConnectionError::MissingClientState,
                })?;
        let message_client_id = to_ibc_client_id(&message.client_id)?;
        let message_consensus_height = to_ibc_height(message.consensus_height)?;
        let proof_height = to_ibc_height(message.proof_height)?;
        let ibc_counterparty = to_ibc_counterparty(message.counterparty)?;
        let message_versions = to_ibc_versions(message.counterparty_versions)?;
        let message_delay_period = Duration::from_nanos(message.delay_period);
        self.validate_self_client(message_client_state.clone())?;

        let host_height =
            self.host_height(&env)
                .map_err(|_| ContractError::IbcConnectionError {
                    error: ConnectionError::Other {
                        description: "failed to get host height".to_string(),
                    },
                })?;

        if message_consensus_height > host_height {
            return Err(ContractError::IbcConnectionError {
                error: ConnectionError::InvalidConsensusHeight {
                    target_height: message_consensus_height,
                    current_height: host_height,
                },
            });
        }
        let prefix_on_a = ibc_counterparty.prefix().clone();
        let prefix_on_b = self.commitment_prefix(deps.as_ref(), &env);

        debug_println!(
            "prefix_on_b is {:?}",
            from_utf8(&prefix_on_b.clone().into_vec()).unwrap()
        );

        let client = self.get_client(deps.as_ref().storage, message_client_id.clone())?;

        // no idea what is this  is this suppose to be like this ?????
        let client_consensus_state_path_on_b =
            commitment::consensus_state_path(&message_client_id, &message_consensus_height);
        let expected_conn_end_on_a = ConnectionEnd::new(
            State::Init,
            ibc_counterparty.client_id().clone(),
            Counterparty::new(message_client_id.clone(), None, prefix_on_b),
            message_versions.clone(),
            message_delay_period,
        );

        debug_println!("expected_connection_end {:?}", expected_conn_end_on_a);

        let consensus_state_of_a_on_b = self
            .consensus_state(deps.storage, &message_client_id, &proof_height)
            .map_err(|_| ContractError::IbcConnectionError {
                error: ConnectionError::Other {
                    description: "failed to fetch consensus state".to_string(),
                },
            })?;

        let connection_path =
            commitment::connection_path(&ibc_counterparty.connection_id.clone().unwrap());
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
            proof_height.to_string(),
            to_vec(&prefix_on_a).map_err(ContractError::Std)?,
            message.proof_init,
            consensus_state_of_a_on_b.root().as_bytes().to_vec(),
            connection_path,
            expected_conn_end_on_a.encode_vec().unwrap().to_vec(),
        );

        // this is verifying tendermint client state and shouldn't have icon-client as an argument
        debug_println!(
            "[ConnOpenTry]: payload client state path {:?}",
            &ibc_counterparty.client_id()
        );
        let client_state_path = commitment::client_state_path(ibc_counterparty.client_id());
        debug_println!(
            "[ConnOpenTry]: the clientstate value is  {:?}",
            message_client_state.value
        );
        let verify_client_full_state = VerifyClientFullState::new(
            proof_height.to_string(),
            to_vec(&prefix_on_a).map_err(ContractError::Std)?,
            message.proof_client,
            consensus_state_of_a_on_b.root().as_bytes().to_vec(),
            client_state_path,
            message_client_state.value.to_vec(),
        );

        let consensus_state_path_on_a =
            commitment::consensus_state_path(&message_client_id, &message_consensus_height);
        let verify_client_consensus_state = VerifyClientConsensusState::new(
            proof_height.to_string(),
            to_vec(&prefix_on_a).map_err(ContractError::Std)?,
            message.proof_consensus,
            consensus_state_of_a_on_b.root().as_bytes().to_vec(),
            consensus_state_path_on_a,
            client_consensus_state_path_on_b,
        );

        let payload = VerifyConnectionPayload {
            client_id: message_client_id.to_string(),
            verify_connection_state,
            verify_client_full_state,
            verify_client_consensus_state,
        };

        client.verify_connection_open_try(deps.as_ref(), payload)?;

        let counter_party_client_id = ibc_counterparty.client_id().clone();

        debug_println!(
            "[ConnOpenTryReply]: counter_party_client_id id {:?}",
            &counter_party_client_id
        );

        let counterparty_conn_id = ibc_counterparty.connection_id().cloned();

        debug_println!(
            "[ConnOpenTryReply]: counterparty conn id  {:?}",
            counterparty_conn_id
        );

        let counterparty_prefix = ibc_counterparty.prefix().clone();

        debug_println!(
            "[ConnOpenTryReply]: counterparty_prefix {:?}",
            counterparty_prefix
        );

        let counterparty = Counterparty::new(
            counter_party_client_id.clone(),
            counterparty_conn_id.clone(),
            counterparty_prefix,
        );

        let version: Vec<Version> = message_versions;

        debug_println!("[ConnOpenTryReply]: version decode{:?}", version);

        debug_println!("[ConnOpenTryReply]: client id is{:?}", message_client_id);

        let connection_id = self.generate_connection_idenfier(deps.storage)?;

        debug_println!("[ConnOpenTryReply]: connection id is{:?}", connection_id);

        let conn_end = ConnectionEnd::new(
            State::TryOpen,
            message_client_id.clone(),
            counterparty,
            version,
            message_delay_period,
        );

        debug_println!("[ConnOpenTryReply]: conn end{:?}", conn_end);

        let event = create_connection_event(
            IbcEventType::OpenTryConnection,
            &connection_id,
            &message_client_id,
            &counter_party_client_id,
            counterparty_conn_id,
        )?;

        self.store_connection_to_client(deps.storage, message_client_id, connection_id.clone())?;
        self.store_connection(deps.storage, connection_id.clone(), conn_end.clone())?;

        self.update_connection_commitment(deps.storage, connection_id.clone(), conn_end)?;

        Ok(Response::new()
            .add_attribute("method", "execute_connection_open_try")
            .add_attribute("connection_id", connection_id.as_str())
            .add_event(event))
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
        _info: MessageInfo,
        msg: MsgConnectionOpenConfirm,
    ) -> Result<Response, ContractError> {
        debug_println!("[ConnOpenConfirm]: Connection Open Confirm");
        let conn_end_on_b = self.connection_end(deps.storage, msg.conn_id_on_b.clone())?;
        debug_println!("[ConnOpenConfirm]: Our Connection {:?}", conn_end_on_b);
        let client_id_on_b = conn_end_on_b.client_id();
        let client_id_on_a = conn_end_on_b.counterparty().client_id();
        debug_println!("");

        if !conn_end_on_b.state_matches(&State::TryOpen) {
            return Err(ContractError::IbcConnectionError {
                error: ConnectionError::ConnectionMismatch {
                    connection_id: msg.conn_id_on_b,
                },
            });
        }

        debug_println!("Connection State Matched");

        let _client_cons_state_path_on_b =
            self.consensus_state(deps.storage, client_id_on_b, &msg.proof_height_on_a)?;
        debug_println!("[ConnOpenConfirm]: Consensus State Path Decoded");
        let consensus_state_of_a_on_b = self
            .consensus_state(deps.storage, client_id_on_b, &msg.proof_height_on_a)
            .map_err(|_| ContractError::IbcConnectionError {
                error: ConnectionError::Other {
                    description: "failed to fetch consensus state".to_string(),
                },
            })?;
        debug_println!("Consensus State Decoded");

        let prefix_on_a = conn_end_on_b.counterparty().prefix();
        let prefix_on_b = self.commitment_prefix(deps.as_ref(), &env);

        let client = self.get_client(deps.as_ref().storage, client_id_on_b.clone())?;

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
        client.verify_connection_open_confirm(
            deps.as_ref(),
            verify_connection_state,
            client_id_on_b,
        )?;

        let connection_id = msg.conn_id_on_b.clone();
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
            return Err(ContractError::IbcConnectionError {
                error: ConnectionError::ConnectionMismatch { connection_id },
            });
        }
        debug_println!("[ConnOpenConfirmReply]: Stored Connection State Matched");
        let counter_party_client_id = client_id_on_a.clone();
        debug_println!(
            "[ConnOpenConfirmReply]: CounterParty ClientId {:?}",
            counter_party_client_id
        );
        let counterparty_conn_id = conn_end_on_b.counterparty().connection_id().cloned();
        debug_println!(
            "[ConnOpenConfirmReply]: CounterParty ConnId {:?}",
            counterparty_conn_id
        );
        let counterparty_prefix = prefix_on_b;
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

        let event = create_connection_event(
            IbcEventType::OpenConfirmConnection,
            &connection_id,
            conn_end.client_id(),
            counterparty.client_id(),
            Some(counter_conn_id),
        )?;

        self.store_connection(deps.storage, connection_id.clone(), conn_end.clone())?;
        debug_println!("[ConnOpenConfirmReply]: Connection Stored");

        self.update_connection_commitment(deps.storage, connection_id.clone(), conn_end)?;

        debug_println!("[ConnOpenConfirmReply]: Commitment Stored Stored");

        Ok(Response::new()
            .add_attribute("method", "execute_connection_open_confirm")
            .add_attribute("connection_id", connection_id.as_str())
            .add_event(event))
    }
}
