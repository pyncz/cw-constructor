use super::weights::WeightedOption;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

pub type ExtensionsConfig<TExtension> = WeightedOption<TExtension>;

impl<TExtension: Default> Default for ExtensionsConfig<TExtension> {
    fn default() -> Self {
        Self {
            value: TExtension::default(),
            weight: 1,
        }
    }
}

#[cw_serde]
pub struct ContractInfo<TExtension, A: Into<String> = Addr> {
    /// NFT contract to mint tokens of
    pub cw721: Option<A>,

    /// Maximum supply
    pub supply: Option<u32>,

    /// Price in native token
    pub price: Option<Coin>,

    /// Config of metadata variants and their probabilities
    pub extensions: Vec<ExtensionsConfig<TExtension>>,

    /// Addresses of the accounts authorized to alter the contract config
    pub admins: Vec<A>,
}
