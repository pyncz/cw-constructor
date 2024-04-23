#![cfg(test)]
use crate::models::config::SlotConfig;
use crate::models::metadata::TokenMetadata;
use crate::msg::{
    ContractInfoMsg, ContractInfoResp, InfoMsg, InfoResp, InstantiateMsg, QueryMsg, TraitsMsg,
    TraitsResp,
};
use crate::tests::utils::cw721::instantiate_cw721;
use crate::tests::utils::entry as constructor;
use crate::tests::utils::shared::BASE_TOKEN_ID;
use cosmwasm_std::Addr;
use cw721::{ContractInfoResponse, NftInfoResponse};
use cw721_base::{ExecuteMsg as Cw721BaseExecuteMsg, MintMsg};
use cw_multi_test::{App, ContractWrapper, Executor};

use super::utils::metadata::{Extension, MergedExtension, TraitExtension};

/// Test if instantiates correctly with no admins
#[test]
fn instantiate_without_admins() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query,
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
        constructor::execute,
        constructor::instantiate,
        constructor::query,
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

/// Test initial `info` query
#[test]
fn initial_info() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query,
    );
    let code_id = app.store_code(Box::new(code));

    // Instantiate cw721 contracts
    let minter = Addr::unchecked("player");
    let base_contract = instantiate_cw721::<Extension>(&mut app, &minter, "BASE");

    // Mint base token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: BASE_TOKEN_ID.to_string(),
        owner: minter.to_string(),
        token_uri: None,
        extension: Extension {
            name: "Collection".to_string(),
            value: 10,
        },
    });
    app.execute_contract(minter.clone(), base_contract.clone(), &mint_msg, &[])
        .unwrap();

    // Instantiate constructor contract
    let constructor_contract = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                base_token: base_contract.into(),
                slots: vec![SlotConfig {
                    name: "slot".to_string(),
                    allowed_contracts: vec![],
                    allow_multiple: false,
                }],
                admins: None,
            },
            &[],
            "Character",
            None,
        )
        .unwrap();

    // Validate response
    let resp: InfoResp<Extension, TraitExtension, MergedExtension> = app
        .wrap()
        .query_wasm_smart(
            constructor_contract.clone(),
            &QueryMsg::Info(InfoMsg {
                token_id: BASE_TOKEN_ID.to_string(),
            }),
        )
        .unwrap();

    assert_eq!(
        resp,
        InfoResp {
            info: NftInfoResponse {
                token_uri: None,
                extension: MergedExtension { value: 10 }
            },
            base_token: TokenMetadata {
                contract: ContractInfoResponse {
                    name: "Test NFT".to_string(),
                    symbol: "BASE".to_string(),
                },
                token: NftInfoResponse {
                    token_uri: None,
                    extension: Extension {
                        name: "Collection".to_string(),
                        value: 10
                    }
                }
            },
            traits: vec![]
        }
    );
}
