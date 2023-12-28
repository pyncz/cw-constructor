use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    ListAdmins {},
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg {
    Greet {},
}

// Responses
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ListAdminsResp {
    pub admins: Vec<Addr>,
}
