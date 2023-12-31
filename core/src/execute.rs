use crate::{
    error::{ContractResponse, ContractResult},
    events::{ACTION, APPLY_ACTION, APPLY_EVENT, EXEMPT_ACTION, EXEMPT_EVENT, EXTEND_EVENT},
    models::{token::TokenConfig, traits::Trait},
    msg::{ApplyMsg, ExemptMsg},
    state::{CONFIG, TRAITS},
    utils::{
        requirements::{require_instantiated, require_sender_cw721_approval},
        validators::validate_traits,
    },
};
use cosmwasm_std::{Attribute, DepsMut, MessageInfo, Response};

/// Add provided tokens as traits
pub fn apply(msg: ApplyMsg, deps: DepsMut, info: MessageInfo) -> ContractResponse {
    require_instantiated(&deps.as_ref(), &info)?;

    let config = CONFIG.load(deps.storage)?;

    // To apply traits, the sender must be:
    // - the base token's owner / approved spender
    require_sender_cw721_approval(&config.base_token, &msg.token_id, &deps.as_ref(), &info)?;
    // - the trait tokens' owner / approved spender
    msg.traits
        .iter()
        .map(|t| require_sender_cw721_approval(&t.address, &t.token_id, &deps.as_ref(), &info))
        .collect::<ContractResult>()?;

    let input: Vec<_> = msg
        .traits
        .into_iter()
        .map(|t| Trait {
            token_id: msg.token_id.to_owned(),
            token: t,
        })
        .collect();

    let mut traits_to_add = validate_traits(&input, &deps.as_ref())?;

    TRAITS.update(deps.storage, |mut traits| -> ContractResult<_> {
        traits.append(&mut traits_to_add);
        Ok(traits)
    })?;
    Ok(Response::new()
        .add_attribute(ACTION, APPLY_ACTION)
        .add_attribute(EXTEND_EVENT, msg.token_id)
        .add_attributes(
            traits_to_add
                .into_iter()
                .map(|t| Attribute::new(APPLY_EVENT, t.token)),
        ))
}

/// Remove provided tokens as traits
pub fn exempt(msg: ExemptMsg, deps: DepsMut, info: MessageInfo) -> ContractResponse {
    require_instantiated(&deps.as_ref(), &info)?;

    // To apply traits, the sender must be:
    // - the trait tokens' owner / approved spender
    msg.traits
        .iter()
        .map(|t| require_sender_cw721_approval(&t.address, &t.token_id, &deps.as_ref(), &info))
        .collect::<ContractResult>()?;

    let traits_to_remove = msg
        .traits
        .iter()
        .map(|t| {
            let address = deps.api.addr_validate(&t.address)?;
            Ok(TokenConfig {
                address,
                token_id: t.token_id.to_owned(),
            })
        })
        .collect::<ContractResult<Vec<_>>>()?;

    TRAITS.update(deps.storage, |traits| -> ContractResult<_> {
        let traits = traits
            .into_iter()
            .filter(|current_t| {
                !traits_to_remove.iter().any(|t| {
                    t.address == current_t.token.address && t.token_id == current_t.token.token_id
                })
            })
            .collect();
        Ok(traits)
    })?;
    Ok(Response::new()
        .add_attribute(ACTION, EXEMPT_ACTION)
        .add_attributes(
            traits_to_remove
                .into_iter()
                .map(|t: TokenConfig| Attribute::new(EXEMPT_EVENT, t)),
        ))
}
