use crate::{
    error::{ContractError, ContractResult},
    models::config::ContractInfo,
};
use cosmwasm_std::{Addr, Coin, Deps, StdResult};

pub fn parse_address(input: &str, deps: &Deps) -> ContractResult<Addr> {
    Ok(deps.api.addr_validate(input)?)
}

pub fn parse_config<TExtension: Clone>(
    input: &ContractInfo<TExtension, String>,
    deps: &Deps,
) -> ContractResult<ContractInfo<TExtension>> {
    // Validate cw721 address
    let cw721 = match &input.cw721 {
        Some(cw721_input) => Some(parse_address(cw721_input, deps)?),
        _ => None,
    };

    // Validate supply
    if let Some(supply) = input.supply {
        if supply == 0 {
            return Err(ContractError::InvalidSupply { supply });
        }
    }

    // Validate price
    if let Some(price) = &input.price {
        if price.amount.is_zero() {
            return Err(ContractError::InvalidAmount {
                amount: price.amount.into(),
            });
        }
    }

    // Validate admins' list
    let admins: StdResult<Vec<_>> = input
        .admins
        .iter()
        .map(|addr| deps.api.addr_validate(addr))
        .collect();
    // If no admins provided, require cw721 being set already to prevent having unsettable contract
    if input.admins.is_empty() && cw721.is_none() {
        return Err(ContractError::NoAdminOrCw721ContractProvided {});
    }

    // Validate extensions
    if input.extensions.is_empty() {
        return Err(ContractError::NoExtensionProvided {});
    }

    Ok(ContractInfo {
        supply: input.supply,
        price: input.price.clone(),
        extensions: input.extensions.clone(),
        admins: admins?,
        cw721,
    })
}

pub fn validate_funds(funds: &[Coin], required_funds: &Coin) -> ContractResult<()> {
    let required_amount = required_funds.amount.u128();
    if required_amount > 0 {
        // Aggregate the amount of the required denom sent
        let amount_sent = funds.iter().fold(0u128, |mut acc, c| {
            if c.denom == required_funds.denom {
                acc += c.amount.u128();
            }
            acc
        });

        // Return an error if not sufficient
        if amount_sent < required_amount {
            return Err(ContractError::InsufficientFunds {
                denom: required_funds.denom.clone(),
                required: required_amount,
                sent: amount_sent,
            });
        }
    }

    Ok(())
}
