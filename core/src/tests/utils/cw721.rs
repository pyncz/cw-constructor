#![cfg(test)]
use cosmwasm_std::{Addr, Empty};
use cw721_base::msg::InstantiateMsg as Cw721InstantiateMsg;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

pub fn create_cw721() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw721_base::entry::execute,
        cw721_base::entry::instantiate,
        cw721_base::entry::query,
    );
    Box::new(contract)
}

pub fn instantiate_cw721(router: &mut App, minter: &Addr, symbol: &str) -> Addr {
    let cw721_id = router.store_code(create_cw721());
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
