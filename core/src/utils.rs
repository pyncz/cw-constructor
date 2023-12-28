use crate::state::{CONFIG, TRAITS};
use crate::{error::ContractError, models::TokenConfig};
use cosmwasm_std::{Addr, Deps};

pub fn validate_trait(
    new_trait: &TokenConfig<String>,
    deps: Deps,
) -> Result<TokenConfig<Addr>, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let traits = TRAITS.load(deps.storage)?;
    let address = deps.api.addr_validate(&new_trait.address).unwrap();

    // The allowed addresses are specified and there's no current trait in it
    if config.allowed_traits_addresses.len() > 0
        && !config.allowed_traits_addresses.contains(&address)
    {
        Err(ContractError::TraitContractNotAllowed { address })
    // The token is already listed as a trait
    } else if traits.into_iter().any(|t| {
        new_trait.address == t.address
            && (!config.allow_multiple_tokens_per_contract || new_trait.token_id == t.token_id)
    }) {
        Err(ContractError::TraitTokenAlreadyApplied {
            address,
            token_id: new_trait.token_id.clone(),
        })
    // It's a valid token to be added as a trait
    } else {
        Ok(TokenConfig {
            address,
            token_id: new_trait.token_id.clone(),
        })
    }
}
