use cw_fiend_frens_shared::metadata::TraitExtension as Extension;
use cw_minter::contract::Contract as BaseContract;
use cw_minter::error::ContractResponse;
use cw_minter::msg::{ExecuteMsg, InstantiateMsg as BaseInstantiateMsg, QueryMsg};

// Version info for migration
const CONTRACT_NAME: &str = "fiend-frens-trait-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

type Contract<'a> = BaseContract<'a, Extension>;
pub type InstantiateMsg = BaseInstantiateMsg<Extension>;

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

        let contract = Contract::default();
        contract.instantiate(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> ContractResponse {
        let contract = Contract::default();
        contract.execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        let contract = Contract::default();
        contract.query(deps, env, msg)
    }
}
