use crate::msg::GreetResp;
use cosmwasm_std::StdResult;

pub fn query_greet() -> StdResult<GreetResp> {
    let resp = GreetResp {
        message: "Hey".to_owned(),
    };
    Ok(resp)
}
