#![cfg(test)]
use super::utils::metadata::{Extension, MergedExtension, TraitExtension};
use crate::contract::{execute, instantiate, query};
use crate::models::config::SlotConfig;
use crate::models::metadata::TokenMetadata;
use crate::models::token::TokenConfig;
use crate::models::traits::TraitWithMetadataResp;
use crate::msg::{
    AllNftInfoMsg, AllNftInfoResp, EquipMsg, ExecuteMsg, InstantiateMsg, NftInfoMsg, NftInfoResp,
    QueryMsg,
};
use crate::tests::utils::cw721::instantiate_cw721;
use crate::tests::utils::shared::{BASE_TOKEN_ID, TRAIT_TOKEN_ID};
use cosmwasm_std::Addr;
use cw721::{ContractInfoResponse, NftInfoResponse};
use cw721_base::{ExecuteMsg as Cw721BaseExecuteMsg, MintMsg};
use cw_multi_test::{App, ContractWrapper, Executor};

/// Test initial `nft_info` query
#[test]
fn initial_nft_info() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        execute,
        instantiate,
        query::<Extension, TraitExtension, MergedExtension>,
    );
    let code_id = app.store_code(Box::new(code));

    let minter = Addr::unchecked("player");

    // Instantiate cw721 contracts
    let base_contract = instantiate_cw721(&mut app, &minter, "BASE");

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
    let resp: NftInfoResp<MergedExtension> = app
        .wrap()
        .query_wasm_smart(
            constructor_contract.clone(),
            &QueryMsg::NftInfo(NftInfoMsg {
                token_id: BASE_TOKEN_ID.to_string(),
            }),
        )
        .unwrap();

    assert_eq!(
        resp,
        NftInfoResp {
            info: TokenMetadata {
                contract: ContractInfoResponse {
                    name: "Test NFT".to_string(),
                    symbol: "BASE".to_string(),
                },
                token: NftInfoResponse {
                    token_uri: None,
                    extension: MergedExtension { value: 10 }
                }
            }
        }
    );
}

/// Test initial `all_nft_info` query
#[test]
fn initial_all_nft_info() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        execute,
        instantiate,
        query::<Extension, TraitExtension, MergedExtension>,
    );
    let code_id = app.store_code(Box::new(code));

    let minter = Addr::unchecked("player");

    // Instantiate cw721 contracts
    let base_contract = instantiate_cw721(&mut app, &minter, "BASE");

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
    let resp: AllNftInfoResp<Extension, TraitExtension> = app
        .wrap()
        .query_wasm_smart(
            constructor_contract.clone(),
            &QueryMsg::AllNftInfo(AllNftInfoMsg {
                token_id: BASE_TOKEN_ID.to_string(),
            }),
        )
        .unwrap();

    assert_eq!(
        resp,
        AllNftInfoResp {
            info: TokenMetadata {
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

/// Test `nft_info` after a trait equipped
#[test]
fn equipped_nft_info() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        execute,
        instantiate,
        query::<Extension, TraitExtension, MergedExtension>,
    );
    let code_id = app.store_code(Box::new(code));

    let minter = Addr::unchecked("player");

    // Instantiate cw721 contracts
    let base_contract = instantiate_cw721(&mut app, &minter, "BASE");
    let trait_contract = instantiate_cw721(&mut app, &minter, "TRAIT");

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
                admins: None,
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
        &vec![],
    )
    .unwrap();

    // Validate nft_info response
    let resp: NftInfoResp<MergedExtension> = app
        .wrap()
        .query_wasm_smart(
            constructor_contract.clone(),
            &QueryMsg::NftInfo(NftInfoMsg {
                token_id: BASE_TOKEN_ID.to_string(),
            }),
        )
        .unwrap();

    assert_eq!(
        resp,
        NftInfoResp {
            info: TokenMetadata {
                contract: ContractInfoResponse {
                    name: "Test NFT".to_string(),
                    symbol: "BASE".to_string(),
                },
                token: NftInfoResponse {
                    token_uri: None,
                    extension: MergedExtension { value: 12 }
                }
            }
        }
    );
}

/// Test `all_nft_info` after a trait equipped
#[test]
fn equipped_all_nft_info() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        execute,
        instantiate,
        query::<Extension, TraitExtension, MergedExtension>,
    );
    let code_id = app.store_code(Box::new(code));

    let minter = Addr::unchecked("player");

    // Instantiate cw721 contracts
    let base_contract = instantiate_cw721(&mut app, &minter, "BASE");
    let trait_contract = instantiate_cw721(&mut app, &minter, "TRAIT");

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
                admins: None,
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
        &vec![],
    )
    .unwrap();

    // Validate response
    let resp: AllNftInfoResp<Extension, TraitExtension> = app
        .wrap()
        .query_wasm_smart(
            constructor_contract.clone(),
            &QueryMsg::AllNftInfo(AllNftInfoMsg {
                token_id: BASE_TOKEN_ID.to_string(),
            }),
        )
        .unwrap();

    assert_eq!(
        resp,
        AllNftInfoResp {
            info: TokenMetadata {
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
            traits: vec![TraitWithMetadataResp {
                token_id: BASE_TOKEN_ID.to_string(),
                token: TokenConfig {
                    address: trait_contract.clone(),
                    token_id: TRAIT_TOKEN_ID.to_string()
                },
                slot: Some("slot".to_string()),
                info: TokenMetadata {
                    contract: ContractInfoResponse {
                        name: "Test NFT".to_string(),
                        symbol: "TRAIT".to_string()
                    },
                    token: NftInfoResponse {
                        token_uri: None,
                        extension: TraitExtension { value: 2 }
                    }
                }
            }]
        }
    );
}
