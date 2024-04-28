use super::weights::WeightedOption;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

pub type ExtensionsConfig<TExtension> = WeightedOption<TExtension>;

#[cw_serde]
pub struct ContractInfo<TExtension: Clone, A: Into<String> = Addr> {
    /// NFT contract to mint tokens of
    pub cw721: A,

    /// Maximum supply
    pub supply: Option<u32>,

    /// Price in native token
    pub price: Option<Coin>,

    /// Config of metadata variants and their probabilities
    pub extensions: Vec<ExtensionsConfig<TExtension>>,
}
