use super::metadata::Extension;
use crate::contract::Contract as MinterContract;
use crate::error::ContractResponse;
use crate::msg::{ExecuteMsg, InstantiateMsg as MinterInstantiateMsg, QueryMsg};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, StdResult};

pub type Contract<'a> = MinterContract<'a, Extension>;
pub type InstantiateMsg = MinterInstantiateMsg<Extension>;

#[allow(dead_code)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResponse {
    Contract::default().instantiate(deps, env, info, msg)
}

#[allow(dead_code)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ContractResponse {
    Contract::default().execute(deps, env, info, msg)
}

#[allow(dead_code)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Contract::default().query(deps, env, msg)
}
