use crate::error::ContractResponse;
use crate::execute;
use crate::instantiate;
use crate::models::metadata::MergeWithTraitExtension;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query;
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult,
};
use serde::{Deserialize, Serialize};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResponse {
    instantiate::init(msg, deps)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query<
    TExtension: Serialize + for<'de> Deserialize<'de> + Clone,
    TTraitExtension: Serialize + for<'de> Deserialize<'de>,
    TMergedExtension: Serialize
        + for<'de> Deserialize<'de>
        + MergeWithTraitExtension<TTraitExtension>
        + From<TExtension>,
>(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractInfo(msg) => to_json_binary(&query::contract_info(&msg, &deps)?),

        QueryMsg::Traits(msg) => to_json_binary(&query::traits(&msg, &deps)?),

        QueryMsg::Tokens(msg) => to_json_binary(&query::tokens(&msg, &deps)?),

        QueryMsg::Info(msg) => {
            to_json_binary(
                &query::info::<TExtension, TTraitExtension, TMergedExtension>(&msg, &deps)?,
            )
        }
    }
}

#[allow(dead_code)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> ContractResponse {
    match msg {
        ExecuteMsg::Equip(msg) => execute::equip(msg, deps, info),
        ExecuteMsg::Unequip(msg) => execute::unequip(msg, deps, info),
    }
}
