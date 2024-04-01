use cosmwasm_schema::cw_serde;
use cw721::{ContractInfoResponse, NftInfoResponse};

pub trait MergeWithTraitExtension<TTraitExtension, TExtension> {
    fn merge(&mut self, extension: &TTraitExtension, original_extension: &TExtension);
}

#[cw_serde]
pub struct TokenMetadata<T> {
    pub contract: ContractInfoResponse,
    pub token: NftInfoResponse<T>,
}
