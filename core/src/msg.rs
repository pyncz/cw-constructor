use crate::models::traits::Trait;
use crate::models::{config::SlotConfig, token::TokenConfig};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

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
    #[returns(GetConfigResp)]
    GetConfig(GetConfigMsg),

    #[returns(GetTraitsResp)]
    GetTraits(GetTraitsMsg),

    #[returns(GetTokensResp)]
    GetTokens(GetTokensMsg),
}

// - GetConfig
#[cw_serde]
pub struct GetConfigMsg {}

#[cw_serde]
pub struct GetConfigResp {
    pub base_token: Addr,
    pub slots: Vec<SlotConfig>,
    pub admins: Vec<Addr>,
}

// - GetTraits
#[cw_serde]
pub struct GetTraitsMsg {
    /// Filter by `name` of the slot
    pub slot: Option<String>,
    /// Filter by `token_id` of the base token
    pub token_id: Option<String>,
}

#[cw_serde]
pub struct GetTraitsResp {
    pub traits: Vec<Trait>,
}

// - GetTokens
#[cw_serde]
pub struct GetTokensMsg {
    /// Filter by `token_id` of the trait token
    pub token_id: Option<String>,
    /// Filter by `address` of the trait token
    pub address: Option<String>,
}

#[cw_serde]
pub struct GetTokensResp {
    pub tokens: Vec<String>,
}

// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    Apply(ApplyMsg),
    Exempt(ExemptMsg),
}

#[cw_serde]
pub struct ApplyMsg {
    /// `token_id` of the base token
    pub token_id: String,
    /// `address` and `token_id` of the traits to apply
    pub traits: Vec<TokenConfig<String>>,
}

#[cw_serde]
pub struct ExemptMsg {
    ///
    pub traits: Vec<TokenConfig<String>>,
}
