use cosmwasm_schema::cw_serde;
use cw721::{ContractInfoResponse, NftInfoResponse};

pub trait MergeWithTraitExtension<TTraitExtension> {
    fn merge(&mut self, extensions: Vec<&TTraitExtension>);
}

#[cw_serde]
pub struct TokenMetadata<T> {
    pub contract: ContractInfoResponse,
    pub token: NftInfoResponse<T>,
}
