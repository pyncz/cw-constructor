use crate::error::ContractResponse;
use crate::models::metadata::MergeWithTraitExtension;
use crate::models::{config::ContractInfo, traits::Trait};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cosmwasm_std::to_json_binary;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, StdResult};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct Contract<'a, TExtension, TTraitExtension, TMergedExtension> {
    pub config: Item<'a, ContractInfo>,
    pub traits: Item<'a, Vec<Trait>>,
    _extension: PhantomData<TExtension>,
    _trait_extension: PhantomData<TTraitExtension>,
    _merged_extension: PhantomData<TMergedExtension>,
}

impl<'a, TExtension, TTraitExtension, TMergedExtension>
    Contract<'a, TExtension, TTraitExtension, TMergedExtension>
where
    TExtension: Serialize + for<'de> Deserialize<'de> + Clone,
    TTraitExtension: Serialize + for<'de> Deserialize<'de>,
    TMergedExtension: Serialize
        + for<'de> Deserialize<'de>
        + MergeWithTraitExtension<TTraitExtension>
        + From<TExtension>,
{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> ContractResponse {
        self._instantiate(msg, deps)
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::ContractInfo(msg) => to_json_binary(&self.contract_info(&msg, &deps)?),
            QueryMsg::Traits(msg) => to_json_binary(&self.traits(&msg, &deps)?),
            QueryMsg::Tokens(msg) => to_json_binary(&self.tokens(&msg, &deps)?),
            QueryMsg::Info(msg) => to_json_binary(&self.info(&msg, &deps)?),
        }
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> ContractResponse {
        match msg {
            ExecuteMsg::Equip(msg) => self.equip(msg, deps, info),
            ExecuteMsg::Unequip(msg) => self.unequip(msg, deps, info),
        }
    }
}

impl<'a, TExtension, TTraitExtension, TMergedExtension> Default
    for Contract<'a, TExtension, TTraitExtension, TMergedExtension>
{
    fn default() -> Self {
        Self {
            config: Item::new("config"),
            traits: Item::new("traits"),
            _extension: PhantomData,
            _trait_extension: PhantomData,
            _merged_extension: PhantomData,
        }
    }
}
