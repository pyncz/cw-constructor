use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

use crate::models::TokenConfig;

// Instantiate message
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub base_token: TokenConfig<String>,
    pub allowed_traits_addresses: Option<Vec<String>>,
    pub admins: Option<Vec<String>>,
}

// Query messages
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig(GetConfigMsg),
    GetTraits(GetTraitsMsg),
}

// - GetConfig
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GetConfigMsg {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GetConfigResp {
    pub base_token: TokenConfig,
    pub allowed_traits_addresses: Vec<Addr>,
    pub admins: Vec<Addr>,
}

// - GetTraits
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GetTraitsMsg {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GetTraitsResp {
    pub traits: Vec<TokenConfig>,
}

// Execute messages
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Apply(ApplyMsg),
    Exempt(ExemptMsg),
    ExemptAll(ExemptAllMsg),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ApplyMsg {
    pub traits: Vec<TokenConfig>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ExemptMsg {
    pub traits: Vec<TokenConfig>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ExemptAllMsg {}
