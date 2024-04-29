use crate::models::config::ContractInfo;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Empty;

// Instantiate message
pub type InstantiateMsg<TExtension> = ContractInfo<TExtension, String>;

// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Return contract info
    #[returns(ContractInfoResp<Empty>)]
    ContractInfo(ContractInfoMsg),
}

// - ContractInfo
#[cw_serde]
pub struct ContractInfoMsg {}

pub type ContractInfoResp<T> = ContractInfo<T>;

// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// Mint related cw721 token
    Mint(MintMsg),

    /// Set related cw721 contract address
    SetCw721(SetCw721Msg),
}

// - Mint
#[cw_serde]
pub struct MintMsg {}

// - Set cw721
#[cw_serde]
pub struct SetCw721Msg {
    pub address: String,
}
