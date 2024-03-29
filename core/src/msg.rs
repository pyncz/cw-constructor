use crate::models::metadata::TokenMetadata;
use crate::models::traits::{TraitResp, TraitWithMetadataResp};
use crate::models::{config::SlotConfig, token::TokenConfig};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Empty};

// Instantiate message
#[cw_serde]
pub struct InstantiateMsg {
    pub base_token: String,
    pub slots: Vec<SlotConfig<String>>,
    pub admins: Option<Vec<String>>,
}

// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get constructor contract's config
    #[returns(ContractInfoResp)]
    ContractInfo(ContractInfoMsg),

    /// Get filtered traits, i.e. trait tokens and a slot
    #[returns(TraitsResp)]
    Traits(TraitsMsg),

    /// Get filtered base tokens
    #[returns(TokensResp)]
    Tokens(TokensMsg),

    /// Get *aggregated* metadata of the base token and its applied trait tokens
    #[returns(NftInfoResp<Empty>)]
    NftInfo(NftInfoMsg),

    /// Get metadata of the base token and its applied trait tokens *separately*
    #[returns(AllNftInfoResp<Empty, Empty>)]
    AllNftInfo(AllNftInfoMsg),
}

// - ContractInfo
#[cw_serde]
pub struct ContractInfoMsg {}

#[cw_serde]
pub struct ContractInfoResp {
    pub base_token: Addr,
    pub slots: Vec<SlotConfig>,
    pub admins: Vec<Addr>,
}

// - Traits
#[cw_serde]
pub struct TraitsMsg {
    /// Filter by `name` of the slot
    pub slot: Option<String>,
    /// Filter by `token_id` of the base token
    pub token_id: Option<String>,
}

#[cw_serde]
pub struct TraitsResp {
    pub traits: Vec<TraitResp>,
}

// - Tokens
#[cw_serde]
pub struct TokensMsg {
    /// Filter by `token_id` of the trait token
    pub token_id: Option<String>,
    /// Filter by `address` of the trait token
    pub address: Option<String>,
}

#[cw_serde]
pub struct TokensResp {
    pub tokens: Vec<String>,
}

// - NftInfo
#[cw_serde]
pub struct NftInfoMsg {
    pub token_id: String,
}

#[cw_serde]
pub struct NftInfoResp<TExtension> {
    pub info: TokenMetadata<TExtension>,
}

// - AllNftInfo
#[cw_serde]
pub struct AllNftInfoMsg {
    pub token_id: String,
}

#[cw_serde]
pub struct AllNftInfoResp<TExtension, TTraitExtension> {
    pub info: TokenMetadata<TExtension>,
    pub traits: Vec<TraitWithMetadataResp<TTraitExtension>>,
}

// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// Equip new trait tokens to the base token
    Equip(EquipMsg),

    /// Remove applied trait tokens
    Unequip(UnequipMsg),
}

// - Equip
#[cw_serde]
pub struct EquipMsg {
    /// `token_id` of the base token
    pub token_id: String,
    /// `address` and `token_id` of the traits to equip
    pub traits: Vec<TokenConfig<String>>,
}

// - Unequip
#[cw_serde]
pub struct UnequipMsg {
    /// `address` and `token_id` of the traits to remove
    pub traits: Vec<TokenConfig<String>>,
}
