mod account;
mod setup;
use std::{collections::HashMap, vec};

use crate::account::*;
use cosmwasm_std::{
    testing::{mock_env, MOCK_CONTRACT_ADDR},
    to_binary, Addr, Binary, ContractInfoResponse, ContractResult, CosmosMsg, IbcMsg, IbcTimeout,
    IbcTimeoutBlock, SystemError, SystemResult, WasmQuery,
};

use cw_xcall_app::{
    state::CwCallService,
    types::{message::CallServiceMessage, request::CallServiceMessageRequest},
};

use setup::test::*;
use test_utils::get_event;

const MOCK_CONTRACT_TO_ADDR: &str = "cosmoscontract";

#[test]
fn send_packet_success() {
    let mock_deps = deps();

    let _mock_info = create_mock_info(MOCK_CONTRACT_ADDR, "umlg", 2000);
    let mut ctx = setup_contracts(mock_deps);
    call_set_xcall_host(&mut ctx).unwrap();
    call_set_ibc_config(&mut ctx).unwrap();
    let src = ctx.connection_host.to_string();
    let result = call_send_call_message(
        &mut ctx,
        MOCK_CONTRACT_TO_ADDR,
        vec![src],
        vec!["somedestination".to_string()],
        vec![1, 2, 3],
        None,
    );
    assert_eq!(true, result.is_ok());
    let result = result.unwrap();
    let event = get_event(&result, "wasm-xcall_app_send_call_message_reply").unwrap();
    assert_eq!("success", event.get("status").unwrap());
}

#[test]
#[should_panic(expected = "RollbackNotPossible")]
fn send_packet_by_non_contract_and_rollback_data_is_not_null() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let env = mock_env();

    let contract = CwCallService::default();

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 3,
    };
    let timeout = IbcTimeout::with_both(timeout_block, env.block.time.plus_seconds(300));
    let data = CallServiceMessageRequest::new(
        mock_info.sender.as_str().to_string(),
        MOCK_CONTRACT_TO_ADDR.to_string(),
        1,
        vec![],
        true,
        vec![1, 2, 3],
    );

    let message: CallServiceMessage = data.into();

    let expected_packet = IbcMsg::SendPacket {
        channel_id: "channel-3".to_string(),
        data: to_binary(&message).unwrap(),
        timeout,
    };

    let result = contract
        .send_packet(
            mock_deps.as_mut(),
            mock_info,
            mock_env(),
            MOCK_CONTRACT_TO_ADDR.to_string(),
            vec![],
            vec![],
            vec![1, 2, 3],
            Some(vec![1, 2, 3]),
        )
        .unwrap();

    assert_eq!(result.messages[0].msg, CosmosMsg::Ibc(expected_packet))
}

#[test]
#[should_panic(expected = "MaxDataSizeExceeded")]
fn send_packet_failure_due_data_len() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(MOCK_CONTRACT_ADDR, "umlg", 2000);

    let _env = mock_env();

    let contract = CwCallService::default();

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    mock_deps.querier.update_wasm(|r| {
        let constract1 = Addr::unchecked(MOCK_CONTRACT_ADDR);
        let mut storage1 = HashMap::<Binary, Binary>::default();
        storage1.insert(b"the key".into(), b"the value".into());
        match r {
            WasmQuery::ContractInfo { contract_addr } => {
                if *contract_addr == constract1 {
                    let response = ContractInfoResponse::default();
                    SystemResult::Ok(ContractResult::Ok(to_binary(&response).unwrap()))
                } else {
                    SystemResult::Err(SystemError::NoSuchContract {
                        addr: contract_addr.clone(),
                    })
                }
            }
            _ => todo!(),
        }
    });

    contract
        .send_packet(
            mock_deps.as_mut(),
            mock_info,
            mock_env(),
            MOCK_CONTRACT_TO_ADDR.to_string(),
            vec![],
            vec![],
            "HuykcBsssssTXfpMmbwWx9COZWbuMkecnMTGI54oXFBsSFOypVHuiBT2egh0drcRAS4wQyGxOCGhL8mBWTttEOG88kuvDcF5R5OmhBa1beo46i9IwD56OqhpzCOVxJqF87fhctAymhmMSBWA95gCnNP45If5FfFtRIsiU9fkwqYPRKCpjtwsFcYJSB3fmABDfsiQBT3rCjvWybzrdN3NoS4VHT3sKuzVeOTNSHDGZaztEpRqBBX1NgNMdky63xfCcslBujryZIbT3xFOXQTzhmCqypqCBsfE2IKbRpZ4zjJjRHhK9e2H2EThk0huP7QVKkHJ2UECyj8QahqvqwtK3QOV8PN1lQmaLV8gtKuBEQalQScHopXOCbeSZgrGRE0r447i7ppCLi6PbX3qja1R3UxMQ2mTIqZRwAsqFHazl7hjchqKkLKrbc0YRz3egQdZi55c7BBpvwGLvEeHUFH4qrSbZ6oRHOJfyWaBtTsoZzjAApSL94EFcFjZV7b5ImDt1uvCy8lGULMig8D8XWcdYQWdlMYwvStzzDpqBU2tw1dX9omD7IJNcBnYNQEXtiEGDdhnCDF9z6lxH0JHG9ZepbiMKi1bduhZrUR51gkqNPT3JxziAlN2xuaM9f3TVIqNLI9IjJYAFNIqe4IZ7qfJCSIoDj2Tq0wJrEkXgW8kAheMzvmOVglr1SlSo3uweVaOgfGbwANak39MtplyksgH8GGgSv0k3ghLHeT06HbKt6MCVCi5fcFLXuCa0HZt7Dslg601YqJn36Hw031ObkJf1HFoNf8mdLHjCfDCXaUwWY9owqmYDL39Jh46P80sXa4u1IqKUfrFMmmCpF7MaVvtdMsJelz2zZHZUPSiC38xfUkOdcgRVcLVBv8GSKcqrMGo9QZs2fu9Zi25WuSZ0SzRo61TjBpRXm1MypIDnxTTEMMBA7l9L7TeojRak80SXhKGx1Pj4AKKNGiKYeIhyx3eSL1JzXmW9qABN6ex1MK4v8pdViMszPgWjeAL95hIWZHuQRMQkTW6A8zBIltmBrM7HAVXbgvMEN48MiacvF7uyC4ogptOw2M01RPX5vgrYS0uXiNUe3AkkPM52z73t6zcNtB1ey1p99HlvVi7ESkPfwQ6MWI2M0bJjru9qYll61idDW3H05v7fFtGg8Ic0MyMbzSX115GIMn6wadHubyaOLNCTJzsApcwuVDUb7uYxRkb53ZP4vVKbPqGugcQojjq22rYNTJt0frigyQYpXm8F1B06VcHnUj460kXEXrpep7UkPaRX5qloF5csnqStuutf9lDPSX8Yrfy6ptdS6FLys0gJpJvR1cDc2h1AfKYyRkflHWUShpJlyrxF4bsOR42vu5ZzX1OQZJaTMaiq1K8IlgzEIFzj9NVji7t26iIgtiZnq17twaw97L3U0I2RlVV6xF9oE27uF08ttTQ0D33VnrzeYBfyrHfrjouf44igELGwolxamYgmaT6NqWhLW45juzqmklNt33DoFRYfMImRrnAbh5zR20XLWAgspPDXgdd52b1sclR6DbAa43wQgdHpoPhSnYCGszGrN2vR1kyMRb32wf37BA725rcOBvfhQSFzNtTk1IqYDyPGUPsZTSknUq4oBRTFJhfzDMh6xy6950EyNAsfUd471kIFvg2dpprhbStY92ftm5TAiAorUXRCljzzU5hfJ6NQinsCmDSRcadtlgn1uThvdqi62xcmDlWDvCf5nKrmad1e3SEmyo99TjjoZXMMPtiGbq9YLEp96JP1TTlcPLHuDewAJNDjQN3ZYg0zQM6b1F3cAD5AgP8ZZc8pK1lJph05YzzV4Lindpx99zewUinVS60ipj4hKbQWNCJhlQCWXPURXI7J8RoLC9leZCqOyPMYEF0tosVmtA4yn1Vup7LJP8DhZ5Br5M0oFGPzmlBSztMj9Gpp0bHnqby5q4elF3KTncyAFDv5xtlN4pxFgclB22aCaT2j7BvNTgaLPQEfH1NQY3fdDzpQbAbGzdLjo77RbClagKYH2iCnq0lg885jMiavPL1NMtqOKPFc".to_string().as_bytes().to_vec(),

           Some(vec![])
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "MaxRollbackSizeExceeded")]
fn send_packet_failure_due_rollback_len() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(MOCK_CONTRACT_ADDR, "umlg", 2000);

    let _env = mock_env();

    let contract = CwCallService::default();

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    mock_deps.querier.update_wasm(|r| {
        let constract1 = Addr::unchecked(MOCK_CONTRACT_ADDR);
        let mut storage1 = HashMap::<Binary, Binary>::default();
        storage1.insert(b"the key".into(), b"the value".into());
        match r {
            WasmQuery::ContractInfo { contract_addr } => {
                if *contract_addr == constract1 {
                    let response = ContractInfoResponse::default();
                    SystemResult::Ok(ContractResult::Ok(to_binary(&response).unwrap()))
                } else {
                    SystemResult::Err(SystemError::NoSuchContract {
                        addr: contract_addr.clone(),
                    })
                }
            }
            _ => todo!(),
        }
    });

    contract
        .send_packet(
            mock_deps.as_mut(),
            mock_info,
            mock_env(),
            MOCK_CONTRACT_TO_ADDR.to_string(),
             vec![],
             vec![],
             vec![],
              Some("HuykcBsssssTXfpMmbwWx9COZWbuMkecnMTGI54oXFBsSFOypVHuiBT2egh0drcRAS4wQyGxOCGhL8mBWTttEOG88kuvDcF5R5OmhBa1beo46i9IwD56OqhpzCOVxJqF87fhctAymhmMSBWA95gCnNP45If5FfFtRIsiU9fkwqYPRKCpjtwsFcYJSB3fmABDfsiQBT3rCjvWybzrdN3NoS4VHT3sKuzVeOTNSHDGZaztEpRqBBX1NgNMdky63xfCcslBujryZIbT3xFOXQTzhmCqypqCBsfE2IKbRpZ4zjJjRHhK9e2H2EThk0huP7QVKkHJ2UECyj8QahqvqwtK3QOV8PN1lQmaLV8gtKuBEQalQScHopXOCbeSZgrGRE0r447i7ppCLi6PbX3qja1R3UxMQ2mTIqZRwAsqFHazl7hjchqKkLKrbc0YRz3egQdZi55c7BBpvwGLvEeHUFH4qrSbZ6oRHOJfyWaBtTsoZzjAApSL94EFcFjZV7b5ImDt1uvCy8lGULMig8D8XWcdYQWdlMYwvStzzDpqBU2tw1dX9omD7IJNcBnYNQEXtiEGDdhnCDF9z6lxH0JHG9ZepbiMKi1bduhZrUR51gkqNPT3JxziAlN2xuaM9f3TVIqNLI9IjJYAFNIqe4IZ7qfJCSIoDj2Tq0wJrEkXgW8kAheMzvmOVglr1SlSo3uweVaOgfGbwANak39MtplyksgH8GGgSv0k3ghLHeT06HbKt6MCVCi5fcFLXuCa0HZt7Dslg601YqJn36Hw031ObkJf1HFoNf8mdLHjCfDCXaUwWY9owqmYDL39Jh46P80sXa4u1IqKUfrFMmmCpF7MaVvtdMsJelz2zZHZUPSiC38xfUkOdcgRVcLVBv8GSKcqrMGo9QZs2fu9Zi25WuSZ0SzRo61TjBpRXm1MypIDnxTTEMMBA7l9L7TeojRak80SXhKGx1Pj4AKKNGiKYeIhyx3eSL1JzXmW9qABN6ex1MK4v8pdViMszPgWjeAL95hIWZHuQRMQkTW6A8zBIltmBrM7HAVXbgvMEN48MiacvF7uyC4ogptOw2M01RPX5vgrYS0uXiNUe3AkkPM52z73t6zcNtB1ey1p99HlvVi7ESkPfwQ6MWI2M0bJjru9qYll61idDW3H05v7fFtGg8Ic0MyMbzSX115GIMn6wadHubyaOLNCTJzsApcwuVDUb7uYxRkb53ZP4vVKbPqGugcQojjq22rYNTJt0frigyQYpXm8F1B06VcHnUj460kXEXrpep7UkPaRX5qloF5csnqStuutf9lDPSX8Yrfy6ptdS6FLys0gJpJvR1cDc2h1AfKYyRkflHWUShpJlyrxF4bsOR42vu5ZzX1OQZJaTMaiq1K8IlgzEIFzj9NVji7t26iIgtiZnq17twaw97L3U0I2RlVV6xF9oE27uF08ttTQ0D33VnrzeYBfyrHfrjouf44igELGwolxamYgmaT6NqWhLW45juzqmklNt33DoFRYfMImRrnAbh5zR20XLWAgspPDXgdd52b1sclR6DbAa43wQgdHpoPhSnYCGszGrN2vR1kyMRb32wf37BA725rcOBvfhQSFzNtTk1IqYDyPGUPsZTSknUq4oBRTFJhfzDMh6xy6950EyNAsfUd471kIFvg2dpprhbStY92ftm5TAiAorUXRCljzzU5hfJ6NQinsCmDSRcadtlgn1uThvdqi62xcmDlWDvCf5nKrmad1e3SEmyo99TjjoZXMMPtiGbq9YLEp96JP1TTlcPLHuDewAJNDjQN3ZYg0zQM6b1F3cAD5AgP8ZZc8pK1lJph05YzzV4Lindpx99zewUinVS60ipj4hKbQWNCJhlQCWXPURXI7J8RoLC9leZCqOyPMYEF0tosVmtA4yn1Vup7LJP8DhZ5Br5M0oFGPzmlBSztMj9Gpp0bHnqby5q4elF3KTncyAFDv5xtlN4pxFgclB22aCaT2j7BvNTgaLPQEfH1NQY3fdDzpQbAbGzdLjo77RbClagKYH2iCnq0lg885jMiavPL1NMtqOKPFc".to_string().as_bytes().to_vec()),
        )
        .unwrap();
}

#[test]
fn send_packet_success_needresponse() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(MOCK_CONTRACT_ADDR, "umlg", 2000);

    let _env = mock_env();

    let contract = CwCallService::default();

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    mock_deps.querier.update_wasm(|r| {
        let constract1 = Addr::unchecked(MOCK_CONTRACT_ADDR);
        let mut storage1 = HashMap::<Binary, Binary>::default();
        storage1.insert(b"the key".into(), b"the value".into());
        match r {
            WasmQuery::ContractInfo { contract_addr } => {
                if *contract_addr == constract1 {
                    let response = ContractInfoResponse::default();
                    SystemResult::Ok(ContractResult::Ok(to_binary(&response).unwrap()))
                } else {
                    SystemResult::Err(SystemError::NoSuchContract {
                        addr: contract_addr.clone(),
                    })
                }
            }
            WasmQuery::Smart {
                contract_addr: _,
                msg: _,
            } => SystemResult::Ok(ContractResult::Ok(to_binary(&10).unwrap())),
            _ => todo!(),
        }
    });

    contract
        .set_connection_host(mock_deps.as_mut().storage, Addr::unchecked("hostaddress"))
        .unwrap();
    contract
        .set_timeout_height(mock_deps.as_mut().storage, 10)
        .unwrap();

    contract
        .send_packet(
            mock_deps.as_mut(),
            mock_info,
            mock_env(),
            MOCK_CONTRACT_TO_ADDR.to_string(),
            vec![],
            vec![],
            vec![1, 2, 3],
            Some(vec![1, 2, 3]),
        )
        .unwrap();

    let result = contract
        .call_requests()
        .load(mock_deps.as_ref().storage, 1)
        .unwrap();

    assert!(result.enabled())
}
