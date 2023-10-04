use cw_xcall_lib::network_address::NetworkAddress;

use crate::types::{config::Config, LOG_PREFIX};

use super::*;
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-xcall";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwCallService<'a> {
    /// This function instantiates a contract and initializes it with the provided message and
    /// information.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is short for "dependencies mutable". It is a struct
    /// that provides access to the contract's dependencies, such as the storage, API, and querier.
    /// * `_env`: The `_env` parameter in the `instantiate` function is of type `Env`, which represents
    /// the environment in which the contract is being executed. It contains information such as the
    /// current block height, the current time, and the address of the contract being executed. However,
    /// in the given code snippet
    /// * `info`: `info` is a struct that contains information about the message sender, such as their
    /// address, the amount of tokens they sent with the message, and the maximum amount of gas they are
    /// willing to pay for the transaction. This information can be used to determine whether the sender
    /// is authorized to perform certain actions.
    /// * `msg`: The `msg` parameter in the `instantiate` function is of type `InstantiateMsg` and
    /// contains the message sent by the user when instantiating the contract. It can contain any custom
    /// data that the user wants to pass to the contract during instantiation.
    /// Returns:
    ///
    /// The `instantiate` function returns a `Result<Response, ContractError>` where `Response` is a
    /// struct representing the response to a message and `ContractError` is an enum representing the
    /// possible errors that can occur during contract execution. The function returns the result of
    /// calling the `init` function with the provided arguments.
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        self.init(deps.storage, info, msg)
    }

    /// This function executes various messages based on their type and returns a response or an error.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct which provides access to the contract's dependencies such
    /// as storage, API, and querier. It is mutable, meaning the contract can modify its dependencies.
    /// * `env`: `env` is a struct that contains information about the current blockchain environment,
    /// such as the block height, time, and chain ID. It is passed as a parameter to the `execute`
    /// function in order to provide context for the execution of the contract.
    /// * `info`: `info` is a struct that contains information about the message sender, including their
    /// address, public key, and the amount of tokens they sent with the message. It is of type
    /// `MessageInfo`.
    /// * `msg`: The `msg` parameter is of type `ExecuteMsg` and represents the message that is being
    /// executed by the contract. It is matched against different variants to determine the action to be
    /// taken.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing an error that occurred during contract
    /// execution.
    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::SetAdmin { address } => {
                let validated_address =
                    CwCallService::validate_address(deps.api, address.as_str())?;
                self.ensure_admin(deps.storage, info.sender)?;
                self.set_admin(deps.storage, validated_address)
            }
            ExecuteMsg::SetProtocolFee { value } => self.set_protocol_fee(deps, info, value),
            ExecuteMsg::SetProtocolFeeHandler { address } => {
                self.set_protocol_feehandler(deps, &info, address)
            }
            ExecuteMsg::SendCallMessage {
                to,
                sources,
                destinations,
                data,
                rollback,
            } => {
                println!("{LOG_PREFIX} Received Send Call Message");
                let sources = sources.unwrap_or(vec![]);
                let dests = destinations.unwrap_or(vec![]);
                self.send_call_message(deps, info, env, to, data, rollback, sources, dests)
            }
            ExecuteMsg::HandleMessage { msg, from_nid } => {
                self.handle_message(deps, info, from_nid, msg)
            }
            ExecuteMsg::HandleError { sn } => self.handle_error(deps, info, sn),
            ExecuteMsg::ExecuteCall { request_id, data } => {
                self.execute_call(deps, info, request_id, data)
            }
            ExecuteMsg::ExecuteRollback { sequence_no } => {
                self.execute_rollback(deps, env, info, sequence_no)
            }
            ExecuteMsg::SetDefaultConnection { nid, address } => {
                self.set_default_connection(deps, info, nid, address)
            }
        }
    }

    /// The `query` function takes in dependencies, environment, and a message, and returns a binary
    /// result based on the type of message received.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is an object of type `Deps` which contains various dependencies required for
    /// executing the contract. These dependencies include the storage, API, and querier. The `deps`
    /// object is passed as an argument to the `query` function to access these dependencies.
    /// * `_env`: The `_env` parameter is an instance of the `Env` struct, which provides information
    /// about the current blockchain environment, such as the block height and time. However, it is not
    /// used in this particular `query` function.
    /// * `msg`: `msg` is a parameter of type `QueryMsg` which is an enum that represents different types
    /// of queries that can be made to the smart contract. The function matches on the variant of
    /// `QueryMsg` that is passed in and executes the corresponding logic.
    ///
    /// Returns:
    ///
    /// The `query` function returns a `StdResult<Binary>` which can contain either the binary
    /// representation of the result of the query or an error if the query fails. The specific result
    /// being returned depends on the type of `QueryMsg` being passed in and the logic of the
    /// corresponding match arm.
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetAdmin {} => match self.query_admin(deps.storage) {
                Ok(admin) => Ok(to_binary(&admin)?),
                Err(error) => Err(StdError::NotFound {
                    kind: error.to_string(),
                }),
            },

            QueryMsg::GetProtocolFee {} => to_binary(&self.get_protocol_fee(deps.storage)),
            QueryMsg::GetProtocolFeeHandler {} => to_binary(&self.get_protocol_feehandler(deps)),
            QueryMsg::GetNetworkAddress {} => {
                to_binary(&self.get_own_network_address(deps.storage, &env).unwrap())
            }
            QueryMsg::VerifySuccess { sn } => {
                to_binary(&self.get_successful_response(deps.storage, sn))
            }
            QueryMsg::GetDefaultConnection { nid } => {
                to_binary(&self.get_default_connection(deps.storage, nid).unwrap())
            }
            QueryMsg::GetFee {
                nid,
                rollback,
                sources,
            } => to_binary(
                &self
                    .get_fee(deps, nid, rollback, sources.unwrap_or(vec![]))
                    .unwrap(),
            ),
        }
    }
    /// This function handles different types of reply messages and calls corresponding functions based on
    /// the message ID.
    ///
    /// Arguments:
    ///
    /// * `deps`: A mutable reference to the dependencies of the contract, which includes access to the
    /// storage, API, and other modules.
    /// * `env`: `env` is an environment variable that provides information about the current execution
    /// environment of the smart contract. It includes information such as the current block height, the
    /// sender address, the contract address, and the current time. This information can be used by the
    /// smart contract to make decisions or perform actions based on
    /// * `msg`: `msg` is a `Reply` struct that contains the message ID and any associated data that was
    /// sent as a reply to a previous message. The `reply` function uses the message ID to determine which
    /// specific reply message was received and then calls the appropriate function to handle that message.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is the response to the message and
    /// `ContractError` is an error type that can be returned if there is an error in processing the
    /// message.

    pub fn reply(&self, deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.id {
            EXECUTE_CALL_ID => self.execute_call_reply(deps, env, msg),
            _ => Err(ContractError::ReplyError {
                code: msg.id,
                msg: "Unknown".to_string(),
            }),
        }
    }

    pub fn migrate(
        &self,
        deps: DepsMut,
        _env: Env,
        _msg: MigrateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(Response::default().add_attribute("migrate", "successful"))
    }
}

impl<'a> CwCallService<'a> {
    fn init(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let last_sequence_no = u128::default();
        let last_request_id = u128::default();
        self.set_admin(store, info.sender.clone())?;
        self.init_last_sequence_no(store, last_sequence_no)?;
        self.init_last_request_id(store, last_request_id)?;
        let caller = info.sender;
        self.store_config(
            store,
            &Config {
                network_id: msg.network_id,
                denom: msg.denom,
            },
        )?;
        self.store_protocol_fee_handler(store, caller.to_string())?;

        Ok(Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("method", "init"))
    }

    pub fn get_own_network_address(
        &self,
        store: &dyn Storage,
        env: &Env,
    ) -> Result<NetworkAddress, ContractError> {
        let config = self.get_config(store)?;
        let address = env.contract.address.to_string();
        let na = NetworkAddress::new(&config.network_id, &address);
        Ok(na)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cw2::{get_contract_version, ContractVersion};

    use crate::{
        contract::{CONTRACT_NAME, CONTRACT_VERSION},
        state::CwCallService,
        MigrateMsg,
    };

    #[test]
    fn test_migrate() {
        let mut mock_deps = mock_dependencies();
        let env = mock_env();

        let contract = CwCallService::default();
        let result = contract.migrate(mock_deps.as_mut(), env, MigrateMsg {});
        assert!(result.is_ok());
        let expected = ContractVersion {
            contract: CONTRACT_NAME.to_string(),
            version: CONTRACT_VERSION.to_string(),
        };
        let version = get_contract_version(&mock_deps.storage).unwrap();
        println!("{version:?}");
        assert_eq!(expected, version);
    }
}
