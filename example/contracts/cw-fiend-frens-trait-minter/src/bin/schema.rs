use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use cw_minter::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<Empty>,
        execute: ExecuteMsg,
        query: QueryMsg
    }
}
