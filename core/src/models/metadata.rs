use cosmwasm_schema::cw_serde;
use cw721::{ContractInfoResponse, NftInfoResponse};

pub trait MergeWithTraitExtension<TExtension> {
    fn merge(&mut self, extension: &TExtension);
}

#[cw_serde]
pub struct TokenMetadata<T> {
    pub contract: ContractInfoResponse,
    pub token: NftInfoResponse<T>,
}
