#![cfg(test)]
use crate::{
    msg::{ContractInfoMsg, ContractInfoResp, ExecuteMsg, QueryMsg, SetCw721Msg},
    tests::utils::{
        init::init_minter,
        metadata::Extension,
        shared::{ADMIN, USER},
    },
};
use cosmwasm_std::Addr;
use cw_multi_test::{App, Executor};

/// Test base setting cw721 as an admin
#[test]
fn set_cw721() {
    let mut app = App::default();
    let minter_contract = init_minter(&mut app, None, None, None, Some(vec![ADMIN.to_string()]));

    // Set cw721
    let update_msg = ExecuteMsg::SetCw721(SetCw721Msg {
        address: "cw721".to_string(),
    });
    app.execute_contract(
        Addr::unchecked(ADMIN),
        minter_contract.clone(),
        &update_msg,
        &[],
    )
    .unwrap();

    // Validate contract info
    let info: ContractInfoResp<Extension> = app
        .wrap()
        .query_wasm_smart(
            &minter_contract,
            &QueryMsg::ContractInfo(ContractInfoMsg {}),
        )
        .unwrap();

    assert_eq!(info.cw721, Some(Addr::unchecked("cw721")));
}

/// Test if returns an error if trying to set cw721 as not an admin
#[test]
fn set_cw721_unauthorized() {
    let mut app = App::default();
    let minter_contract = init_minter(&mut app, None, None, None, Some(vec![ADMIN.to_string()]));

    // Set cw721
    let update_msg = ExecuteMsg::SetCw721(SetCw721Msg {
        address: "cw721".to_string(),
    });
    let resp = app.execute_contract(Addr::unchecked(USER), minter_contract, &update_msg, &[]);

    assert!(resp.is_err());
}

/// Test if returns an error if provide an invalid address
#[test]
fn set_cw721_invalid() {
    let mut app = App::default();
    let minter_contract = init_minter(&mut app, None, None, None, Some(vec![ADMIN.to_string()]));

    // Set cw721
    let update_msg = ExecuteMsg::SetCw721(SetCw721Msg {
        address: "".to_string(),
    });
    let resp = app.execute_contract(Addr::unchecked(ADMIN), minter_contract, &update_msg, &[]);

    assert!(resp.is_err());
}
