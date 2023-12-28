use crate::error::ContractError;
use cosmwasm_std::Response;

pub fn execute_greet() -> Result<Response, ContractError> {
    Ok(Response::new())
}
