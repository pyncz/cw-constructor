use crate::{
    msg::{GetConfigMsg, GetConfigResp, GetTraitsMsg, GetTraitsResp},
    state::{CONFIG, TRAITS},
};
use cosmwasm_std::{Deps, StdResult};

pub fn query_config(_msg: GetConfigMsg, deps: Deps) -> StdResult<GetConfigResp> {
    let config = CONFIG.load(deps.storage)?;
    let resp = GetConfigResp {
        admins: config.admins,
        allowed_traits_addresses: config.allowed_traits_addresses,
        base_token: config.base_token,
    };
    Ok(resp)
}

pub fn query_traits(_msg: GetTraitsMsg, deps: Deps) -> StdResult<GetTraitsResp> {
    let traits = TRAITS.load(deps.storage)?;
    let resp = GetTraitsResp { traits };
    Ok(resp)
}
