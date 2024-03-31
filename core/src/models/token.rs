use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

impl<T, A> PartialEq<TokenConfig<A>> for TokenConfig<T>
where
    A: Into<String> + Clone,
    T: Into<String> + Clone,
{
    fn eq(&self, other: &TokenConfig<A>) -> bool {
        self.address.clone().into() == other.address.clone().into()
            && self.token_id == other.token_id
    }
}
