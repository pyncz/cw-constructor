use crate::contract::Contract;
use crate::error::{ContractError, ContractResult};
use cosmwasm_std::{Deps, MessageInfo};
use serde::de::DeserializeOwned;
use serde::Serialize;

impl<'a, TExtension> Contract<'a, TExtension>
where
    TExtension: Serialize + DeserializeOwned + Clone,
{
    pub fn require_instantiated(&self, deps: &Deps, _info: &MessageInfo) -> ContractResult {
        let config = self.config.may_load(deps.storage)?;
        if config.is_none() {
            return Err(ContractError::NotInstantiated {});
        }
        Ok(())
    }
}
