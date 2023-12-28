use crate::{
    error::ContractError,
    events::{ACTION, APPLY_ACTION, APPLY_EVENT, EXEMPT_ACTION, EXEMPT_ALL_ACTION, EXEMPT_EVENT},
    models::TokenConfig,
    msg::{ApplyMsg, ExemptAllMsg, ExemptMsg},
    state::TRAITS,
    utils::validate_trait,
};
use cosmwasm_std::{Attribute, DepsMut, Response, StdResult};

/// Add provided tokens as traits
pub fn execute_apply(msg: ApplyMsg, deps: DepsMut) -> Result<Response, ContractError> {
    let traits_to_add: Result<Vec<_>, ContractError> = msg
        .traits
        .into_iter()
        .map(|t| validate_trait(&t, deps.as_ref()))
        .collect();
    let mut traits_to_add = traits_to_add.unwrap();

    TRAITS.update(deps.storage, |mut traits| -> Result<_, ContractError> {
        traits.append(&mut traits_to_add);
        Ok(traits)
    })?;
    Ok(Response::new()
        .add_attribute(ACTION, APPLY_ACTION)
        .add_attributes(
            traits_to_add
                .into_iter()
                .map(|t| Attribute::new(APPLY_EVENT, format!("{}:{}", t.address, t.token_id))),
        ))
}

/// Remove provided tokens as traits
pub fn execute_exempt(msg: ExemptMsg, deps: DepsMut) -> Result<Response, ContractError> {
    let traits_to_remove: StdResult<Vec<_>> = msg
        .traits
        .into_iter()
        .map(|t| {
            let address = deps.api.addr_validate(&t.address).unwrap();
            Ok(TokenConfig {
                address,
                token_id: t.token_id,
            })
        })
        .collect();
    let traits_to_remove = traits_to_remove.unwrap();

    TRAITS.update(deps.storage, |traits| -> Result<_, ContractError> {
        let traits = traits
            .into_iter()
            .filter(|current_t| {
                !(&traits_to_remove)
                    .into_iter()
                    .any(|t| t.address == current_t.address && t.token_id == current_t.token_id)
            })
            .collect();
        Ok(traits)
    })?;
    Ok(Response::new()
        .add_attribute(ACTION, EXEMPT_ACTION)
        .add_attributes(
            traits_to_remove
                .into_iter()
                .map(|t| Attribute::new(EXEMPT_EVENT, format!("{}:{}", t.address, t.token_id))),
        ))
}

/// Reset all added traits
pub fn execute_exempt_all(_msg: ExemptAllMsg, deps: DepsMut) -> Result<Response, ContractError> {
    let current_traits = TRAITS.load(deps.storage)?;

    TRAITS.save(deps.storage, &vec![])?;
    Ok(Response::new()
        .add_attribute(ACTION, EXEMPT_ALL_ACTION)
        .add_attributes(
            current_traits
                .into_iter()
                .map(|t| Attribute::new(EXEMPT_EVENT, format!("{}:{}", t.address, t.token_id))),
        ))
}
