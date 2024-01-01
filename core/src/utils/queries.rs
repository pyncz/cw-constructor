use cosmwasm_std::{to_json_binary, Deps, QueryRequest, StdResult, WasmQuery};
use cw721::{ContractInfoResponse, NftInfoResponse, OwnerOfResponse};
use cw721_base::QueryMsg as Cw721QueryMsg;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::models::metadata::TokenMetadata;

fn query_smart<Msg: Serialize, Resp: DeserializeOwned>(
    address: &String,
    msg: &Msg,
    deps: &Deps,
) -> StdResult<Resp> {
    let req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: address.to_owned(),
        msg: to_json_binary(&msg)?,
    });

    deps.querier.query::<Resp>(&req)
}

pub fn cw721_owner_of(
    address: &String,
    token_id: &String,
    deps: &Deps,
) -> StdResult<OwnerOfResponse> {
    query_smart(
        address,
        &Cw721QueryMsg::OwnerOf {
            token_id: token_id.to_owned(),
            include_expired: None,
        },
        deps,
    )
}

pub fn cw721_contract_info(address: &String, deps: &Deps) -> StdResult<ContractInfoResponse> {
    query_smart(address, &Cw721QueryMsg::ContractInfo {}, deps)
}

pub fn cw721_nft_info<T: for<'de> Deserialize<'de>>(
    address: &String,
    token_id: &String,
    deps: &Deps,
) -> StdResult<NftInfoResponse<T>> {
    query_smart(
        address,
        &Cw721QueryMsg::NftInfo {
            token_id: token_id.to_string(),
        },
        deps,
    )
}

pub fn cw721_info<T: for<'de> Deserialize<'de>>(
    address: &String,
    token_id: &String,
    deps: &Deps,
) -> StdResult<TokenMetadata<T>> {
    Ok(TokenMetadata {
        token: cw721_nft_info(address, token_id, deps)?,
        contract: cw721_contract_info(address, deps)?,
    })
}
