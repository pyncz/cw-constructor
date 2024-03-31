#![cfg(test)]
use crate::entry as constructor;
use crate::models::config::SlotConfig;

use crate::models::metadata::TokenMetadata;
use crate::models::token::TokenConfig;
use crate::models::traits::{TraitResp, TraitWithMetadataResp};
use crate::msg::{
    AllNftInfoMsg, AllNftInfoResp, EquipMsg, ExecuteMsg, InstantiateMsg, NftInfoMsg, NftInfoResp,
    QueryMsg, TraitsMsg, TraitsResp,
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

/// Test if a trait token equips correctly
#[test]
fn equip_allowed_trait() {
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

    // Validate added traits
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

    assert_eq!(
        resp,
        TraitsResp {
            traits: vec![TraitResp {
                token_id: BASE_TOKEN_ID.to_string(),
                token: TokenConfig {
                    address: Addr::unchecked(trait_contract.clone()),
                    token_id: TRAIT_TOKEN_ID.to_string(),
                },
                slot: Some("slot".to_string()),
            }]
        }
    );
}

/// Test equip trait token not listed as a slot contract - should fail
#[test]
fn equip_not_allowed_trait() {
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

    // Equip trait
    let resp = app.execute_contract(
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
    );

    assert!(resp.is_err());
}

/// Test equip not owned nor approved trait to owned base token - should fail
#[test]
fn equip_not_owned_trait() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query::<Extension, TraitExtension, MergedExtension>,
    );
    let code_id = app.store_code(Box::new(code));

    // Instantiate cw721 contracts
    let base_owner = Addr::unchecked("base_owner");
    let base_contract = instantiate_cw721::<Extension>(&mut app, &base_owner, "BASE");
    let trait_owner = Addr::unchecked("trait_owner");
    let trait_contract = instantiate_cw721::<TraitExtension>(&mut app, &trait_owner, "TRAIT");

    // Mint base token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: BASE_TOKEN_ID.to_string(),
        owner: base_owner.to_string(),
        token_uri: None,
        extension: Extension {
            name: "Collection".to_string(),
            value: 10,
        },
    });
    app.execute_contract(base_owner.clone(), base_contract.clone(), &mint_msg, &[])
        .unwrap();

    // Mint trait token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: TRAIT_TOKEN_ID.to_string(),
        owner: trait_owner.to_string(),
        token_uri: None,
        extension: TraitExtension { value: 2 },
    });
    app.execute_contract(trait_owner.clone(), trait_contract.clone(), &mint_msg, &[])
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

    // Equip trait not owned nor approved
    let resp = app.execute_contract(
        base_owner.clone(),
        constructor_contract.clone(),
        &ExecuteMsg::Equip(EquipMsg {
            token_id: BASE_TOKEN_ID.to_owned(),
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    );

    assert!(resp.is_err());
}

/// Test equip not owned but approved trait to owned base token
#[test]
fn equip_approved_trait() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query::<Extension, TraitExtension, MergedExtension>,
    );
    let code_id = app.store_code(Box::new(code));

    // Instantiate cw721 contracts
    let base_owner = Addr::unchecked("base_owner");
    let base_contract = instantiate_cw721::<Extension>(&mut app, &base_owner, "BASE");
    let trait_owner = Addr::unchecked("trait_owner");
    let trait_contract = instantiate_cw721::<TraitExtension>(&mut app, &trait_owner, "TRAIT");

    // Mint base token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: BASE_TOKEN_ID.to_string(),
        owner: base_owner.to_string(),
        token_uri: None,
        extension: Extension {
            name: "Collection".to_string(),
            value: 10,
        },
    });
    app.execute_contract(base_owner.clone(), base_contract.clone(), &mint_msg, &[])
        .unwrap();

    // Mint trait token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: TRAIT_TOKEN_ID.to_string(),
        owner: trait_owner.to_string(),
        token_uri: None,
        extension: TraitExtension { value: 2 },
    });
    app.execute_contract(trait_owner.clone(), trait_contract.clone(), &mint_msg, &[])
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

    // Approve first and then try to equip
    // - approve
    let approve_msg = Cw721ExecuteMsg::Approve {
        token_id: TRAIT_TOKEN_ID.to_string(),
        spender: base_owner.clone().into(),
        expires: None,
    };
    app.execute_contract(
        trait_owner.clone(),
        trait_contract.clone(),
        &approve_msg,
        &[],
    )
    .unwrap();

    // - equip
    app.execute_contract(
        base_owner.clone(),
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

    assert_eq!(
        resp,
        TraitsResp {
            traits: vec![TraitResp {
                token_id: BASE_TOKEN_ID.to_string(),
                token: TokenConfig {
                    address: Addr::unchecked(trait_contract.clone()),
                    token_id: TRAIT_TOKEN_ID.to_string(),
                },
                slot: Some("slot".to_string()),
            }]
        }
    );
}

/// Test equip a trait on not owned nor approved base token - should fail
#[test]
fn equip_on_not_owned_token() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query::<Extension, TraitExtension, MergedExtension>,
    );
    let code_id = app.store_code(Box::new(code));

    // Instantiate cw721 contracts
    let base_owner = Addr::unchecked("base_owner");
    let base_contract = instantiate_cw721::<Extension>(&mut app, &base_owner, "BASE");

    let trait_owner = Addr::unchecked("trait_owner");
    let trait_contract = instantiate_cw721::<TraitExtension>(&mut app, &trait_owner, "TRAIT");

    // Mint base token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: BASE_TOKEN_ID.to_string(),
        owner: base_owner.to_string(),
        token_uri: None,
        extension: Extension {
            name: "Collection".to_string(),
            value: 10,
        },
    });
    app.execute_contract(base_owner.clone(), base_contract.clone(), &mint_msg, &[])
        .unwrap();

    // Mint trait token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: TRAIT_TOKEN_ID.to_string(),
        owner: trait_owner.to_string(),
        token_uri: None,
        extension: TraitExtension { value: 2 },
    });
    app.execute_contract(trait_owner.clone(), trait_contract.clone(), &mint_msg, &[])
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

    // Equip a trait on base token not owned nor approved
    let resp = app.execute_contract(
        trait_owner.clone(),
        constructor_contract.clone(),
        &ExecuteMsg::Equip(EquipMsg {
            token_id: BASE_TOKEN_ID.to_owned(),
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    );

    assert!(resp.is_err());
}

/// Test equip a trait on approved base token
#[test]
fn equip_on_approved_token() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        constructor::execute,
        constructor::instantiate,
        constructor::query::<Extension, TraitExtension, MergedExtension>,
    );
    let code_id = app.store_code(Box::new(code));

    // Instantiate cw721 contracts
    let base_owner = Addr::unchecked("base_owner");
    let base_contract = instantiate_cw721::<Extension>(&mut app, &base_owner, "BASE");

    let trait_owner = Addr::unchecked("trait_owner");
    let trait_contract = instantiate_cw721::<TraitExtension>(&mut app, &trait_owner, "TRAIT");

    // Mint base token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: BASE_TOKEN_ID.to_string(),
        owner: base_owner.to_string(),
        token_uri: None,
        extension: Extension {
            name: "Collection".to_string(),
            value: 10,
        },
    });
    app.execute_contract(base_owner.clone(), base_contract.clone(), &mint_msg, &[])
        .unwrap();

    // Mint trait token
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: TRAIT_TOKEN_ID.to_string(),
        owner: trait_owner.to_string(),
        token_uri: None,
        extension: TraitExtension { value: 2 },
    });
    app.execute_contract(trait_owner.clone(), trait_contract.clone(), &mint_msg, &[])
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
                admins: None,
            },
            &[],
            "Character",
            None,
        )
        .unwrap();

    // Approve first and then try to equip
    // - approve
    let approve_msg = Cw721ExecuteMsg::Approve {
        token_id: BASE_TOKEN_ID.to_string(),
        spender: trait_owner.clone().into(),
        expires: None,
    };
    app.execute_contract(base_owner.clone(), base_contract.clone(), &approve_msg, &[])
        .unwrap();

    // - equip
    app.execute_contract(
        trait_owner.clone(),
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

    assert_eq!(
        resp,
        TraitsResp {
            traits: vec![TraitResp {
                token_id: BASE_TOKEN_ID.to_string(),
                token: TokenConfig {
                    address: Addr::unchecked(trait_contract.clone()),
                    token_id: TRAIT_TOKEN_ID.to_string(),
                },
                slot: Some("slot".to_string()),
            }]
        }
    );
}

/// Test `nft_info` after a trait is equipped
#[test]
fn equipped_nft_info() {
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
                    extension: MergedExtension { value: 12 }
                }
            }
        }
    );
}

/// Test `all_nft_info` after a trait is equipped
#[test]
fn equipped_all_nft_info() {
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

/// Test equip trait even if slot is already taken - should fail
#[test]
fn equip_taken_slot() {
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

    // Mint trait token:
    // - token_id=1
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: "1".to_string(),
        owner: minter.to_string(),
        token_uri: None,
        extension: TraitExtension { value: 1 },
    });
    app.execute_contract(minter.clone(), trait_contract.clone(), &mint_msg, &[])
        .unwrap();
    // - token_id=2
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: "2".to_string(),
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
                token_id: "1".to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    )
    .unwrap();

    // Equip another trait as the same slot
    let resp = app.execute_contract(
        minter.clone(),
        constructor_contract.clone(),
        &ExecuteMsg::Equip(EquipMsg {
            token_id: BASE_TOKEN_ID.to_owned(),
            traits: vec![TokenConfig {
                token_id: "2".to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    );

    assert!(resp.is_err());
}

/// Test equip trait even if slot is already taken with `allow_multiple` set (should succeed)
#[test]
fn equip_taken_slot_with_multiple_allowed() {
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

    // Mint trait token:
    // - token_id=1
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: "1".to_string(),
        owner: minter.to_string(),
        token_uri: None,
        extension: TraitExtension { value: 1 },
    });
    app.execute_contract(minter.clone(), trait_contract.clone(), &mint_msg, &[])
        .unwrap();
    // - token_id=2
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: "2".to_string(),
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
                    allow_multiple: true,
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
                token_id: "1".to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    )
    .unwrap();

    // Equip another trait as the same slot
    app.execute_contract(
        minter.clone(),
        constructor_contract.clone(),
        &ExecuteMsg::Equip(EquipMsg {
            token_id: BASE_TOKEN_ID.to_owned(),
            traits: vec![TokenConfig {
                token_id: "2".to_string(),
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

    assert_eq!(
        resp,
        TraitsResp {
            traits: vec![
                TraitResp {
                    token_id: BASE_TOKEN_ID.to_string(),
                    token: TokenConfig {
                        address: Addr::unchecked(trait_contract.clone()),
                        token_id: "1".to_string(),
                    },
                    slot: Some("slot".to_string()),
                },
                TraitResp {
                    token_id: BASE_TOKEN_ID.to_string(),
                    token: TokenConfig {
                        address: Addr::unchecked(trait_contract.clone()),
                        token_id: "2".to_string(),
                    },
                    slot: Some("slot".to_string()),
                }
            ]
        }
    );
}

/// Test equip trait already equipped on the same token - should fail
#[test]
fn equip_equipped_trait_on_same_token() {
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
                    allow_multiple: true,
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

    // Try to equip again
    let resp = app.execute_contract(
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
    );

    assert!(resp.is_err());
}

/// Test equip trait already equipped on a different token - should fail
#[test]
fn equip_equipped_trait_on_different_token() {
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

    // Mint base token:
    // - token_id=1
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: "1".to_string(),
        owner: minter.to_string(),
        token_uri: None,
        extension: Extension {
            name: "Collection".to_string(),
            value: 10,
        },
    });
    app.execute_contract(minter.clone(), base_contract.clone(), &mint_msg, &[])
        .unwrap();
    // - token_id=2
    let mint_msg = Cw721BaseExecuteMsg::Mint(MintMsg {
        token_id: "2".to_string(),
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
                    allow_multiple: true,
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
            token_id: "1".to_owned(),
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    )
    .unwrap();

    // Try to equip again on different base token
    let resp = app.execute_contract(
        minter.clone(),
        constructor_contract.clone(),
        &ExecuteMsg::Equip(EquipMsg {
            token_id: "2".to_owned(),
            traits: vec![TokenConfig {
                token_id: TRAIT_TOKEN_ID.to_string(),
                address: trait_contract.to_string(),
            }],
        }),
        &vec![],
    );

    assert!(resp.is_err());
}
