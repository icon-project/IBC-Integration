use super::*;
use common::constants::{ICON_CLIENT_STATE_TYPE_URL, ICON_CONSENSUS_STATE_TYPE_URL};
use common::icon::icon::lightclient::v1::{
    ClientState as RawClientState, ConsensusState as RawConsensusState,
};
use cosmwasm_std::to_binary;
use cw_common::hex_string::HexString;
use cw_common::raw_types::{channel::*, Any};
use cw_common::raw_types::{Protobuf, RawHeight};

use hex::FromHexError;
use prost::{DecodeError, Message};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-ibc-core";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwIbcClientContext<'a> {
    /// The `instantiate` function initializes the contract and sets default values for channel, client,
    /// and connection counters, as well as setting the contract owner.
    ///
    /// Arguments:
    ///
    /// * `deps`: A mutable reference to the dependencies of the contract, which includes access to the
    /// storage, API, and other contracts.
    /// * `_env`: _env is an object that represents the current blockchain environment, including
    /// information such as the block height, time, and chain ID. It is passed as a parameter to the
    /// instantiate function in the CosmWasm smart contract framework.
    /// * `info`: MessageInfo is a struct that contains information about the message being executed,
    /// such as the sender address, the amount of tokens sent with the message, and the message ID. It is
    /// passed as a parameter to the instantiate function in order to set the owner of the contract.
    /// * `_msg`: The `_msg` parameter is of type `InstantiateMsg` and represents the message sent by the
    /// user to instantiate the contract. It contains any custom data or parameters required for the
    /// contract initialization.
    ///
    /// Returns:
    ///
    /// A `Result<Response, ContractError>` is being returned. The `Response` contains an attribute
    /// "method" with the value "instantiate". If there are no errors, the `Result` will be `Ok`,
    /// otherwise it will be `Err(ContractError::Std)`.
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;

        // self.init_channel_counter(deps.storage, u64::default())?;
        self.init_client_counter(deps.storage, u64::default())?;
        //  self.init_connection_counter(deps.storage, u64::default())?;
        self.set_owner(deps.storage, info.sender)?;

        Ok(Response::new().add_attribute("method", "instantiate"))
    }

    /// This function handles the execution of various IBC-related messages in a contract.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides access to the contract's dependencies
    /// such as storage, querier, and API interfaces. It is used to interact with the blockchain and
    /// other contracts.
    /// * `env`: `env` is a struct that contains information about the current execution environment,
    /// such as the block height and time, the chain ID, and the sender address. It is passed as an
    /// argument to the `execute` function in a CosmWasm smart contract.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being executed, such as the sender address, the amount of funds sent with the message, and the
    /// gas limit. It is passed as an argument to the `execute` function in the Cosmos SDK.
    /// * `msg`: The `msg` parameter in the `execute` function is of type `CoreExecuteMsg` and
    /// represents the message that the contract should execute. The function matches the type of the
    /// message and calls the appropriate function to handle it.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing possible errors that can occur
    /// during contract execution.
    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: CoreExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            CoreExecuteMsg::RegisterClient {
                client_type,
                client_address,
            } => {
                self.check_sender_is_owner(deps.as_ref().storage, info.sender.clone())?;
                let client_type = ClientType::new(client_type);
                self.register_client(deps, client_type, client_address)
            }
            CoreExecuteMsg::CreateClient {
                client_state,
                consensus_state,
                signer,
            } => {
                self.check_sender_is_owner(deps.as_ref().storage, info.sender.clone())?;
                let client_state_bytes = client_state
                    .to_bytes()
                    .map_err(|e| Into::<FromHexError>::into(e))?;
                let client_state = Any::decode(client_state_bytes.as_slice())
                    .map_err(|e| Into::<DecodeError>::into(e))?;
                let consensus_state_bytes = consensus_state
                    .to_bytes()
                    .map_err(|e| Into::<FromHexError>::into(e))?;
                let consensus_state = Any::decode(consensus_state_bytes.as_slice())
                    .map_err(|e| Into::<DecodeError>::into(e))?;

                let signer = Self::to_signer(&signer)?;
                let msg = IbcMsgCreateClient {
                    client_state: client_state,
                    consensus_state: consensus_state,
                    signer,
                };
                self.create_client(deps, info, msg)
            }
            CoreExecuteMsg::UpdateClient {
                client_id,
                header,
                signer,
            } => {
                self.check_sender_is_owner(deps.as_ref().storage, info.sender.clone())?;
                let header_bytes = header
                    .to_bytes()
                    .map_err(|e| Into::<FromHexError>::into(e))?;
                let header = Any::decode(header_bytes.as_slice())
                    .map_err(|e| Into::<DecodeError>::into(e))?;

                let signer = Self::to_signer(&signer)?;
                let msg = IbcMsgUpdateClient {
                    client_id: IbcClientId::from_str(&client_id).map_err(|error| {
                        ContractError::IbcDecodeError {
                            error: error.to_string(),
                        }
                    })?,
                    header,
                    signer,
                };
                println!("Updating Client For {}", &client_id);
                self.update_client(deps, info, msg)
            }
            CoreExecuteMsg::UpgradeClient {} => {
                unimplemented!()
            }
            CoreExecuteMsg::ClientMisbehaviour {} => {
                unimplemented!()
            }
            _ => return Err(ContractError::UnsupportedMessage),
        }
    }
    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetClientRegistry { _type } => {
                let res = self
                    .get_client_from_registry(deps.storage, ClientType::new(_type.clone()))
                    .map_err(|_| ContractError::InvalidClientType { client_type: _type })
                    .unwrap();
                let addr = Addr::unchecked(res);
                to_binary(&addr)
            }
            QueryMsg::GetClientType { client_id } => {
                let res = self
                    .get_client_type(deps.storage, ClientId::from_str(&client_id).unwrap())
                    .map_err(|_| ContractError::InvalidClientId { client_id })
                    .unwrap();
                to_binary(&res)
            }
            QueryMsg::GetClientImplementation { client_id } => {
                let res = self
                    .get_client_implementations(
                        deps.storage,
                        ClientId::from_str(&client_id).unwrap(),
                    )
                    .map_err(|_| ContractError::InvalidClientId { client_id })
                    .unwrap();
                let addr = Addr::unchecked(res);
                to_binary(&addr)
            }
            QueryMsg::GetConsensusState { client_id, height } => {
                let raw_height: RawHeight = RawHeight::decode(height.to_bytes().unwrap().as_ref())
                    .map_err(|_| ClientError::InvalidHeight)
                    .unwrap();
                let height =
                    Height::new(raw_height.revision_number, raw_height.revision_height).unwrap();

                let res = self
                    .consensus_state(
                        deps.storage,
                        &IbcClientId::from_str(&client_id).unwrap(),
                        &height,
                    )
                    .map_err(|_| ContractError::InvalidClientId { client_id })
                    .unwrap();

                let any = Any {
                    type_url: ICON_CONSENSUS_STATE_TYPE_URL.to_string(),
                    value: res.as_bytes(),
                };
                to_binary(&any.encode_to_vec())
            }
            QueryMsg::GetClientState { client_id } => {
                let res = self
                    .get_client_state(deps.storage, ClientId::from_str(&client_id).unwrap())
                    .map_err(|_| ContractError::InvalidClientId { client_id })
                    .unwrap();
                let any = Any {
                    type_url: ICON_CLIENT_STATE_TYPE_URL.to_string(),
                    value: res,
                };
                to_binary(&any.encode_to_vec())
            }

            QueryMsg::GetNextClientSequence => {
                let res = self
                    .client_counter(deps.storage)
                    .map_err(|_| ContractError::InvalidNextClientSequence {})
                    .unwrap();
                to_binary(&res)
            }
            _ => {
                return Err(StdError::NotFound {
                    kind: "Query".to_string(),
                })
            }
        }
    }

    /// This function handles different types of replies based on their ID and executes the
    /// corresponding function.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract, which includes
    /// access to the storage, API, and other modules. It is of type `DepsMut`.
    /// * `_env`: _env is an object of type `Env` which represents the environment in which the contract
    /// is executing. It contains information such as the current block height, the current time, and
    /// the address of the contract.
    /// * `message`: The `message` parameter is of type `Reply` and contains information about the reply
    /// being processed, including the ID of the reply and any associated data.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` and `ContractError` are defined types.
    pub fn reply(
        &self,
        deps: DepsMut,
        _env: Env,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.id {
            EXECUTE_CREATE_CLIENT => self.execute_create_client_reply(deps, message),
            EXECUTE_UPDATE_CLIENT => self.execute_update_client_reply(deps, message),
            EXECUTE_UPGRADE_CLIENT => self.execute_upgrade_client_reply(deps, message),
            MISBEHAVIOUR => self.execute_misbehaviour_reply(deps, message),

            _ => Err(ContractError::ReplyError {
                code: message.id,
                msg: "InvalidReplyID".to_string(),
            }),
        }
        //  Ok(Response::new())
    }

    /// This function calculates the fee for a given expected gas amount and gas price.
    ///
    /// Arguments:
    ///
    /// * `expected_gas`: `expected_gas` is an input parameter of type `u64` which represents the
    /// expected amount of gas required to execute a transaction on the blockchain. Gas is a unit of
    /// measurement for the computational effort required to execute a transaction or contract on the
    /// Ethereum network. The higher the gas limit, the more
    ///
    /// Returns:
    ///
    /// The function `calculate_fee` returns a `u128` value, which represents the calculated fee based
    /// on the expected gas and the gas price.
    pub fn calculate_fee(&self, expected_gas: u64) -> u128 {
        let fee = expected_gas as u128 * self.gas_price();

        fee.checked_div(GAS_DENOMINATOR as u128).unwrap()
    }

    /// This function calculates the gas price in Rust programming language.
    ///
    /// Returns:
    ///
    /// an unsigned 128-bit integer, which represents the gas price calculated based on the default gas
    /// numerator and gas adjustment numerator values.
    pub fn gas_price(&self) -> u128 {
        let price = GAS_NUMERATOR_DEFAULT * GAS_ADJUSTMENT_NUMERATOR_DEFAULT;

        price.checked_div(GAS_DENOMINATOR).unwrap();

        price as u128
    }
    /// The function updates the balance of each coin in a vector by subtracting a fee and returns the
    /// updated vector.
    ///
    /// Arguments:
    ///
    /// * `coins`: A vector of `Coin` structs representing the current balance of the user. Each `Coin`
    /// struct contains an amount and a denomination.
    /// * `fee`: The `fee` parameter is an unsigned 128-bit integer representing the amount of fee to be
    /// deducted from each coin's balance.
    ///
    /// Returns:
    ///
    /// a `Result` type with a vector of `Coin` objects as the successful result or a `ContractError` if
    /// there is an insufficient balance.
    pub fn update_fee(&self, coins: Vec<Coin>, fee: u128) -> Result<Vec<Coin>, ContractError> {
        if coins.is_empty() {
            return Err(ContractError::InsufficientBalance {});
        }

        let updated_coins = coins
            .into_iter()
            .map(|coin| {
                let updated_balance = coin.amount.u128().checked_sub(fee).unwrap();

                Coin::new(updated_balance, coin.denom)
            })
            .collect::<Vec<Coin>>();

        Ok(updated_coins)
    }

    /// This function converts a hexadecimal string to a Rust type that implements the Message trait and
    /// can be converted to another type using the TryFrom trait.
    ///
    /// Arguments:
    ///
    /// * `hex_str`: A hexadecimal string that represents the serialized bytes of a protobuf message.
    ///
    /// Returns:
    ///
    /// a `Result` with a generic type `T` which is the converted message from the raw bytes provided in
    /// the `HexString`. If the conversion is successful, it returns an `Ok` variant with the converted
    /// message. If there is an error during the conversion, it returns an `Err` variant with a
    /// `ContractError` that describes the error encountered.
    pub fn from_raw<R: Message + std::default::Default + Clone, T: TryFrom<R>>(
        hex_str: &HexString,
    ) -> Result<T, ContractError>
    where
        <T as TryFrom<R>>::Error: std::fmt::Debug,
    {
        let bytes = hex_str
            .to_bytes()
            .map_err(|e| ContractError::IbcDecodeError {
                error: e.to_string(),
            })?;
        let raw = <R as Message>::decode(bytes.as_slice()).map_err(|error| {
            ContractError::IbcDecodeError {
                error: error.to_string(),
            }
        })?;
        let message = T::try_from(raw).map_err(|error| {
            let err = format!("Failed to convert to ibc type with error {:?}", error);
            ContractError::IbcRawConversionError { error: err }
        })?;
        Ok(message)
    }

    /// The function converts a hexadecimal string to a Signer object and returns an error if the
    /// conversion fails.
    ///
    /// Arguments:
    ///
    /// * `str`: A hexadecimal string representing a signer address.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either a `Signer` or a `ContractError`.
    pub fn to_signer(str: &HexString) -> Result<Signer, ContractError> {
        let bytes = str.to_bytes().map_err(|e| ContractError::IbcDecodeError {
            error: e.to_string(),
        })?;
        let signer_string =
            String::from_utf8(bytes).map_err(|error| ContractError::IbcDecodeError {
                error: error.to_string(),
            })?;

        let signer =
            Signer::from_str(&signer_string).map_err(|error| ContractError::IbcDecodeError {
                error: error.to_string(),
            })?;
        Ok(signer)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::context::CwIbcClientContext;
    use common::{
        constants::ICON_CONSENSUS_STATE_TYPE_URL,
        icon::icon::lightclient::v1::ConsensusState as RawConsensusState,
    };
    use ibc::core::ics02_client::height::Height;

    use prost::Message;

    use super::{instantiate, query, InstantiateMsg, QueryMsg};

    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        to_vec, Addr, OwnedDeps,
    };
    use cw_common::raw_types::{Any, RawHeight};
    use cw_common::{hex_string::HexString, ibc_types::IbcClientId, types::ClientType};

    const SENDER: &str = "sender";

    fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info(SENDER, &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        deps
    }

    #[test]
    fn test_query_next_client_sequence() {
        let contract = CwIbcClientContext::default();
        let mut deps = setup();
        let msg = QueryMsg::GetNextClientSequence;
        let result = query(deps.as_ref(), mock_env(), msg.clone()).unwrap();
        let result_parsed: u64 = from_binary(&result).unwrap();
        assert_eq!(0, result_parsed);

        contract
            .increase_client_counter(deps.as_mut().storage)
            .unwrap();
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let result_parsed: u64 = from_binary(&result).unwrap();
        assert_eq!(1, result_parsed);
    }

    #[test]
    fn test_query_get_client_registry() {
        let client_type_str = "test_client_type".to_string();
        let client = "test_client".to_string();
        let client_type = ClientType::new(client_type_str.clone());
        let contract = CwIbcClientContext::default();
        let mut deps = setup();

        contract
            .store_client_into_registry(deps.as_mut().storage, client_type, client.clone())
            .unwrap();

        let msg = QueryMsg::GetClientRegistry {
            _type: client_type_str.clone(),
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let result_parsed: Addr = from_binary(&result).unwrap();
        assert_eq!(client, result_parsed.as_str());
    }

    #[ignore]
    #[test]
    fn test_query_get_consensus_state() {
        let contract = CwIbcClientContext::default();
        let client_id = "test_client".to_string();
        let mut deps = setup();
        let commitment_root =
            "0x7702db70e830e07b4ff46313456fc86d677c7eeca0c011d7e7dcdd48d5aacfe2".to_string();
        let consensus_state = RawConsensusState {
            message_root: commitment_root.encode_to_vec(),
        };

        let height = Height::new(123, 456).unwrap();
        let raw_height: RawHeight = RawHeight::from(height);
        contract
            .store_consensus_state(
                deps.as_mut().storage,
                &IbcClientId::from_str(&client_id).unwrap(),
                height,
                consensus_state.encode_to_vec(),
            )
            .unwrap();

        let msg = QueryMsg::GetConsensusState {
            client_id,
            height: HexString::from_bytes(&raw_height.encode_to_vec()),
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let result_parsed: Vec<u8> = from_binary(&result).unwrap();

        let result_decoded = Any::decode(result_parsed.as_ref()).unwrap();
        assert_eq!(
            ICON_CONSENSUS_STATE_TYPE_URL.to_string(),
            result_decoded.type_url
        );
    }
}
