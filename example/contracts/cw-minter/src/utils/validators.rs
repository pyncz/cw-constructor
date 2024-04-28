use crate::{
    error::{ContractError, ContractResult},
    models::config::ContractInfo,
};
use cosmwasm_std::{Coin, Deps};

pub fn parse_config<TExtension: Clone>(
    input: &ContractInfo<TExtension, String>,
    deps: &Deps,
) -> ContractResult<ContractInfo<TExtension>> {
    // Validate cw721 address
    let cw721 = deps.api.addr_validate(&input.cw721)?;

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

    Ok(ContractInfo {
        supply: input.supply,
        price: input.price.clone(),
        extensions: input.extensions.clone(),
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
