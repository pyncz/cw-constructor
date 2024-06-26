use super::{metadata::TokenMetadata, token::TokenConfig};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Trait<A: Into<String> + Clone = Addr> {
    /// token_id of the base token the trait is applied for
    pub token_id: String,
    /// `address` and `token_id` of the token to add as a trait
    pub token: TokenConfig<A>,
}

#[cw_serde]
pub struct TraitResp {
    pub token_id: String,
    pub token: TokenConfig<Addr>,
    pub slot: String,
}

#[cw_serde]
pub struct TraitWithMetadataResp<TTraitExtension> {
    pub token_id: String,
    pub token: TokenConfig<Addr>,
    pub slot: String,
    pub info: TokenMetadata<TTraitExtension>,
}
