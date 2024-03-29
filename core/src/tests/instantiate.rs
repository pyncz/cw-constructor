#![cfg(test)]
use super::utils::metadata::{Extension, MergedExtension, TraitExtension};
use crate::contract::{execute, instantiate, query};
use crate::msg::{
    ContractInfoMsg, ContractInfoResp, InstantiateMsg, QueryMsg, TraitsMsg, TraitsResp,
};
use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor};

/// Test if instantiates correctly with no admins
#[test]
fn instantiate_without_admins() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        execute,
        instantiate,
        query::<Extension, TraitExtension, MergedExtension>,
    );
    let code_id = app.store_code(Box::new(code));
    let base_token_unchecked = "base_token".to_string();
    let base_token = Addr::unchecked("base_token");

    let contract_address = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                base_token: base_token_unchecked.to_owned(),
                slots: vec![],
                admins: None,
            },
            &[],
            "Contract 1",
            None,
        )
        .unwrap();

    // Sanity query contract info
    let info: ContractInfoResp = app
        .wrap()
        .query_wasm_smart(
            contract_address.clone(),
            &QueryMsg::ContractInfo(ContractInfoMsg {}),
        )
        .unwrap();

    assert_eq!(
        info,
        ContractInfoResp {
            base_token: base_token.to_owned(),
            slots: vec![],
            admins: vec![]
        }
    );

    // Sanity query trait tokens
    let traits: TraitsResp = app
        .wrap()
        .query_wasm_smart(
            contract_address.clone(),
            &QueryMsg::Traits(TraitsMsg {
                slot: None,
                token_id: None,
            }),
        )
        .unwrap();

    assert_eq!(traits, TraitsResp { traits: vec![] });
}

/// Test if instantiates correctly with some admins
#[test]
fn instantiate_with_admins() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        execute,
        instantiate,
        query::<Extension, TraitExtension, MergedExtension>,
    );
    let code_id = app.store_code(Box::new(code));
    let base_token_unchecked = "base_token".to_string();
    let base_token = Addr::unchecked("base_token");

    let contract_address = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                base_token: base_token_unchecked,
                slots: vec![],
                admins: Some(vec!["admin1".to_owned(), "admin2".to_owned()]),
            },
            &[],
            "Contract 2",
            None,
        )
        .unwrap();

    let resp: ContractInfoResp = app
        .wrap()
        .query_wasm_smart(
            contract_address,
            &QueryMsg::ContractInfo(ContractInfoMsg {}),
        )
        .unwrap();

    assert_eq!(
        resp,
        ContractInfoResp {
            base_token,
            slots: vec![],
            admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
        }
    );
}
