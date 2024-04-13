use cosmwasm_std::Empty;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
pub use cw721_base::{
    ContractError, Cw721Contract, ExecuteMsg, InstantiateMsg, MintMsg, MinterResponse, QueryMsg,
};
use serde::{Deserialize, Serialize};

type Contract<'a, T> = Cw721Contract<'a, T, Empty>;

#[allow(dead_code)]
pub fn instantiate<T: Serialize + for<'de> Deserialize<'de> + Clone>(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    Contract::<T>::default().instantiate(deps, env, info, msg)
}

#[allow(dead_code)]
pub fn execute<T: Serialize + for<'de> Deserialize<'de> + Clone>(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg<T>,
) -> Result<Response, ContractError> {
    Contract::<T>::default().execute(deps, env, info, msg)
}

#[allow(dead_code)]
pub fn query<T: Serialize + for<'de> Deserialize<'de> + Clone>(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    Contract::<T>::default().query(deps, env, msg)
}
