use cw_constructor::contract::Contract as ConstructorContract;
use cw_constructor::error::ContractResponse;
use cw_constructor::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_fiend_frens_shared::metadata::{Extension, MergedExtension, TraitExtension};
use semver::Version;

// Version info for migration
const CONTRACT_NAME: &str = "fiend-frens-constructor";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

type Contract<'a> = ConstructorContract<'a, Extension, TraitExtension, MergedExtension>;

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
    use cw_constructor::error::ContractError;

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

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> ContractResponse {
        // Current contract state
        let current = cw2::get_contract_version(deps.storage)?;
        let original_name = current.contract;
        let current_version = current.version;
        let current_version_semver: Version = current_version.parse()?;

        // New state to migrate to
        let new_name = CONTRACT_NAME.to_string();
        let new_version: String = CONTRACT_VERSION.to_string();
        let new_version_semver: Version = new_version.parse()?;

        // Validate migration params
        if original_name != new_name {
            return Err(ContractError::InvalidMigrationContractName {
                original_name,
                new_name,
            });
        }
        if current_version_semver >= new_version_semver {
            return Err(ContractError::InvalidMigrationVersion {
                current_version,
                new_version,
            });
        }

        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        Ok(Response::default())
    }
}
