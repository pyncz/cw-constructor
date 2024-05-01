#![cfg(test)]
use super::utils::{
    cw721::instantiate_cw721,
    entry::InstantiateMsg,
    metadata::Extension,
    shared::{ADMIN, USER},
};
use crate::{
    models::config::ExtensionsConfig,
    msg::{ExecuteMsg, MintMsg, SetCw721Msg},
    tests::utils::entry as minter,
};
use cosmwasm_std::{coins, Addr, Coin, Uint128};
use cw721::{Cw721ExecuteMsg, Cw721QueryMsg, NumTokensResponse, TokensResponse};
use cw_multi_test::{App, ContractWrapper, Executor};

fn init_cw721_with_minter(app: &mut App, price: Option<Coin>, supply: Option<u32>) -> (Addr, Addr) {
    let code = ContractWrapper::new(minter::execute, minter::instantiate, minter::query);
    let code_id = app.store_code(Box::new(code));

    // Instantiate minter contract
    let minter_contract = app
        .instantiate_contract(
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
        .unwrap();

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

/// Test base mint with no funds required
#[test]
fn mint() {
    let mut app = App::default();
    let (minter_contract, cw721_contract) = init_cw721_with_minter(&mut app, None, None);

    // Mint
    let user = Addr::unchecked(USER);
    let mint_msg = ExecuteMsg::Mint(MintMsg {});
    app.execute_contract(user.clone(), minter_contract, &mint_msg, &[])
        .unwrap();

    // Validate num_tokens
    let resp: NumTokensResponse = app
        .wrap()
        .query_wasm_smart(&cw721_contract, &Cw721QueryMsg::NumTokens {})
        .unwrap();
    assert_eq!(resp, NumTokensResponse { count: 1 });

    // Validate owner's token_ids
    let tokens_msg = Cw721QueryMsg::Tokens {
        owner: user.into(),
        start_after: None,
        limit: None,
    };
    let resp: TokensResponse = app
        .wrap()
        .query_wasm_smart(&cw721_contract, &tokens_msg)
        .unwrap();
    assert_eq!(
        resp,
        TokensResponse {
            tokens: vec!["1".to_string()]
        }
    );
}

/// Test minting multiple items
#[test]
fn mint_multiple() {
    let mut app = App::default();
    let (minter_contract, cw721_contract) = init_cw721_with_minter(&mut app, None, None);

    // Mint two and check if the token_ids are sequential
    let user = Addr::unchecked(USER);
    let mint_msg = ExecuteMsg::Mint(MintMsg {});
    app.execute_contract(user.clone(), minter_contract.clone(), &mint_msg, &[])
        .unwrap();
    app.execute_contract(user.clone(), minter_contract.clone(), &mint_msg, &[])
        .unwrap();

    let tokens_msg = Cw721QueryMsg::Tokens {
        owner: user.clone().into(),
        start_after: None,
        limit: None,
    };
    let resp: TokensResponse = app
        .wrap()
        .query_wasm_smart(&cw721_contract, &tokens_msg)
        .unwrap();
    assert_eq!(
        resp,
        TokensResponse {
            tokens: vec!["1".to_string(), "2".to_string()]
        }
    );

    // Burn one before minting one and checking if the token_ids are expected
    let burn_msg = Cw721ExecuteMsg::Burn {
        token_id: "2".to_string(),
    };
    app.execute_contract(user.clone(), cw721_contract.clone(), &burn_msg, &[])
        .unwrap();
    app.execute_contract(user, minter_contract, &mint_msg, &[])
        .unwrap();
    let resp: TokensResponse = app
        .wrap()
        .query_wasm_smart(&cw721_contract, &tokens_msg)
        .unwrap();
    assert_eq!(
        resp,
        TokensResponse {
            tokens: vec!["1".to_string(), "3".to_string()]
        }
    );
}

/// Test mint with required funds
#[test]
fn mint_with_required_funds() {
    let user = Addr::unchecked(USER);
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &user, coins(15, "gold"))
            .unwrap()
    });
    let price = Coin {
        amount: Uint128::new(5),
        denom: "gold".to_string(),
    };
    let (minter_contract, _) = init_cw721_with_minter(&mut app, Some(price.clone()), None);

    // Mint
    let user = Addr::unchecked(USER);
    let mint_msg = ExecuteMsg::Mint(MintMsg {});
    app.execute_contract(user, minter_contract.clone(), &mint_msg, &[price])
        .unwrap();

    // Check if funds are transferred
    assert_eq!(
        app.wrap()
            .query_balance(USER, "gold")
            .unwrap()
            .amount
            .u128(),
        10
    );
    // Check if funds are received
    assert_eq!(
        app.wrap()
            .query_balance(minter_contract, "gold")
            .unwrap()
            .amount
            .u128(),
        5
    );
}

/// Test mint with required funds when no amount attached
#[test]
fn mint_with_required_funds_empty() {
    let user = Addr::unchecked(USER);
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &user, coins(15, "gold"))
            .unwrap()
    });
    let (minter_contract, _) = init_cw721_with_minter(
        &mut app,
        Some(Coin {
            amount: Uint128::new(5),
            denom: "gold".to_string(),
        }),
        None,
    );

    // Mint
    let user = Addr::unchecked(USER);
    let mint_msg = ExecuteMsg::Mint(MintMsg {});
    let resp = app.execute_contract(user, minter_contract, &mint_msg, &[]);
    assert!(resp.is_err());
}

/// Test mint with required funds when insufficient amount attached
#[test]
fn mint_with_required_funds_insufficient() {
    let user = Addr::unchecked(USER);
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &user, coins(15, "gold"))
            .unwrap()
    });
    let (minter_contract, _) = init_cw721_with_minter(
        &mut app,
        Some(Coin {
            amount: Uint128::new(5),
            denom: "gold".to_string(),
        }),
        None,
    );

    // Mint
    let user = Addr::unchecked(USER);
    let mint_msg = ExecuteMsg::Mint(MintMsg {});
    let resp = app.execute_contract(
        user,
        minter_contract,
        &mint_msg,
        &[Coin {
            amount: Uint128::new(4),
            denom: "gold".to_string(),
        }],
    );
    assert!(resp.is_err());
}

/// Test mint with required funds but passing them separately
#[test]
fn mint_with_separated_funds() {
    let user = Addr::unchecked(USER);
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &user, coins(15, "gold"))
            .unwrap()
    });
    let (minter_contract, _) = init_cw721_with_minter(
        &mut app,
        Some(Coin {
            amount: Uint128::new(5),
            denom: "gold".to_string(),
        }),
        None,
    );

    // Mint
    let user = Addr::unchecked(USER);
    let mint_msg = ExecuteMsg::Mint(MintMsg {});
    app.execute_contract(
        user,
        minter_contract.clone(),
        &mint_msg,
        &[
            Coin {
                amount: Uint128::new(4),
                denom: "gold".to_string(),
            },
            Coin {
                amount: Uint128::new(1),
                denom: "gold".to_string(),
            },
        ],
    )
    .unwrap();

    // Check if funds are transferred
    assert_eq!(
        app.wrap()
            .query_balance(USER, "gold")
            .unwrap()
            .amount
            .u128(),
        10
    );
    // Check if funds are received
    assert_eq!(
        app.wrap()
            .query_balance(minter_contract, "gold")
            .unwrap()
            .amount
            .u128(),
        5
    );
}

/// Test trying to mint more than supply
#[test]
fn mint_out_supply() {
    let mut app = App::default();
    let supply = 5;
    let (minter_contract, _) = init_cw721_with_minter(&mut app, None, Some(supply));

    // Mint supply
    let user = Addr::unchecked(USER);
    let mint_msg = ExecuteMsg::Mint(MintMsg {});
    for i in 0..supply {
        println!("{}", i);
        app.execute_contract(user.clone(), minter_contract.clone(), &mint_msg, &[])
            .unwrap();
    }

    // Try to mint (supply + 1)
    let resp = app.execute_contract(user, minter_contract, &mint_msg, &[]);
    assert!(resp.is_err());
}

/// Test trying to mint more than supply even is some tokens are burned
#[test]
fn mint_out_supply_with_burn() {
    let mut app = App::default();
    let supply = 5;
    let (minter_contract, cw721_contract) = init_cw721_with_minter(&mut app, None, Some(supply));

    // Mint supply
    let user = Addr::unchecked(USER);
    let mint_msg = ExecuteMsg::Mint(MintMsg {});
    for i in 0..supply {
        println!("{}", i);
        app.execute_contract(user.clone(), minter_contract.clone(), &mint_msg, &[])
            .unwrap();
    }
    // Burn one
    let burn_msg = Cw721ExecuteMsg::Burn {
        token_id: "2".to_string(),
    };
    app.execute_contract(user.clone(), cw721_contract, &burn_msg, &[])
        .unwrap();

    // Try to mint (supply + 1)
    let resp = app.execute_contract(user, minter_contract, &mint_msg, &[]);
    assert!(resp.is_err());
}
