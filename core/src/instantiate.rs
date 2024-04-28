use crate::contract::Contract;
use crate::error::ContractResponse;
use crate::events::{
    ACTION, ADMIN_ADDED_EVENT, BASE_TOKEN_SET_EVENT, INSTANTIATE_ACTION, SLOT_ADDED_EVENT,
};
use crate::msg::InstantiateMsg;
use crate::utils::validators::parse_config;
use cosmwasm_std::{Attribute, DepsMut, Env, MessageInfo, Response};

impl<'a, TExtension, TTraitExtension, TMergedExtension>
    Contract<'a, TExtension, TTraitExtension, TMergedExtension>
{
    pub fn _instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> ContractResponse {
        let config = parse_config(&msg, &deps.as_ref())?;

        // Init state
        self.config.save(deps.storage, &config)?;
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
