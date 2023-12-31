use cosmwasm_schema::cw_serde;
use cw721::{ContractInfoResponse, NftInfoResponse};

#[cw_serde]
pub struct Metadata<T> {
    pub contract: ContractInfoResponse,
    pub token: NftInfoResponse<T>,
}

#[cw_serde]
pub struct TraitMetadata<T> {
    pub slot: String,
    pub contract: ContractInfoResponse,
    pub token: NftInfoResponse<T>,
}

#[cw_serde]
pub struct ConstructorMetadata<T> {
    /// Metadata of the base cw721 token
    pub base_token: Metadata<T>,
    /// List of each applied traits' metadata
    pub traits: Vec<TraitMetadata<T>>,
}
