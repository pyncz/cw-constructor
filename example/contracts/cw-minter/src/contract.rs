use crate::error::ContractResponse;
use crate::models::config::ContractInfo;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult};
use cw_storage_plus::Item;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Contract<'a, TExtension>
where
    TExtension: Serialize + DeserializeOwned + Clone,
{
    pub config: Item<'a, ContractInfo<TExtension>>,
    pub mint_count: Item<'a, u32>,
}

impl<'a, TExtension> Contract<'a, TExtension>
where
    TExtension: Serialize + DeserializeOwned + Clone,
{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg<TExtension>,
    ) -> ContractResponse {
        self._instantiate(deps, env, info, msg)
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::ContractInfo(msg) => to_json_binary(&self.contract_info(&msg, &deps)?),
        }
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> ContractResponse {
        match msg {
            ExecuteMsg::Mint(msg) => self.mint(deps, env, info, msg),

            // Admin actions
            ExecuteMsg::SetCw721(msg) => self.set_cw721(deps, env, info, msg),
        }
    }
}

impl<'a, TExtension> Default for Contract<'a, TExtension>
where
    TExtension: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self {
            config: Item::new("config"),
            mint_count: Item::new("counter"),
        }
    }
}
