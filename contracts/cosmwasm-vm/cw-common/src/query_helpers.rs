use cosmwasm_std::{Binary, Empty, QueryRequest, Deps, ContractInfoResponse, StdError};

pub fn build_smart_query(contract: String, msg: Binary) -> QueryRequest<Empty> {
    QueryRequest::Wasm(cosmwasm_std::WasmQuery::Smart {
        contract_addr: contract,
        msg,
    })
}

pub fn build_contract_info_query(address:String)->QueryRequest<Empty>{
    QueryRequest::Wasm(cosmwasm_std::WasmQuery::ContractInfo { contract_addr: address })
}

pub fn get_contract_info(deps:Deps, address:String)->Result<ContractInfoResponse,StdError>{
    let query= build_contract_info_query(address);
    let response:ContractInfoResponse=deps.querier.query(&query)?;
    Ok(response)

}
