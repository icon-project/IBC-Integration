#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, IbcEndpoint, IbcPacket,
    IbcPacketReceiveMsg, IbcTimeout, IbcTimeoutBlock, MessageInfo, Reply, Response, StdResult,
    SubMsg, Timestamp,
};
use cw2::set_contract_version;
use cw_common::{Packet, ProstMessage, RawPacket};

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
        ExecuteMsg::SendPacket { message } => {
            let message: RawPacket = ProstMessage::decode(message.as_slice())
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

            let submessage: SubMsg<Empty> = SubMsg::reply_always(
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
        521 => Ok(Response::new().add_attribute("reply", "success")),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {}
