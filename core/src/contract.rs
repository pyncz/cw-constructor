use crate::error::ContractResponse;
use crate::execute;
use crate::instantiate::init;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query;
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResponse {
    init(msg, deps)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig(msg) => to_json_binary(&query::config(&msg, &deps)?),
        QueryMsg::GetTraits(msg) => to_json_binary(&query::traits(&msg, &deps)?),
        QueryMsg::GetTokens(msg) => to_json_binary(&query::tokens(&msg, &deps)?),
    }
}

#[allow(dead_code)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> ContractResponse {
    match msg {
        ExecuteMsg::Apply(msg) => execute::apply(msg, deps, info),
        ExecuteMsg::Exempt(msg) => execute::exempt(msg, deps, info),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::{GetConfigMsg, GetConfigResp};
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    #[test]
    fn test_instantiate() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let base_token_unchecked = "base_token".to_string();
        let base_token = Addr::unchecked("base_token");

        // Case: No admins
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

        let resp: GetConfigResp = app
            .wrap()
            .query_wasm_smart(contract_address, &QueryMsg::GetConfig(GetConfigMsg {}))
            .unwrap();

        assert_eq!(
            resp,
            GetConfigResp {
                base_token: base_token.to_owned(),
                slots: vec![],
                admins: vec![]
            }
        );

        // Case: Some admins
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

        let resp: GetConfigResp = app
            .wrap()
            .query_wasm_smart(contract_address, &QueryMsg::GetConfig(GetConfigMsg {}))
            .unwrap();

        assert_eq!(
            resp,
            GetConfigResp {
                base_token,
                slots: vec![],
                admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
            }
        );
    }
}
