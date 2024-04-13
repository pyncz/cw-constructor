#![cfg(test)]
use crate::contract as constructor;
use crate::models::config::SlotConfig;

use crate::models::metadata::TokenMetadata;
use crate::models::token::TokenConfig;
use crate::msg::{
    EquipMsg, ExecuteMsg, InfoMsg, InfoResp, InstantiateMsg, QueryMsg, TraitsMsg, TraitsResp,
    UnequipMsg,
};
use crate::tests::utils::shared::{BASE_TOKEN_ID, TRAIT_TOKEN_ID};
use cosmwasm_std::Addr;
use cw721::{ContractInfoResponse, Cw721ExecuteMsg, NftInfoResponse};
use cw721_base::{ExecuteMsg as Cw721BaseExecuteMsg, MintMsg};
use cw_multi_test::{App, ContractWrapper, Executor};

use super::utils::{
    cw721::instantiate_cw721,
    metadata::{Extension, MergedExtension, TraitExtension},
};

/// Test if a trait token unequips correctly
#[test]
fn unequip_allowed_trait() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query::<Extension, TraitExtension, MergedExtension>,
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

    // Unequip trait
    app.execute_contract(
        minter.clone(),
        constructor_contract.clone(),
        &ExecuteMsg::Unequip(UnequipMsg {
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    )
    .unwrap();

    // Validate removed traits
    let resp: TraitsResp = app
        .wrap()
        .query_wasm_smart(
            constructor_contract.clone(),
            &QueryMsg::Traits(TraitsMsg {
                token_id: Some(BASE_TOKEN_ID.to_string()),
                slot: None,
            }),
        )
        .unwrap();

    assert_eq!(resp, TraitsResp { traits: vec![] });
}

/// Test unequip already not equipped trait - should fail
#[test]
fn unequip_not_equipped_trait() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query::<Extension, TraitExtension, MergedExtension>,
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
                admins: None,
            },
            &[],
            "Character",
            None,
        )
        .unwrap();

    // Unequip trait
    let resp = app.execute_contract(
        minter.clone(),
        constructor_contract.clone(),
        &ExecuteMsg::Unequip(UnequipMsg {
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    );

    assert!(resp.is_err());
}

/// Test unequip not owned nor approved trait from owned base token - should fail
#[test]
fn unequip_not_owned_trait() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query::<Extension, TraitExtension, MergedExtension>,
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

    // Unequip trait
    let sender = Addr::unchecked("minter_wannabe");
    let resp = app.execute_contract(
        sender,
        constructor_contract.clone(),
        &ExecuteMsg::Unequip(UnequipMsg {
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    );

    assert!(resp.is_err());
}

/// Test unequip not owned but approved trait from owned base token
#[test]
fn unequip_approved_trait() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query::<Extension, TraitExtension, MergedExtension>,
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

    // Approve first and then try to unequip
    // - approve
    let spender = Addr::unchecked("spender");
    let approve_msg = Cw721ExecuteMsg::Approve {
        spender: spender.clone().into(),
        token_id: TRAIT_TOKEN_ID.to_string(),
        expires: None,
    };
    app.execute_contract(minter.clone(), trait_contract.clone(), &approve_msg, &[])
        .unwrap();

    // - unequip
    app.execute_contract(
        spender,
        constructor_contract.clone(),
        &ExecuteMsg::Unequip(UnequipMsg {
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    )
    .unwrap();

    // Validate the changes
    let resp: TraitsResp = app
        .wrap()
        .query_wasm_smart(
            constructor_contract.clone(),
            &QueryMsg::Traits(TraitsMsg {
                token_id: Some(BASE_TOKEN_ID.to_string()),
                slot: None,
            }),
        )
        .unwrap();

    assert_eq!(resp, TraitsResp { traits: vec![] });
}

/// Test `info` after a trait is unequipped
#[test]
fn unequipped_info() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query::<Extension, TraitExtension, MergedExtension>,
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

    // Unequip trait
    app.execute_contract(
        minter.clone(),
        constructor_contract.clone(),
        &ExecuteMsg::Unequip(UnequipMsg {
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
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
