use crate::error::ContractError;
use crate::execute::execute_greet;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::query_list_admins;
use crate::state::ADMINS;
use cosmwasm_std::{
    entry_point, to_json_binary, Attribute, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|address| deps.api.addr_validate(&address))
        .collect();
    let admins = admins.unwrap();

    ADMINS.save(deps.storage, &admins)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attributes(
            admins
                .into_iter()
                .map(|admin_address| Attribute::new("set_admin", admin_address))
                .collect::<Vec<Attribute>>(),
        ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ListAdmins {} => to_json_binary(&query_list_admins(deps)?),
    }
}

#[allow(dead_code)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Greet {} => execute_greet(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::ListAdminsResp;
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    #[test]
    fn test_instantiate() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        // Case: No admins
        let contract_address = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { admins: vec![] },
                &[],
                "Contract 1",
                None,
            )
            .unwrap();

        let resp: ListAdminsResp = app
            .wrap()
            .query_wasm_smart(contract_address, &QueryMsg::ListAdmins {})
            .unwrap();

        assert_eq!(resp, ListAdminsResp { admins: vec![] });

        // Case: Some admins
        let contract_address = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["admin1".to_owned(), "admin2".to_owned()],
                },
                &[],
                "Contract 2",
                None,
            )
            .unwrap();

        let resp: ListAdminsResp = app
            .wrap()
            .query_wasm_smart(contract_address, &QueryMsg::ListAdmins {})
            .unwrap();

        assert_eq!(
            resp,
            ListAdminsResp {
                admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
            }
        );
    }
}
