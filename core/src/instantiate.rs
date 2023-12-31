use crate::error::ContractResponse;
use crate::events::{
    ACTION, ADMIN_ADDED_EVENT, BASE_TOKEN_SET_EVENT, INSTANTIATE_ACTION, SLOT_ADDED_EVENT,
};
use crate::models::config::ContractInfo;
use crate::msg::InstantiateMsg;
use crate::state::CONFIG;
use crate::utils::validation::validate_config;
use cosmwasm_std::{Attribute, DepsMut, Response};

pub fn init(msg: InstantiateMsg, deps: DepsMut) -> ContractResponse {
    let input = ContractInfo {
        base_token: msg.base_token,
        slots: msg.slots,
        admins: msg.admins.unwrap_or(vec![]),
    };
    let config = validate_config(&input, &deps.as_ref())?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute(ACTION, INSTANTIATE_ACTION)
        .add_attribute(BASE_TOKEN_SET_EVENT, config.base_token)
        .add_attributes(
            config
                .admins
                .iter()
                .map(|addr| Attribute::new(ADMIN_ADDED_EVENT, addr)),
        )
        .add_attributes(
            config
                .slots
                .iter()
                .map(|slot| Attribute::new(SLOT_ADDED_EVENT, &slot.name)),
        ))
}
