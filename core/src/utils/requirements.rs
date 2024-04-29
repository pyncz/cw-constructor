use super::queries::cw721_owner_of;
use crate::contract::Contract;
use crate::error::{ContractError, ContractResult};
use crate::models::config::ContractInfo;
use cosmwasm_std::{Deps, MessageInfo};

impl<'a, TExtension, TTraitExtension, TMergedExtension>
    Contract<'a, TExtension, TTraitExtension, TMergedExtension>
{
    pub fn require_instantiated(
        &self,
        deps: &Deps,
        _info: &MessageInfo,
    ) -> ContractResult<ContractInfo> {
        let config = self.config.may_load(deps.storage)?;
        config.ok_or(ContractError::NotInstantiated {})
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

    pub fn require_sender(
        &self,
        addresses: &Vec<String>,
        _deps: &Deps,
        info: &MessageInfo,
    ) -> ContractResult {
        if !addresses.contains(&info.sender.to_string()) {
            return Err(ContractError::Unauthorized {
                sender: info.sender.to_owned(),
            });
        }
        Ok(())
    }

    pub fn require_sender_cw721_approval<A: ToString>(
        &self,
        address: A,
        token_id: &String,
        deps: &Deps,
        info: &MessageInfo,
    ) -> ContractResult {
        let owner_of_res = cw721_owner_of(&address.to_string(), &token_id, &deps)?;
        let mut spenders: Vec<_> = owner_of_res
            .approvals
            .into_iter()
            .map(|a| a.spender)
            .collect();
        spenders.push(owner_of_res.owner);

        self.require_sender(&spenders, deps, info)
    }
}
