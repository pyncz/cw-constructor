use cosmwasm_std::Empty;
use cw721_base::ContractError;
use cw_fiend_frens_shared::metadata::TraitExtension;

// Version info for migration
const CONTRACT_NAME: &str = "fiend-frens-trait";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Cw721MetadataContract<'a> =
    cw721_base::Cw721Contract<'a, TraitExtension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<TraitExtension, Empty>;
pub type InstantiateMsg = cw721_base::InstantiateMsg;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        Cw721MetadataContract::default().instantiate(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721MetadataContract::default().query(deps, env, msg)
    }
}
