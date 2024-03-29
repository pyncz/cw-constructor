use crate::{
    models::{
        metadata::{MergeWithTraitExtension, TokenMetadata},
        traits::{TraitResp, TraitWithMetadataResp},
    },
    msg::{
        AllNftInfoMsg, AllNftInfoResp, ContractInfoMsg, ContractInfoResp, NftInfoMsg, NftInfoResp,
        TokensMsg, TokensResp, TraitsMsg, TraitsResp,
    },
    state::{CONFIG, TRAITS},
    utils::{
        helpers::get_slot_config_by_address,
        queries::{cw721_info, cw721_nft_info},
    },
};
use cosmwasm_std::{Deps, StdResult};
use cw721::NftInfoResponse;
use itertools::Itertools;
use serde::Deserialize;

/// Get contract configuration, including `base_token`, `admins` and trait slots' setup
pub fn contract_info(_msg: &ContractInfoMsg, deps: &Deps) -> StdResult<ContractInfoResp> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ContractInfoResp {
        base_token: config.base_token,
        slots: config.slots,
        admins: config.admins,
    })
}

/// List trait tokens featured in the contract
/// filtered by applied `slot` and their base token's `token_id`
pub fn traits(msg: &TraitsMsg, deps: &Deps) -> StdResult<TraitsResp> {
    let traits = TRAITS.load(deps.storage)?;

    // Equip filters from the message
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

/// List token IDs of the base tokens managed by the contract
/// filtered by `address` and `token_id` of the traits it features
pub fn tokens(msg: &TokensMsg, deps: &Deps) -> StdResult<TokensResp> {
    let traits = TRAITS.load(deps.storage)?;

    // Equip filters from the message
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

/// Get merged metadata of base token and its applied trait tokens
pub fn nft_info<
    TExtension: for<'de> Deserialize<'de>,
    TTraitExtension: for<'de> Deserialize<'de>,
    TMergedExtension: MergeWithTraitExtension<TTraitExtension> + From<TExtension>,
>(
    msg: &NftInfoMsg,
    deps: &Deps,
) -> StdResult<NftInfoResp<TMergedExtension>> {
    let config = CONFIG.load(deps.storage)?;
    let traits = TRAITS.load(deps.storage)?;

    let base_token_info =
        cw721_info::<TExtension>(&config.base_token.to_string(), &msg.token_id, deps)?;

    let merged_extension: TMergedExtension = traits
        .into_iter()
        .filter(|t| t.token_id == msg.token_id)
        .fold(base_token_info.token.extension.into(), |mut acc, t| {
            // Applied trait's info
            let trait_info = cw721_nft_info::<TTraitExtension>(
                &t.token.address.to_string(),
                &t.token.token_id,
                deps,
            );

            if let Ok(trait_info) = trait_info {
                // Aggregate the initial base token's metadata with the trait metadata
                acc.merge(&trait_info.extension);
            }

            acc
        });

    Ok(NftInfoResp {
        info: TokenMetadata {
            contract: base_token_info.contract,
            token: NftInfoResponse {
                token_uri: base_token_info.token.token_uri,
                extension: merged_extension,
            },
        },
    })
}

/// Get separate metadata of base token and its trait tokens
pub fn all_nft_info<
    TExtension: for<'de> Deserialize<'de>,
    TTraitExtension: for<'de> Deserialize<'de>,
>(
    msg: &AllNftInfoMsg,
    deps: &Deps,
) -> StdResult<AllNftInfoResp<TExtension, TTraitExtension>> {
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
                info: cw721_info::<TTraitExtension>(
                    &t.token.address.to_string(),
                    &t.token.token_id,
                    deps,
                )?,
                slot,
            })
        })
        .collect::<StdResult<Vec<TraitWithMetadataResp<TTraitExtension>>>>()?;

    Ok(AllNftInfoResp {
        info: cw721_info::<TExtension>(&config.base_token.to_string(), &msg.token_id, deps)?,
        traits: traits_info,
    })
}
