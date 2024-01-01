use cosmwasm_schema::cw_serde;
use cw721::{ContractInfoResponse, NftInfoResponse};

#[cw_serde]
pub struct TokenMetadata<T> {
    pub contract: ContractInfoResponse,
    pub token: NftInfoResponse<T>,
}
