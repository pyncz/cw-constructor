use super::helpers::all_unique;
use crate::contract::Contract;
use crate::error::{ContractError, ContractResult};
use crate::models::config::{ContractInfo, SlotConfig};
use crate::models::token::TokenConfig;
use crate::models::traits::Trait;
use cosmwasm_std::{Addr, Deps, StdResult};

impl<'a, TExtension, TTraitExtension, TMergedExtension>
    Contract<'a, TExtension, TTraitExtension, TMergedExtension>
{
    pub fn get_slot_config_by_address(
        &self,
        address: &Addr,
        deps: &Deps,
    ) -> StdResult<Option<SlotConfig>> {
        let config = self.config.load(deps.storage)?;
        Ok(config
            .slots
            .into_iter()
            .find(|slot| slot.allowed_contracts.contains(address)))
    }

    // Traits
    fn validate_trait(&self, input: &Trait<String>, deps: &Deps) -> ContractResult<Trait> {
        let traits = self.traits.load(deps.storage)?;

        let address = deps.api.addr_validate(&input.token.address)?;
        let token_id = input.token.token_id.to_owned();
        let parsed_trait = Trait {
            token_id: input.token_id.to_owned(),
            token: TokenConfig {
                token_id: token_id.to_owned(),
                address: address.to_owned(),
            },
        };

        // Validate if there's a specified slot for the provided token address
        let slot_config = self.get_slot_config_by_address(&address, deps)?;
        if slot_config.is_none() {
            return Err(ContractError::UnknownTraitAddress { address });
        }

        // Validate if the slot is not already taken for this base token
        if let Some(conf) = slot_config {
            if !conf.allow_multiple
                && traits
                    .iter()
                    .filter(|t| t.token_id == parsed_trait.token_id)
                    .any(|t| {
                        match self
                            .get_slot_config_by_address(&t.token.address, deps)
                            .unwrap_or(None)
                        {
                            Some(t_conf) => t_conf.name == conf.name,
                            _ => false,
                        }
                    })
            {
                return Err(ContractError::TraitSlotTaken {
                    token_id: parsed_trait.token_id,
                    slot: conf.name,
                });
            }
        }

        // Validate if the token is not listed as a trait for some base token already
        if traits
            .iter()
            .any(|t| address == t.token.address && *token_id == t.token.token_id)
        {
            return Err(ContractError::TraitTokenAlreadyApplied {
                address,
                token_id: token_id.to_owned(),
            });
        }

        Ok(parsed_trait)
    }

    pub fn validate_traits(
        &self,
        input: &Vec<Trait<String>>,
        deps: &Deps,
    ) -> ContractResult<Vec<Trait>> {
        // Validate traits' data against the current state
        let new_traits = input
            .iter()
            .map(|t| self.validate_trait(t, deps))
            .collect::<ContractResult<Vec<_>>>()?;

        // Validate traits against the peer traits in the traits to add:
        // - if there are no duplicate tokens
        if !all_unique(
            new_traits
                .iter()
                .map(|t| (&t.token.address, &t.token.token_id)),
        ) {
            return Err(ContractError::TraitDuplicateToken {});
        }

        // - if there are no tokens racing for the same base token's slot
        if !all_unique(
            new_traits
                .iter()
                .filter_map(|t| -> Option<(String, String)> {
                    match self
                        .get_slot_config_by_address(&t.token.address, deps)
                        .unwrap_or(None)
                    {
                        // Require slot+token uniqueness only among those slots where having multiple traits is not allowed
                        Some(slot_config) if !slot_config.allow_multiple => {
                            Some((t.token_id.to_owned(), slot_config.name))
                        }
                        _ => None,
                    }
                }),
        ) {
            return Err(ContractError::TraitTokenRace {});
        }

        Ok(new_traits)
    }
}

// Config
pub fn validate_slot_config(input: &SlotConfig<String>, deps: &Deps) -> ContractResult<SlotConfig> {
    // Validate allowed contract addresses
    let allowed_contracts: StdResult<Vec<_>> = input
        .allowed_contracts
        .iter()
        .map(|addr| deps.api.addr_validate(addr))
        .collect();

    Ok(SlotConfig {
        name: input.name.to_owned(),
        allowed_contracts: allowed_contracts?,
        allow_multiple: input.allow_multiple,
    })
}

pub fn validate_slot_configs(
    input: &Vec<SlotConfig<String>>,
    deps: &Deps,
) -> ContractResult<Vec<SlotConfig>> {
    // Validate uniqueness of the slot names
    if !all_unique(input.iter().map(|slot| &slot.name)) {
        return Err(ContractError::SlotConfigDuplicateName {});
    }

    // Validate uniqueness of the slots' allowed addresses
    if !all_unique(input.iter().flat_map(|slot| &slot.allowed_contracts)) {
        return Err(ContractError::SlotConfigDuplicateAddress {});
    }

    // Validate slots
    input
        .iter()
        .map(|slot| validate_slot_config(slot, deps))
        .collect()
}

pub fn parse_config(input: &ContractInfo<String>, deps: &Deps) -> ContractResult<ContractInfo> {
    // Validate admin addresses
    let admins: StdResult<Vec<_>> = input
        .admins
        .iter()
        .map(|addr| deps.api.addr_validate(addr))
        .collect();

    Ok(ContractInfo {
        base_token: deps.api.addr_validate(&input.base_token)?,
        slots: validate_slot_configs(&input.slots, deps)?,
        admins: admins?,
    })
}
