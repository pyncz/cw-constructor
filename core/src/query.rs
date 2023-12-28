use crate::{msg::ListAdminsResp, state::ADMINS};
use cosmwasm_std::{Deps, StdResult};

pub fn query_list_admins(deps: Deps) -> StdResult<ListAdminsResp> {
    let admins = ADMINS.load(deps.storage)?;
    let resp = ListAdminsResp { admins };
    Ok(resp)
}
