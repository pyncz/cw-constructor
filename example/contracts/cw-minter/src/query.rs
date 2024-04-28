use crate::{
    contract::Contract,
    msg::{ContractInfoMsg, ContractInfoResp},
};
use cosmwasm_std::{Deps, StdResult};
use serde::{de::DeserializeOwned, Serialize};

impl<'a, TExtension> Contract<'a, TExtension>
where
    TExtension: Serialize + DeserializeOwned + Clone,
{
    pub fn contract_info(
        &self,
        _msg: &ContractInfoMsg,
        deps: &Deps,
    ) -> StdResult<ContractInfoResp<TExtension>> {
        self.config.load(deps.storage)
    }
}
