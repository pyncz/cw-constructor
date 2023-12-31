use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use std::fmt::Display;

#[cw_serde]
pub struct TokenConfig<A: Into<String> = Addr> {
    pub address: A,
    pub token_id: String,
}

impl<T> Into<String> for TokenConfig<T>
where
    T: Into<String> + Display,
{
    fn into(self) -> String {
        format!("{}:{}", self.address, self.token_id)
    }
}
