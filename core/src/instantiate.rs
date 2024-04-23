use crate::contract::Contract;
use crate::error::ContractResponse;
use crate::events::{
    ACTION, ADMIN_ADDED_EVENT, BASE_TOKEN_SET_EVENT, INSTANTIATE_ACTION, SLOT_ADDED_EVENT,
};
use crate::models::config::ContractInfo;
use crate::msg::InstantiateMsg;
use crate::utils::validators::validate_config;
use cosmwasm_std::{Attribute, DepsMut, Response};

impl<'a, TExtension, TTraitExtension, TMergedExtension>
    Contract<'a, TExtension, TTraitExtension, TMergedExtension>
{
    pub fn _instantiate(&self, msg: InstantiateMsg, deps: DepsMut) -> ContractResponse {
        let input = ContractInfo {
            base_token: msg.base_token,
            slots: msg.slots,
            admins: msg.admins.unwrap_or(vec![]),
        };

        // Init state
        // - config
        let config = validate_config(&input, &deps.as_ref())?;
        self.config.save(deps.storage, &config)?;
        // - traits
        self.traits.save(deps.storage, &vec![])?;

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
}
