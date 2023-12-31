use cosmwasm_std::{to_json_binary, Deps, QueryRequest, StdResult, WasmQuery};
use cw721::OwnerOfResponse as Cw721OwnerOfResponse;
use cw721_base::QueryMsg as Cw721QueryMsg;

pub fn cw721_owner_of(
    address: &str,
    token_id: &str,
    deps: &Deps,
) -> StdResult<Cw721OwnerOfResponse> {
    let msg = Cw721QueryMsg::OwnerOf {
        token_id: token_id.to_owned(),
        include_expired: None,
    };
    let req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: address.to_string(),
        msg: to_json_binary(&msg)?,
    });

    deps.querier.query::<Cw721OwnerOfResponse>(&req)
}
