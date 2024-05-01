#![cfg(test)]
use super::{cw721::instantiate_cw721, entry::InstantiateMsg, metadata::Extension, shared::ADMIN};
use crate::{
    models::config::ExtensionsConfig,
    msg::{ExecuteMsg, SetCw721Msg},
    tests::utils::entry as minter,
};
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{App, ContractWrapper, Executor};

/// Instantiate minter contract
pub fn init_minter(app: &mut App, price: Option<Coin>, supply: Option<u32>) -> Addr {
    let code = ContractWrapper::new(minter::execute, minter::instantiate, minter::query);
    let code_id = app.store_code(Box::new(code));

    app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            supply,
            price,
            extensions: vec![ExtensionsConfig {
                value: Extension {},
                weight: 1,
            }],
            admins: vec![ADMIN.to_string()],
            cw721: None,
        },
        &[],
        "Minter",
        None,
    )
    .unwrap()
}

/// Instantiate minter and related cw721 contracts
pub fn init_cw721_with_minter(
    app: &mut App,
    price: Option<Coin>,
    supply: Option<u32>,
) -> (Addr, Addr) {
    // Instantiate minter contract
    let minter_contract = init_minter(app, price, supply);

    // Instantiate cw721 contracts
    let cw721_contract = instantiate_cw721::<Extension>(app, &minter_contract, "CW721");

    // Set cw721 for minter contract
    let admin = Addr::unchecked(ADMIN);
    let update_msg = ExecuteMsg::SetCw721(SetCw721Msg {
        address: cw721_contract.clone().into(),
    });
    app.execute_contract(admin, minter_contract.clone(), &update_msg, &[])
        .unwrap();

    (minter_contract, cw721_contract)
}
