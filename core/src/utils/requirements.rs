use super::queries::cw721_owner_of;
use crate::error::{ContractError, ContractResult};
use crate::state::CONFIG;
use cosmwasm_std::{Deps, MessageInfo};

#[allow(dead_code)]
pub fn require_instantiated(deps: &Deps, _info: &MessageInfo) -> ContractResult {
    let config = CONFIG.may_load(deps.storage)?;
    if config.is_none() {
        return Err(ContractError::NotInstantiated {});
    }
    Ok(())
}

#[allow(dead_code)]
pub fn require_admin(deps: &Deps, info: &MessageInfo) -> ContractResult {
    let config = CONFIG.load(deps.storage)?;
    if !config.admins.contains(&info.sender) {
        return Err(ContractError::NotAdmin {
            sender: info.sender.to_owned(),
        });
    }
    Ok(())
}

#[allow(dead_code)]
pub fn require_sender(addresses: &Vec<String>, _deps: &Deps, info: &MessageInfo) -> ContractResult {
    if !addresses.contains(&info.sender.to_string()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender.to_owned(),
        });
    }
    Ok(())
}

#[allow(dead_code)]
pub fn require_sender_cw721_approval<A: ToString>(
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

    require_sender(&spenders, deps, info)
}
