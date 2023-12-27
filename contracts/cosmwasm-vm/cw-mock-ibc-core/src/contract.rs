use common::ibc::core::ics04_channel::timeout::TimeoutHeight;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, Event, IbcEndpoint, IbcPacket,
    IbcPacketReceiveMsg, IbcTimeout, IbcTimeoutBlock, MessageInfo, Reply, Response, StdResult,
    SubMsg, Timestamp, WasmMsg,
};
use cw2::set_contract_version;
use cw_common::hex_string::HexString;
use cw_common::ibc_types::IbcHeight;
use cw_common::raw_types::channel::RawPacket;
use cw_common::ProstMessage;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{State, STATE};
use cw_common::core_msg::QueryMsg;

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
            let packet = packet.to_bytes().unwrap();
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

            let data = hex::decode(message).unwrap();

            let message: RawPacket = ProstMessage::decode(data.as_slice()).unwrap();

            //let packet_data = Packet::try_from(message).unwrap();

            if state.owner != info.sender {
                return Err(ContractError::Unauthorized {});
            }
            let src_endpoint = IbcEndpoint {
                port_id: message.source_port.to_string(),
                channel_id: message.source_channel.to_string(),
            };

            let dst_endpoint = IbcEndpoint {
                port_id: message.destination_port.to_string(),
                channel_id: message.destination_channel.to_string(),
            };
            let timeout_height: TimeoutHeight =
                TimeoutHeight::from(IbcHeight::try_from(message.timeout_height.unwrap()).unwrap());
            let timeout = IbcTimeout::with_both(
                IbcTimeoutBlock {
                    revision: timeout_height.commitment_revision_number(),
                    height: timeout_height.commitment_revision_height(),
                },
                Timestamp::from_nanos(message.timeout_timestamp),
            );

            let ibc_packet = IbcPacket::new(
                message.data,
                src_endpoint,
                dst_endpoint,
                message.sequence,
                timeout,
            );

            let message = cw_common::ibc_dapp_msg::ExecuteMsg::IbcPacketReceive {
                msg: IbcPacketReceiveMsg::new(ibc_packet, info.sender.clone()),
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
            let message = cw_common::ibc_dapp_msg::ExecuteMsg::IbcChannelConnect { msg };

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
        ExecuteMsg::WriteAcknowledgement {
            packet: _,
            acknowledgement,
        } => Ok(Response::new().add_event(event_ack(acknowledgement))),
    }
}

pub fn event_ack(ack: HexString) -> Event {
    Event::new("write_acknowledgement").add_attribute("data", hex::encode(ack.to_bytes().unwrap()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNextSequenceSend {
            port_id: _,
            channel_id: _,
        } => {
            let state = STATE.load(deps.storage).unwrap();

            Ok(to_binary(&state.sequence).unwrap())
        }
        QueryMsg::GetLatestHeight { client_id: _ } => Ok(to_binary(&100000_u64).unwrap()),
        _ => Err(cosmwasm_std::StdError::NotFound {
            kind: "Query Not Found".to_string(),
        }),
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

        let mesasge = ExecuteMsg::ReceivePacket { message: "080a12086f75722d706f72741a094368616e6e6c652d31220a74686569722d706f72742a096368616e6e6c652d33329c01f89a00b897f895b84261726368776179316e633574617461667636657971376c6c6b7232677635306666396532326d6e66373071676a6c763733376b746d74346573777271676a33336736b8426172636877617931377039727a776e6e6678636a703332756e397567377968687a67746b68766c396a666b737a74677735756836397761633270677373663035703701008a68656c6c6f776f726c643a02100a".to_string() };

        let res = execute(mock_deps.as_mut(), env, info, mesasge);

        assert!(res.is_ok())
    }
}
