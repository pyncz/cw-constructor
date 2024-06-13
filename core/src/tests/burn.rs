#![cfg(test)]
use crate::models::config::SlotConfig;
use crate::models::metadata::TokenMetadata;
use crate::models::token::TokenConfig;
use crate::msg::{EquipMsg, ExecuteMsg, InfoMsg, InfoResp, InstantiateMsg, QueryMsg};
use crate::tests::utils::entry as constructor;
use crate::tests::utils::metadata::MergedExtension;
use crate::tests::utils::shared::{BASE_TOKEN_ID, TRAIT_TOKEN_ID};
use cosmwasm_std::{Addr, StdResult};
use cw721::{ContractInfoResponse, Cw721ExecuteMsg, NftInfoResponse};
use cw721_base::{ExecuteMsg as Cw721BaseExecuteMsg, MintMsg};
use cw_multi_test::{App, ContractWrapper, Executor};

use super::utils::{
    cw721::instantiate_cw721,
    metadata::{Extension, TraitExtension},
};

/// Test burning base token - should fail on info query
#[test]
fn burn_base_token() {
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
    let trait_contract = instantiate_cw721::<TraitExtension>(&mut app, &minter, "TRAIT");

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

    // Mint trait token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: TRAIT_TOKEN_ID.to_string(),
        owner: minter.to_string(),
        token_uri: None,
        extension: TraitExtension { value: 2 },
    });
    app.execute_contract(minter.clone(), trait_contract.clone(), &mint_msg, &[])
        .unwrap();

    // Instantiate constructor contract
    let constructor_contract = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                base_token: base_contract.clone().into(),
                slots: vec![SlotConfig {
                    name: "slot".to_string(),
                    allowed_contracts: vec![trait_contract.to_string()],
                    allow_multiple: false,
                }],
                admins: vec![],
            },
            &[],
            "Character",
            None,
        )
        .unwrap();

    // Equip trait
    app.execute_contract(
        minter.clone(),
        constructor_contract.clone(),
        &ExecuteMsg::Equip(EquipMsg {
            token_id: BASE_TOKEN_ID.to_owned(),
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &[],
    )
    .unwrap();

    // Burn base token
    app.execute_contract(
        minter,
        base_contract,
        &Cw721ExecuteMsg::Burn {
            token_id: BASE_TOKEN_ID.to_owned(),
        },
        &[],
    )
    .unwrap();

    // Validate `info` response fails
    let resp: StdResult<InfoResp<Extension, TraitExtension, MergedExtension>> =
        app.wrap().query_wasm_smart(
            constructor_contract,
            &QueryMsg::Info(InfoMsg {
                token_id: BASE_TOKEN_ID.to_string(),
            }),
        );

    assert!(resp.is_err());
}

/// Test burning trait token - should still response ignoring the burned token's attributes
#[test]
fn burn_trait_token() {
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
    let trait_contract = instantiate_cw721::<TraitExtension>(&mut app, &minter, "TRAIT");

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

    // Mint trait token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: TRAIT_TOKEN_ID.to_string(),
        owner: minter.to_string(),
        token_uri: None,
        extension: TraitExtension { value: 2 },
    });
    app.execute_contract(minter.clone(), trait_contract.clone(), &mint_msg, &[])
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
                    allowed_contracts: vec![trait_contract.to_string()],
                    allow_multiple: false,
                }],
                admins: vec![],
            },
            &[],
            "Character",
            None,
        )
        .unwrap();

    // Equip trait
    app.execute_contract(
        minter.clone(),
        constructor_contract.clone(),
        &ExecuteMsg::Equip(EquipMsg {
            token_id: BASE_TOKEN_ID.to_owned(),
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &[],
    )
    .unwrap();

    // Burn trait token
    app.execute_contract(
        minter,
        trait_contract,
        &Cw721ExecuteMsg::Burn {
            token_id: TRAIT_TOKEN_ID.to_owned(),
        },
        &[],
    )
    .unwrap();

    // Validate `info` response ignores burned trait's info
    let resp: StdResult<InfoResp<Extension, TraitExtension, MergedExtension>> =
        app.wrap().query_wasm_smart(
            constructor_contract,
            &QueryMsg::Info(InfoMsg {
                token_id: BASE_TOKEN_ID.to_string(),
            }),
        );

    assert_eq!(
        resp,
        Ok(InfoResp {
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
        })
    );
}
