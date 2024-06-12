use crate::{
    contract::Contract,
    error::{ContractError, ContractResponse, ContractResult},
    events::{ACTION, EQUIP_ACTION, EQUIP_EVENT, EQUIP_ON_EVENT, UNEQUIP_ACTION, UNEQUIP_EVENT},
    models::{token::TokenConfig, traits::Trait},
    msg::{EquipMsg, UnequipMsg},
};
use cosmwasm_std::{Attribute, DepsMut, Env, MessageInfo, Response};

impl<'a, TExtension, TTraitExtension, TMergedExtension>
    Contract<'a, TExtension, TTraitExtension, TMergedExtension>
{
    /// Add provided tokens as traits
    pub fn equip(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: EquipMsg,
    ) -> ContractResponse {
        let config = self.require_instantiated(&deps.as_ref(), &info)?;

        // To equip traits, the sender must be:
        // - the base token's owner / approved spender
        self.require_sender_cw721_approval(
            config.base_token,
            &msg.token_id,
            &deps.as_ref(),
            &info,
        )?;

        // - the trait tokens' owner / approved spender
        msg.traits.iter().try_for_each(|t| {
            self.require_sender_cw721_approval(&t.address, &t.token_id, &deps.as_ref(), &info)
        })?;

        let input: Vec<_> = msg
            .traits
            .into_iter()
            .map(|t| Trait {
                token_id: msg.token_id.to_owned(),
                token: t,
            })
            .collect();

        let mut traits_to_add = self.validate_traits(&input, &deps.as_ref())?;

        self.traits
            .update(deps.storage, |mut traits| -> ContractResult<_> {
                traits.append(&mut traits_to_add);
                Ok(traits)
            })?;
        Ok(Response::new()
            .add_attribute(ACTION, EQUIP_ACTION)
            .add_attribute(EQUIP_ON_EVENT, msg.token_id)
            .add_attributes(
                traits_to_add
                    .into_iter()
                    .map(|t| Attribute::new(EQUIP_EVENT, t.token)),
            ))
    }

    /// Remove provided tokens as traits
    pub fn unequip(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: UnequipMsg,
    ) -> ContractResponse {
        let config = self.require_instantiated(&deps.as_ref(), &info)?;

        let equipped_traits = self.traits.load(deps.storage)?;

        let traits_to_remove = msg
            .traits
            .iter()
            .map(|t| {
                // To unequip traits:
                // - trait must be currently equipped
                // TODO: Or is it better to just skip it silently..?
                let current_trait = equipped_traits
                    .iter()
                    .find(|current_t| current_t.token == *t)
                    .ok_or(ContractError::NotEquipped {})?;

                // - the sender must be the trait or base token's owner / approved spender
                self.require_sender_cw721_approval(&t.address, &t.token_id, &deps.as_ref(), &info)
                    .or_else(|_| {
                        self.require_sender_cw721_approval(
                            config.base_token.clone(),
                            &current_trait.token_id,
                            &deps.as_ref(),
                            &info,
                        )
                    })?;

                let address = deps.api.addr_validate(&t.address)?;
                Ok(TokenConfig {
                    address,
                    token_id: t.token_id.to_owned(),
                })
            })
            .collect::<ContractResult<Vec<_>>>()?;

        self.traits
            .update(deps.storage, |traits| -> ContractResult<_> {
                let traits = traits
                    .into_iter()
                    .filter(|current_t| !traits_to_remove.contains(&current_t.token))
                    .collect();
                Ok(traits)
            })?;
        Ok(Response::new()
            .add_attribute(ACTION, UNEQUIP_ACTION)
            .add_attributes(
                traits_to_remove
                    .into_iter()
                    .map(|t: TokenConfig| Attribute::new(UNEQUIP_EVENT, t)),
            ))
    }
}
