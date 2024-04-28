#![cfg(test)]
use std::fmt::Debug;

use super::cw721_entry;
use cosmwasm_std::{Addr, Empty};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use serde::{de::DeserializeOwned, Serialize};

pub fn create_cw721<T: Serialize + DeserializeOwned + Clone + Debug + 'static>(
) -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw721_entry::execute::<T>,
        cw721_entry::instantiate::<T>,
        cw721_entry::query::<T>,
    );
    Box::new(contract)
}

pub fn instantiate_cw721<T: Serialize + DeserializeOwned + Clone + Debug + 'static>(
    router: &mut App,
    minter: &Addr,
    symbol: &str,
) -> Addr {
    let cw721_id = router.store_code(create_cw721::<T>());
    let msg = Cw721InstantiateMsg {
        name: "Test NFT".to_string(),
        symbol: symbol.to_string(),
        minter: minter.into(),
    };
    let address = router
        .instantiate_contract(cw721_id, minter.clone(), &msg, &[], "cw721", None)
        .unwrap();

    address
}
