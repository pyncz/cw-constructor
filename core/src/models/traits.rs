use super::token::TokenConfig;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Trait<A: Into<String> = Addr> {
    /// token_id of the base token the trait is applied for
    pub token_id: String,
    /// `address` and `token_id` of the token to add as a trait
    pub token: TokenConfig<A>,
}
