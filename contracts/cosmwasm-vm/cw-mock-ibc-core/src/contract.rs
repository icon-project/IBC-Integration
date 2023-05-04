#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, IbcEndpoint, IbcPacket,
    IbcPacketReceiveMsg, IbcTimeout, IbcTimeoutBlock, MessageInfo, Reply, Response, StdResult,
    SubMsg, Timestamp, WasmMsg,
};
use cw2::set_contract_version;

use cw_common::ibc_types::IbcPacket as Packet;
use cw_common::raw_types::RawPacket;
use cw_common::ProstMessage;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-mock-ibc-core";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    let state = State {
        xcall_address: Addr::unchecked(""),
        owner: info.sender,
        sequence: 0,
    };

    STATE.save(deps.storage, &state).unwrap();
    Ok(Response::new().add_attribute("method", "init"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendPacket { packet } => {
            let message: RawPacket = ProstMessage::decode(packet.as_slice())
                .map_err(|_error| ContractError::Unauthorized {})?;

            Ok(Response::new()
                .add_attribute("action", "send_packet")
                .add_attribute("source_channel", message.clone().source_channel)
                .add_attribute("source_port", message.clone().source_port)
                .add_attribute("destination_channel", message.clone().destination_channel)
                .add_attribute("destination_port", message.clone().destination_port)
                .set_data(message.encode_to_vec()))
        }
        ExecuteMsg::ReceivePacket { message } => {
            let state = STATE.load(deps.as_ref().storage).unwrap();

            let message: RawPacket = ProstMessage::decode(message.as_slice()).unwrap();

            let packet_data = Packet::try_from(message).unwrap();

            if state.owner != info.sender {
                return Err(ContractError::Unauthorized {});
            }
            let src_endpoint = IbcEndpoint {
                port_id: packet_data.port_id_on_a.to_string(),
                channel_id: packet_data.chan_id_on_a.to_string(),
            };

            let dst_endpoint = IbcEndpoint {
                port_id: packet_data.port_id_on_b.to_string(),
                channel_id: packet_data.chan_id_on_b.to_string(),
            };
            let timeout = IbcTimeout::with_both(
                IbcTimeoutBlock {
                    revision: packet_data.timeout_height_on_b.commitment_revision_number(),
                    height: packet_data.timeout_height_on_b.commitment_revision_height(),
                },
                Timestamp::from_nanos(packet_data.timeout_timestamp_on_b.nanoseconds()),
            );

            let ibc_packet = IbcPacket::new(
                packet_data.data,
                src_endpoint,
                dst_endpoint,
                packet_data.seq_on_a.into(),
                timeout,
            );

            let message = cw_common::xcall_msg::ExecuteMsg::IbcPacketReceive {
                msg: IbcPacketReceiveMsg::new(ibc_packet, info.sender),
            };

            let submessage: SubMsg<Empty> = SubMsg::reply_on_success(
                CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                    contract_addr: state.xcall_address.to_string(),
                    msg: to_binary(&message).unwrap(),
                    funds: info.funds,
                }),
                521,
            );

            STATE
                .save(
                    deps.storage,
                    &State {
                        xcall_address: state.xcall_address,
                        owner: state.owner,
                        sequence: state.sequence + 1,
                    },
                )
                .unwrap();

            Ok(Response::new()
                .add_submessage(submessage)
                .add_attribute("method", "receive_packet"))
        }
        ExecuteMsg::RegisterXcall { address } => {
            STATE
                .update(deps.storage, |mut data| -> Result<_, ContractError> {
                    data.xcall_address = address.clone();

                    Ok(data)
                })
                .unwrap();

            Ok(Response::new().add_attribute("register_xcall", address.to_string()))
        }
        ExecuteMsg::IbcConfig { msg } => {
            let state = STATE.load(deps.as_ref().storage).unwrap();
            let message = cw_common::xcall_msg::ExecuteMsg::IbcChannelConnect { msg };

            let message = WasmMsg::Execute {
                contract_addr: state.xcall_address.into_string(),
                msg: to_binary(&message).unwrap(),
                funds: info.funds,
            };

            let sub_message = SubMsg::reply_on_success(CosmosMsg::Wasm(message), 532);

            Ok(Response::new()
                .add_submessage(sub_message.clone())
                .add_attribute("method", "ibc_config")
                .add_attribute("data", to_binary(&sub_message).unwrap().to_base64()))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::SequenceSend {
            port_id: _,
            channel_id: _,
        } => {
            let state = STATE.load(deps.storage).unwrap();

            Ok(to_binary(&state.sequence).unwrap())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        521 => Ok(Response::new()
            .add_attribute("method", "reply_521")
            .add_attribute("data", to_binary(&msg.result).unwrap().to_base64())),
        532 => Ok(Response::new()
            .add_attribute("method", "reply_532")
            .add_attribute("data", to_binary(&msg.result).unwrap().to_base64())),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Coin,
    };

    use crate::{
        msg::ExecuteMsg,
        state::{State, STATE},
    };

    use super::execute;

    #[test]
    fn test() {
        let mut mock_deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[Coin::new(1000000, "denom")]);
        let data: Vec<u8> = vec![
            8, 1, 18, 8, 111, 117, 114, 45, 112, 111, 114, 116, 26, 9, 99, 104, 97, 110, 110, 101,
            108, 45, 49, 34, 10, 116, 104, 101, 105, 114, 95, 112, 111, 114, 116, 42, 9, 99, 104,
            97, 110, 110, 101, 108, 45, 51, 50, 133, 3, 123, 34, 109, 101, 115, 115, 97, 103, 101,
            95, 116, 121, 112, 101, 34, 58, 34, 99, 97, 108, 108, 95, 115, 101, 114, 118, 105, 99,
            101, 95, 114, 101, 113, 117, 101, 115, 116, 34, 44, 34, 112, 97, 121, 108, 111, 97,
            100, 34, 58, 91, 50, 52, 56, 44, 57, 51, 44, 50, 49, 48, 44, 49, 52, 53, 44, 49, 50,
            48, 44, 49, 50, 49, 44, 49, 50, 50, 44, 49, 49, 54, 44, 49, 48, 49, 44, 49, 49, 53, 44,
            49, 49, 54, 44, 49, 48, 53, 44, 49, 49, 48, 44, 49, 48, 51, 44, 57, 55, 44, 57, 57, 44,
            57, 57, 44, 49, 49, 49, 44, 49, 49, 55, 44, 49, 49, 48, 44, 49, 49, 54, 44, 49, 56, 52,
            44, 54, 54, 44, 57, 55, 44, 49, 49, 52, 44, 57, 57, 44, 49, 48, 52, 44, 49, 49, 57, 44,
            57, 55, 44, 49, 50, 49, 44, 52, 57, 44, 57, 57, 44, 49, 49, 52, 44, 49, 48, 54, 44, 49,
            48, 52, 44, 53, 52, 44, 53, 50, 44, 57, 55, 44, 53, 55, 44, 49, 50, 49, 44, 52, 56, 44,
            53, 48, 44, 57, 55, 44, 53, 54, 44, 53, 53, 44, 49, 48, 51, 44, 49, 49, 48, 44, 49, 49,
            56, 44, 52, 56, 44, 49, 48, 55, 44, 53, 55, 44, 49, 49, 57, 44, 49, 49, 53, 44, 53, 49,
            44, 49, 49, 56, 44, 53, 54, 44, 49, 48, 54, 44, 49, 50, 49, 44, 49, 48, 48, 44, 49, 49,
            54, 44, 53, 52, 44, 49, 48, 57, 44, 52, 56, 44, 49, 49, 57, 44, 49, 49, 53, 44, 49, 48,
            55, 44, 49, 49, 52, 44, 49, 49, 52, 44, 52, 56, 44, 49, 49, 53, 44, 49, 49, 53, 44, 53,
            48, 44, 53, 52, 44, 53, 49, 44, 49, 48, 51, 44, 49, 49, 51, 44, 53, 52, 44, 49, 49, 53,
            44, 49, 49, 48, 44, 53, 48, 44, 49, 48, 57, 44, 57, 57, 44, 49, 49, 53, 44, 53, 52, 44,
            49, 48, 55, 44, 49, 48, 54, 44, 49, 49, 57, 44, 53, 49, 44, 49, 49, 51, 44, 49, 44, 49,
            50, 56, 44, 49, 51, 49, 44, 49, 44, 50, 44, 51, 93, 125, 58, 2, 16, 10,
        ];

        STATE
            .save(
                mock_deps.as_mut().storage,
                &State {
                    xcall_address: Addr::unchecked("input"),
                    owner: Addr::unchecked("sender"),
                    sequence: 10,
                },
            )
            .unwrap();

        let mesasge = ExecuteMsg::ReceivePacket { message: data };

        let res = execute(mock_deps.as_mut(), env, info, mesasge);

        assert!(res.is_ok())
    }
}
