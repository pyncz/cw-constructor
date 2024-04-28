use crate::events::{
    ACTION, CW721_SET_EVENT, INSTANTIATE_ACTION, MINT_PRICE_AMOUNT_SET_EVENT,
    MINT_PRICE_DENOM_EVENT, SUPPLY_SET_EVENT,
};
use crate::msg::InstantiateMsg;
use crate::utils::validators::parse_config;
use crate::{contract::Contract, error::ContractResponse};
use cosmwasm_std::{Attribute, DepsMut, Env, MessageInfo, Response};
use serde::{de::DeserializeOwned, Serialize};

impl<'a, TExtension> Contract<'a, TExtension>
where
    TExtension: Serialize + DeserializeOwned + Clone,
{
    pub fn _instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg<TExtension>,
    ) -> ContractResponse {
        let config = parse_config(&msg, &deps.as_ref())?;

        // Init state
        self.config.save(deps.storage, &config)?;
        self.mint_count.save(deps.storage, &0)?;

        let mut attrs: Vec<Attribute> = vec![];
        if let Some(supply) = msg.supply {
            attrs.push(Attribute::new(SUPPLY_SET_EVENT, supply.to_string()));
        }
        if let Some(price) = msg.price {
            attrs.push(Attribute::new(MINT_PRICE_DENOM_EVENT, &price.denom));
            attrs.push(Attribute::new(
                MINT_PRICE_AMOUNT_SET_EVENT,
                &price.amount.to_string(),
            ));
        }
        Ok(Response::new()
            .add_attribute(ACTION, INSTANTIATE_ACTION)
            .add_attribute(CW721_SET_EVENT, &msg.cw721)
            .add_attributes(attrs))
    }
}
