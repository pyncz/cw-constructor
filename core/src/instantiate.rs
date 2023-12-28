use crate::events::{
    ACTION, ADMIN_SET_EVENT, ALLOWED_TRAIT_ADDRESS_SET_EVENT, BASE_TOKEN_SET_EVENT,
    INSTANTIATE_ACTION,
};
use crate::models::Config;
use crate::msg::InstantiateMsg;
use crate::state::CONFIG;
use crate::{error::ContractError, models::TokenConfig};
use cosmwasm_std::{Attribute, DepsMut, Response, StdResult};

pub fn init(msg: InstantiateMsg, deps: DepsMut) -> Result<Response, ContractError> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .unwrap_or(vec![])
        .into_iter()
        .map(|address| deps.api.addr_validate(&address))
        .collect();
    let admins = admins.unwrap();

    let allowed_traits_addresses: StdResult<Vec<_>> = msg
        .allowed_traits_addresses
        .unwrap_or(vec![])
        .into_iter()
        .map(|address| deps.api.addr_validate(&address))
        .collect();
    let allowed_traits_addresses = allowed_traits_addresses.unwrap();

    let base_token = TokenConfig {
        address: deps.api.addr_validate(&msg.base_token.address).unwrap(),
        token_id: msg.base_token.token_id,
    };

    CONFIG.save(
        deps.storage,
        &Config {
            base_token: base_token.clone(),
            allowed_traits_addresses: allowed_traits_addresses.clone(),
            allow_multiple_tokens_per_contract: msg
                .allow_multiple_tokens_per_contract
                .unwrap_or(false),
            admins: admins.clone(),
        },
    )?;

    Ok(Response::new()
        .add_attribute(ACTION, INSTANTIATE_ACTION)
        .add_attributes(
            admins
                .into_iter()
                .map(|addr| Attribute::new(ADMIN_SET_EVENT, addr)),
        )
        .add_attributes(
            allowed_traits_addresses
                .into_iter()
                .map(|addr| Attribute::new(ALLOWED_TRAIT_ADDRESS_SET_EVENT, addr)),
        )
        .add_attribute(BASE_TOKEN_SET_EVENT, base_token))
}
