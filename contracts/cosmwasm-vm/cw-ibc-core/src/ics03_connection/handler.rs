use std::{str::from_utf8, time::Duration};

use cw_common::{
    client_msg::VerifyConnectionPayload,
    hex_string::HexString,
    raw_types::connection::{
        RawMsgConnectionOpenAck, RawMsgConnectionOpenConfirm, RawMsgConnectionOpenInit,
        RawMsgConnectionOpenTry,
    },
};

use crate::{
    conversions::{
        to_ibc_client_id, to_ibc_connection_id, to_ibc_counterparty, to_ibc_height, to_ibc_version,
        to_ibc_versions,
    },
    validations::{ensure_connection_state, ensure_consensus_height_valid},
};
use cw_common::cw_println;

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

        let client_state = self.client_state(deps.as_ref(), &client_id)?;

        if client_state.is_frozen() {
            return Err(ClientError::ClientFrozen {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }

        let delay_period = Duration::from_nanos(message.delay_period);
        let ibc_version = to_ibc_version(message.version).ok();
        let ibc_counterparty = to_ibc_counterparty(message.counterparty)?;

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

        self.update_connection_commitment(deps.storage, &connection_identifier, &connection_end)?;
        self.store_connection_to_client(deps.storage, &client_id, &connection_identifier)?;
        self.store_connection(deps.storage, &connection_identifier, &connection_end)?;

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
        msg: RawMsgConnectionOpenAck,
    ) -> Result<Response, ContractError> {
        cw_println!(deps, "[ConnOpenAck]: Connection Open Ack");
        let message_client_state = msg.client_state.ok_or(ContractError::IbcConnectionError {
            error: ConnectionError::MissingClientState,
        })?;

        let consensus_height = to_ibc_height(msg.consensus_height)?;
        let proof_height = to_ibc_height(msg.proof_height)?;
        let message_version = to_ibc_version(msg.version)?;

        let host_height = self.host_height(&env)?;
        cw_println!(deps, "[ConnOpenAck]: Host Height {:?}", host_height);

        ensure_consensus_height_valid(&host_height, &consensus_height)?;
        cw_println!(deps, "[ConnOpenAck]:Consensus Height Valid");

        self.validate_self_client(message_client_state.clone())?;
        cw_println!(deps, "[ConnOpenAck]: Self Client Valid");

        let connection_id = to_ibc_connection_id(&msg.connection_id)?;
        let mut connection_end = self.connection_end(deps.storage, &connection_id)?;
        let client_id = connection_end.client_id();
        let client_state = self.client_state(deps.as_ref(), client_id)?;

        if client_state.is_frozen() {
            return Err(ClientError::ClientFrozen {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }
        let prefix = self.commitment_prefix(deps.as_ref(), &env);

        let counterparty_client_id = connection_end.counterparty().client_id();
        let counterparty_prefix = connection_end.counterparty().prefix();
        let counter_connection_id = to_ibc_connection_id(&msg.counterparty_connection_id)?;

        ensure_connection_state(&connection_id, &connection_end, &State::Init)?;

        if !(connection_end.versions().contains(&message_version)) {
            return Err(ContractError::IbcConnectionError {
                error: ConnectionError::ConnectionMismatch { connection_id },
            });
        }

        cw_println!(deps, "[ConnOpenAck]: State Matched");

        let consensus_state = self.consensus_state(deps.as_ref(), client_id, &proof_height)?;

        let client = self.get_light_client(deps.as_ref().storage, client_id)?;

        let expected_connection_end: ConnectionEnd = ConnectionEnd::new(
            State::TryOpen,
            counterparty_client_id.clone(),
            Counterparty::new(client_id.clone(), Some(connection_id.clone()), prefix),
            vec![message_version.clone()],
            connection_end.delay_period(),
        );
        cw_println!(
            deps,
            "[ConnOpenAck]: Expected conn end {:?}",
            expected_connection_end
        );
        let connection_path = commitment::connection_path(&counter_connection_id);
        let verify_connection_state = VerifyConnectionState::new(
            proof_height.to_string(),
            to_vec(&counterparty_prefix)?,
            msg.proof_try,
            connection_path,
            expected_connection_end.encode_vec(),
        );

        let client_state_path = commitment::client_state_path(counterparty_client_id);
        let verify_client_full_state = VerifyClientFullState::new(
            proof_height.to_string(),
            to_vec(&counterparty_prefix)?,
            msg.proof_client,
            client_state_path,
            message_client_state.value,
        );

        let consensus_state_path_on_b =
            commitment::consensus_state_path(counterparty_client_id, &consensus_height);
        let verify_client_consensus_state = VerifyClientConsensusState::new(
            proof_height.to_string(),
            to_vec(&counterparty_prefix)?,
            msg.proof_consensus,
            consensus_state_path_on_b,
            consensus_state.clone().as_bytes(),
        );
        let payload = VerifyConnectionPayload {
            client_id: client_id.to_string(),
            verify_connection_state,
            verify_client_full_state,
            verify_client_consensus_state,
        };

        client.verify_connection_open_ack(deps.as_ref(), payload)?;

        let counterparty = Counterparty::new(
            counterparty_client_id.clone(),
            Some(counter_connection_id.clone()),
            counterparty_prefix.clone(),
        );

        connection_end.set_state(State::Open);
        connection_end.set_version(message_version);
        connection_end.set_counterparty(counterparty.clone());

        let event = create_connection_event(
            IbcEventType::OpenAckConnection,
            &connection_id,
            &connection_end.client_id().clone(),
            counterparty.client_id(),
            Some(counter_connection_id),
        )?;

        self.store_connection(deps.storage, &connection_id, &connection_end)?;
        cw_println!(deps, "[ConnOpenAckReply]: Connection Stored");

        self.update_connection_commitment(deps.storage, &connection_id, &connection_end)?;

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
        let client_id = to_ibc_client_id(&message.client_id)?;
        let consensus_height = to_ibc_height(message.consensus_height)?;
        let proof_height = to_ibc_height(message.proof_height)?;

        let client_state = self.client_state(deps.as_ref(), &client_id)?;

        if client_state.is_frozen() {
            return Err(ClientError::ClientFrozen {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }

        let counterparty = to_ibc_counterparty(message.counterparty)?;
        let counterparty_prefix = counterparty.prefix().clone();
        let counterparty_client_id = counterparty.client_id().clone();
        let counterparty_connection_id = counterparty.connection_id;

        let message_versions = to_ibc_versions(message.counterparty_versions)?;
        let message_delay_period = Duration::from_nanos(message.delay_period);
        self.validate_self_client(message_client_state.clone())?;

        let host_height = self.host_height(&env)?;

        ensure_consensus_height_valid(&host_height, &consensus_height)?;

        let prefix = self.commitment_prefix(deps.as_ref(), &env);

        cw_println!(
            deps,
            "prefix_on_b is {:?}",
            from_utf8(&prefix.clone().into_vec()).unwrap()
        );

        let client = self.get_light_client(deps.as_ref().storage, &client_id)?;

        let expected_connection_end = ConnectionEnd::new(
            State::Init,
            counterparty_client_id.clone(),
            Counterparty::new(client_id.clone(), None, prefix),
            message_versions.clone(),
            message_delay_period,
        );

        cw_println!(
            deps,
            "[ConnOpenTry]: expected counterpart connection_end:{:?}",
            HexString::from_bytes(&expected_connection_end.encode_vec())
        );

        let consensus_state = self.consensus_state(deps.as_ref(), &client_id, &proof_height)?;
        cw_println!(
            deps,
            "[ConnOpenTry]: root: {:?} ",
            HexString::from_bytes(consensus_state.root().as_bytes())
        );
        let counterparty_connection_path =
            commitment::connection_path(&counterparty_connection_id.clone().unwrap());
        cw_println!(
            deps,
            "[ConnOpenTry]: connkey: {:?}",
            HexString::from_bytes(&counterparty_connection_path)
        );

        let verify_connection_state = VerifyConnectionState::new(
            proof_height.to_string(),
            to_vec(&counterparty_prefix).map_err(ContractError::Std)?,
            message.proof_init,
            counterparty_connection_path,
            expected_connection_end.encode_vec(),
        );

        // this is verifying tendermint client state and shouldn't have icon-client as an argument
        cw_println!(
            deps,
            "[ConnOpenTry]: payload client state path {:?}",
            &counterparty_client_id
        );
        let client_state_path = commitment::client_state_path(&counterparty_client_id);
        cw_println!(
            deps,
            "[ConnOpenTry]: the clientstate value is  {:?}",
            message_client_state.value
        );
        let verify_client_full_state = VerifyClientFullState::new(
            proof_height.to_string(),
            to_vec(&counterparty_prefix).map_err(ContractError::Std)?,
            message.proof_client,
            client_state_path,
            message_client_state.value.to_vec(),
        );

        let consensus_state_path = commitment::consensus_state_path(&client_id, &consensus_height);

        let verify_client_consensus_state = VerifyClientConsensusState::new(
            proof_height.to_string(),
            to_vec(&counterparty_prefix).map_err(ContractError::Std)?,
            message.proof_consensus,
            consensus_state_path,
            consensus_state.as_bytes(),
        );

        let payload = VerifyConnectionPayload {
            client_id: client_id.to_string(),
            verify_connection_state,
            verify_client_full_state,
            verify_client_consensus_state,
        };

        client.verify_connection_open_try(deps.as_ref(), payload)?;

        let counterparty = Counterparty::new(
            counterparty_client_id.clone(),
            counterparty_connection_id.clone(),
            counterparty_prefix,
        );

        let connection_id = self.generate_connection_idenfier(deps.storage)?;

        cw_println!(
            deps,
            "[ConnOpenTryReply]: connection id is{:?}",
            connection_id
        );

        let conn_end = ConnectionEnd::new(
            State::TryOpen,
            client_id.clone(),
            counterparty,
            message_versions,
            message_delay_period,
        );

        cw_println!(deps, "[ConnOpenTryReply]: conn end{:?}", conn_end);

        let event = create_connection_event(
            IbcEventType::OpenTryConnection,
            &connection_id,
            &client_id,
            &counterparty_client_id,
            counterparty_connection_id,
        )?;

        self.store_connection_to_client(deps.storage, &client_id, &connection_id)?;
        self.store_connection(deps.storage, &connection_id, &conn_end)?;
        self.update_connection_commitment(deps.storage, &connection_id, &conn_end)?;

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
        msg: RawMsgConnectionOpenConfirm,
    ) -> Result<Response, ContractError> {
        cw_println!(deps, "[ConnOpenConfirm]: Connection Open Confirm");

        let proof_height = to_ibc_height(msg.proof_height.clone())?;

        let connection_id = to_ibc_connection_id(&msg.connection_id)?;
        let mut connection_end = self.connection_end(deps.storage, &connection_id)?;
        let client_id = connection_end.client_id();
        let client_state = self.client_state(deps.as_ref(), client_id)?;

        if client_state.is_frozen() {
            return Err(ClientError::ClientFrozen {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }
        let prefix = self.commitment_prefix(deps.as_ref(), &env);
        cw_println!(
            deps,
            "[ConnOpenConfirm]: Our Connection {:?}",
            connection_end
        );

        let counterparty = connection_end.counterparty().clone();
        let counterparty_client_id = counterparty.client_id();
        let counterparty_prefix = counterparty.prefix();
        let counterparty_conn_id = counterparty.connection_id().cloned();
        cw_println!(deps, "[ConnOpenConfirm]: CounterParty  {:?}", &counterparty);

        ensure_connection_state(&connection_id, &connection_end, &State::TryOpen)?;

        let client = self.get_light_client(deps.as_ref().storage, client_id)?;

        let expected_connection_end = ConnectionEnd::new(
            State::Open,
            counterparty_client_id.clone(),
            Counterparty::new(client_id.clone(), Some(connection_id.clone()), prefix),
            connection_end.versions().to_vec(),
            connection_end.delay_period(),
        );

        let connection_path = commitment::connection_path(counterparty.connection_id().unwrap());

        let verify_connection_state = VerifyConnectionState::new(
            proof_height.to_string(),
            to_vec(&counterparty_prefix).map_err(ContractError::Std)?,
            msg.proof_ack,
            connection_path,
            expected_connection_end.encode_vec(),
        );
        client.verify_connection_open_confirm(deps.as_ref(), verify_connection_state, client_id)?;

        cw_println!(
            deps,
            "[ConnOpenConfirmReply]: CounterParty  {:?}",
            counterparty
        );
        connection_end.set_state(State::Open);

        let event = create_connection_event(
            IbcEventType::OpenConfirmConnection,
            &connection_id,
            connection_end.client_id(),
            counterparty.client_id(),
            counterparty_conn_id,
        )?;

        self.store_connection(deps.storage, &connection_id, &connection_end)?;
        cw_println!(deps, "[ConnOpenConfirmReply]: Connection Stored");

        self.update_connection_commitment(deps.storage, &connection_id, &connection_end)?;

        cw_println!(deps, "[ConnOpenConfirmReply]: Commitment Stored Stored");

        Ok(Response::new()
            .add_attribute("method", "execute_connection_open_confirm")
            .add_attribute("connection_id", connection_id.as_str())
            .add_event(event))
    }
}
