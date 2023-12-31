use crate::{
    msg::{GetConfigMsg, GetConfigResp, GetTokensMsg, GetTokensResp, GetTraitsMsg, GetTraitsResp},
    state::{CONFIG, TRAITS},
    utils::helpers::get_slot_config_by_address,
};
use cosmwasm_std::{Deps, StdResult};
use itertools::Itertools;

pub fn config(_msg: &GetConfigMsg, deps: &Deps) -> StdResult<GetConfigResp> {
    let config = CONFIG.load(deps.storage)?;

    Ok(GetConfigResp {
        base_token: config.base_token,
        slots: config.slots,
        admins: config.admins,
    })
}

pub fn traits(msg: &GetTraitsMsg, deps: &Deps) -> StdResult<GetTraitsResp> {
    let traits = TRAITS.load(deps.storage)?;

    // Apply filters from the message
    let filtered_traits = traits
        .into_iter()
        .filter(|t| {
            if let Some(token_id) = &msg.token_id {
                if *token_id != t.token_id {
                    return false;
                }
            }
            if let Some(slot) = &msg.slot {
                let t_conf = get_slot_config_by_address(&t.token.address, deps).unwrap_or(None);
                if Some(slot.to_owned()) != t_conf.map(|c| c.name) {
                    return false;
                }
            }
            true
        })
        .collect();

    Ok(GetTraitsResp {
        traits: filtered_traits,
    })
}

pub fn tokens(msg: &GetTokensMsg, deps: &Deps) -> StdResult<GetTokensResp> {
    let traits = TRAITS.load(deps.storage)?;

    // Apply filters from the message
    let tokens = traits
        .iter()
        .filter(|t| {
            if let Some(address) = msg.address.to_owned() {
                if t.token.address != address {
                    return false;
                }
                if let Some(token_id) = msg.token_id.to_owned() {
                    if t.token.token_id != token_id {
                        return false;
                    }
                }
            }
            true
        })
        .map(|t| t.token_id.to_owned())
        .unique()
        .collect::<Vec<String>>();

    Ok(GetTokensResp { tokens })
}
