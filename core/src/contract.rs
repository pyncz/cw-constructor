use crate::error::ContractError;
use crate::execute::{execute_apply, execute_exempt, execute_exempt_all};
use crate::instantiate::init;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_config, query_traits};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    init(msg, deps)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig(msg) => to_json_binary(&query_config(msg, deps)?),
        QueryMsg::GetTraits(msg) => to_json_binary(&query_traits(msg, deps)?),
    }
}

#[allow(dead_code)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Apply(msg) => execute_apply(msg, deps),
        ExecuteMsg::Exempt(msg) => execute_exempt(msg, deps),
        ExecuteMsg::ExemptAll(msg) => execute_exempt_all(msg, deps),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::TokenConfig,
        msg::{GetConfigMsg, GetConfigResp},
    };
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    #[test]
    fn test_instantiate() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let base_token_unchecked = TokenConfig {
            address: "base_token".to_string(),
            token_id: "1".to_string(),
        };
        let base_token: TokenConfig = TokenConfig {
            address: Addr::unchecked("base_token"),
            token_id: "1".to_string(),
        };

        // Case: No admins
        let contract_address = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    base_token: base_token_unchecked.clone(),
                    allowed_traits_addresses: None,
                    allow_multiple_tokens_per_contract: None,
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
                base_token: base_token.clone(),
                allowed_traits_addresses: vec![],
                allow_multiple_tokens_per_contract: false,
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
                    allowed_traits_addresses: None,
                    allow_multiple_tokens_per_contract: None,
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
                allowed_traits_addresses: vec![],
                allow_multiple_tokens_per_contract: false,
                admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
            }
        );
    }
}
