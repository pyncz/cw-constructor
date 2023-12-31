use crate::{
    msg::{GetConfigMsg, GetConfigResp, GetTraitsMsg, GetTraitsResp},
    state::{CONFIG, TRAITS},
};
use cosmwasm_std::{Deps, StdResult};

pub fn config(_msg: GetConfigMsg, deps: Deps) -> StdResult<GetConfigResp> {
    let config = CONFIG.load(deps.storage)?;

    Ok(GetConfigResp {
        base_token: config.base_token,
        slots: config.slots,
        admins: config.admins,
    })
}

pub fn traits(_msg: GetTraitsMsg, deps: Deps) -> StdResult<GetTraitsResp> {
    let traits = TRAITS.load(deps.storage)?;

    // TODO: Apply filters from the message
    Ok(GetTraitsResp { traits })
}
