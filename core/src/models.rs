use cosmwasm_std::Addr;
use cw721::{ContractInfoResponse, NftInfoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Metadata<T> {
    pub contract: ContractInfoResponse,
    pub token: NftInfoResponse<T>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TokenConfig<A: Into<String> = Addr> {
    pub address: A,
    pub token_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Config {
    /// Address and token_id of the base cw721 token to be extended
    pub base_token: TokenConfig,
    /// Addresses of the contracts which tokens are allowed to be added as traits
    pub allowed_traits_addresses: Vec<Addr>,

    pub admins: Vec<Addr>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ConstructorMetadata<T> {
    /// Metadata of the base cw721 token
    pub base_token: Metadata<T>,
    /// List of each applied traits' metadata
    pub traits: Vec<Metadata<T>>,
}
