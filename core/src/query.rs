use crate::{
    contract::Contract,
    models::{
        metadata::MergeWithTraitExtension,
        traits::{TraitResp, TraitWithMetadataResp},
    },
    msg::{
        ContractInfoMsg, ContractInfoResp, InfoMsg, InfoResp, TokensMsg, TokensResp, TraitsMsg,
        TraitsResp,
    },
    utils::queries::cw721_info,
};
use cosmwasm_std::{Deps, StdResult};
use cw721::NftInfoResponse;
use itertools::Itertools;
use serde::{de::DeserializeOwned, Serialize};

impl<'a, TExtension, TTraitExtension, TMergedExtension>
    Contract<'a, TExtension, TTraitExtension, TMergedExtension>
where
    TExtension: Serialize + DeserializeOwned + Clone,
    TTraitExtension: Serialize + DeserializeOwned,
    TMergedExtension:
        Serialize + DeserializeOwned + MergeWithTraitExtension<TTraitExtension> + From<TExtension>,
{
    /// Get contract configuration, including `base_token`, `admins` and trait slots' setup
    pub fn contract_info(
        &self,
        _msg: &ContractInfoMsg,
        deps: &Deps,
    ) -> StdResult<ContractInfoResp> {
        self.config.load(deps.storage)
    }

    /// List trait tokens featured in the contract
    /// filtered by applied `slot` and their base token's `token_id`
    pub fn traits(&self, msg: &TraitsMsg, deps: &Deps) -> StdResult<TraitsResp> {
        let traits = self.traits.load(deps.storage)?;

        // Equip filters from the message
        let filtered_traits = traits
            .into_iter()
            .filter_map(|t| {
                if let Some(token_id) = &msg.token_id {
                    if *token_id != t.token_id {
                        return None;
                    }
                }
                let t_slot = self
                    .get_slot_config_by_address(&t.token.address, deps)
                    .unwrap_or(None)?
                    .name;
                if let Some(slot) = &msg.slot {
                    if *slot != t_slot {
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
    pub fn tokens(&self, msg: &TokensMsg, deps: &Deps) -> StdResult<TokensResp> {
        let traits = self.traits.load(deps.storage)?;

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
            .collect();

        Ok(TokensResp { tokens })
    }

    /// Get metadata of base token and its applied trait tokens
    pub fn info(
        &self,
        msg: &InfoMsg,
        deps: &Deps,
    ) -> StdResult<InfoResp<TExtension, TTraitExtension, TMergedExtension>> {
        let config = self.config.load(deps.storage)?;
        let traits = self.traits.load(deps.storage)?;

        // Raise the error in case the base token is burned
        let base_token_info =
            cw721_info::<TExtension>(&config.base_token.to_string(), &msg.token_id, deps)?;

        let traits_info = traits
            .into_iter()
            .filter(|t| t.token_id == msg.token_id)
            .filter_map(|t| {
                let slot = self
                    .get_slot_config_by_address(&t.token.address, deps)
                    .unwrap_or(None)?
                    .name;
                // Ignore the trait in case the token is burned
                cw721_info::<TTraitExtension>(&t.token.address.to_string(), &t.token.token_id, deps)
                    .ok()
                    .map(|info| TraitWithMetadataResp {
                        token_id: t.token_id,
                        token: t.token.to_owned(),
                        info,
                        slot,
                    })
            })
            .collect::<Vec<_>>();

        let mut merged_extension: TMergedExtension = base_token_info.token.extension.clone().into();
        merged_extension.merge(
            traits_info
                .iter()
                .map(|t| &t.info.token.extension)
                .collect(),
        );

        Ok(InfoResp {
            info: NftInfoResponse {
                token_uri: base_token_info.token.token_uri.clone(),
                extension: merged_extension,
            },
            base_token: base_token_info,
            traits: traits_info,
        })
    }
}
