use cw_constructor::contract as ConstructorContract;
use cw_constructor::error::ContractResponse;
use cw_constructor::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_fiend_frens_shared::metadata::{Extension, MergedExtension, TraitExtension};

// Version info for migration
const CONTRACT_NAME: &str = "fiend-frens-constructor";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, StdResult};

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> ContractResponse {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        ConstructorContract::instantiate(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> ContractResponse {
        ConstructorContract::execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        ConstructorContract::query::<Extension, TraitExtension, MergedExtension>(deps, env, msg)
    }
}
