use cosmwasm_std::{Binary, QueryRequest, Empty};

pub fn build_smart_query(contract: String, msg: Binary) -> QueryRequest<Empty> {
    QueryRequest::Wasm(cosmwasm_std::WasmQuery::Smart {
        contract_addr: contract,
        msg,
    })
}

