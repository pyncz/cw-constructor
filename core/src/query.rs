use crate::{
    models::traits::{TraitResp, TraitWithMetadataResp},
    msg::{
        AllNftInfoMsg, AllNftInfoResp, ContractInfoMsg, ContractInfoResp, NftInfoMsg, NftInfoResp,
        TokensMsg, TokensResp, TraitsMsg, TraitsResp,
    },
    state::{CONFIG, TRAITS},
    utils::{helpers::get_slot_config_by_address, queries::cw721_info},
};
use cosmwasm_std::{Deps, StdResult};
use itertools::Itertools;
use serde::Deserialize;

pub fn contract_info(_msg: &ContractInfoMsg, deps: &Deps) -> StdResult<ContractInfoResp> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ContractInfoResp {
        base_token: config.base_token,
        slots: config.slots,
        admins: config.admins,
    })
}

pub fn traits(msg: &TraitsMsg, deps: &Deps) -> StdResult<TraitsResp> {
    let traits = TRAITS.load(deps.storage)?;

    // Apply filters from the message
    let filtered_traits = traits
        .into_iter()
        .filter_map(|t| {
            if let Some(token_id) = &msg.token_id {
                if *token_id != t.token_id {
                    return None;
                }
            }
            let t_slot = get_slot_config_by_address(&t.token.address, deps)
                .unwrap_or(None)
                .map(|s| s.name);
            if let Some(slot) = &msg.slot {
                if Some(slot.to_owned()) != t_slot {
                    return None;
                }
            }
            Some(TraitResp {
                token_id: t.token_id,
                token: t.token,
                slot: t_slot,
            })
        })
        .collect();

    Ok(TraitsResp {
        traits: filtered_traits,
    })
}

pub fn tokens(msg: &TokensMsg, deps: &Deps) -> StdResult<TokensResp> {
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

    Ok(TokensResp { tokens })
}

pub fn nft_info<T: for<'de> Deserialize<'de>>(
    msg: &NftInfoMsg,
    deps: &Deps,
) -> StdResult<NftInfoResp<T>> {
    // TODO: Aggregate traits' metadata
    let config = CONFIG.load(deps.storage)?;
    Ok(NftInfoResp {
        info: cw721_info::<T>(&config.base_token.to_string(), &msg.token_id, deps)?,
    })
}

pub fn all_nft_info<T: for<'de> Deserialize<'de>>(
    msg: &AllNftInfoMsg,
    deps: &Deps,
) -> StdResult<AllNftInfoResp<T>> {
    let config = CONFIG.load(deps.storage)?;
    let traits = TRAITS.load(deps.storage)?;

    let traits_info = traits
        .into_iter()
        .filter(|t| t.token_id == msg.token_id)
        .map(|t| {
            let slot = get_slot_config_by_address(&t.token.address, deps)
                .unwrap_or(None)
                .map(|s| s.name);
            Ok(TraitWithMetadataResp {
                token_id: t.token_id,
                token: t.token.to_owned(),
                info: cw721_info::<T>(&t.token.address.to_string(), &t.token.token_id, deps)?,
                slot,
            })
        })
        .collect::<StdResult<Vec<TraitWithMetadataResp<T>>>>()?;

    Ok(AllNftInfoResp {
        info: cw721_info::<T>(&config.base_token.to_string(), &msg.token_id, deps)?,
        traits: traits_info,
    })
}
