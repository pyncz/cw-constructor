#![cfg(test)]
use super::utils::{entry::InstantiateMsg, metadata::Extension, shared::ADMIN};
use crate::{models::config::ExtensionsConfig, tests::utils::entry as minter};
use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor};

/// Test if instantiates correctly with some admins
#[test]
fn instantiate_with_admins() {
    let mut app = App::default();

    let code = ContractWrapper::new(minter::execute, minter::instantiate, minter::query);
    let code_id = app.store_code(Box::new(code));

    app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            supply: None,
            price: None,
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
    .unwrap();
}

/// Test if instantiates correctly with no admins but cw721 set
#[test]
fn instantiate_with_cw721() {
    let mut app = App::default();

    let code = ContractWrapper::new(minter::execute, minter::instantiate, minter::query);
    let code_id = app.store_code(Box::new(code));

    app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            supply: None,
            price: None,
            extensions: vec![ExtensionsConfig {
                value: Extension {},
                weight: 1,
            }],
            admins: vec![],
            cw721: Some("cw721".to_string()),
        },
        &[],
        "Minter",
        None,
    )
    .unwrap();
}

/// Test error if neither of admins nor cw721 set
#[test]
fn instantiate_with_no_extensions() {
    let mut app = App::default();

    let code = ContractWrapper::new(minter::execute, minter::instantiate, minter::query);
    let code_id = app.store_code(Box::new(code));

    let resp = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            supply: None,
            price: None,
            extensions: vec![],
            admins: vec![ADMIN.to_string()],
            cw721: None,
        },
        &[],
        "Minter",
        None,
    );
    assert!(resp.is_err());
}

/// Test error if neither of admins nor cw721 are provided
#[test]
fn instantiate_with_no_cw721_nor_admins() {
    let mut app = App::default();

    let code = ContractWrapper::new(minter::execute, minter::instantiate, minter::query);
    let code_id = app.store_code(Box::new(code));

    let resp = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            supply: None,
            price: None,
            extensions: vec![ExtensionsConfig {
                value: Extension {},
                weight: 1,
            }],
            admins: vec![],
            cw721: None,
        },
        &[],
        "Minter",
        None,
    );
    assert!(resp.is_err());
}

/// Test error if invalid addresses are provided
#[test]
fn instantiate_with_invalid_addresses() {
    let mut app = App::default();

    let code = ContractWrapper::new(minter::execute, minter::instantiate, minter::query);
    let code_id = app.store_code(Box::new(code));

    let resp = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            supply: None,
            price: None,
            extensions: vec![ExtensionsConfig {
                value: Extension {},
                weight: 1,
            }],
            admins: vec!["".to_string()],
            cw721: Some("".to_string()),
        },
        &[],
        "Minter",
        None,
    );
    assert!(resp.is_err());
}
