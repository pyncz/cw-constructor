use cosmwasm_std::Empty;
use cw721::ContractInfoResponse;
pub use cw721_base::{
    ContractError, Cw721Contract, ExecuteMsg, InstantiateMsg, MintMsg, MinterResponse, QueryMsg,
};

type Contract<'a, T> = Cw721Contract<'a, T, Empty>;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate<T: Serialize + for<'de> Deserialize<'de> + Clone>(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let contract = Contract::<T>::default();

    let info = ContractInfoResponse {
        name: msg.name,
        symbol: msg.symbol,
    };
    contract.contract_info.save(deps.storage, &info)?;

    let minter = deps.api.addr_validate(&msg.minter)?;
    contract.minter.save(deps.storage, &minter)?;

    Ok(Response::default())
}

#[allow(dead_code)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute<T: Serialize + for<'de> Deserialize<'de> + Clone>(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg<T>,
) -> Result<Response, ContractError> {
    Contract::<T>::default().execute(deps, env, info, msg)
}

#[allow(dead_code)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query<T: Serialize + for<'de> Deserialize<'de> + Clone>(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    Contract::<T>::default().query(deps, env, msg)
}
