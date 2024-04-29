use crate::contract::Contract;
use crate::error::{ContractError, ContractResult};
use crate::models::config::ContractInfo;
use cosmwasm_std::{Deps, MessageInfo};
use serde::de::DeserializeOwned;
use serde::Serialize;

impl<'a, TExtension> Contract<'a, TExtension>
where
    TExtension: Serialize + DeserializeOwned + Clone,
{
    pub fn require_instantiated(
        &self,
        deps: &Deps,
        _info: &MessageInfo,
    ) -> ContractResult<ContractInfo<TExtension>> {
        self.config
            .may_load(deps.storage)?
            .ok_or(ContractError::NotInstantiated {})
    }

    pub fn require_admin(&self, deps: &Deps, info: &MessageInfo) -> ContractResult {
        let config = self.config.load(deps.storage)?;
        if !config.admins.contains(&info.sender) {
            return Err(ContractError::NotAdmin {
                sender: info.sender.to_owned(),
            });
        }
        Ok(())
    }
}
