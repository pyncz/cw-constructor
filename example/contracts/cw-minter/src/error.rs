use cosmwasm_std::{Response, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Not instantiated!")]
    NotInstantiated {},

    #[error("Invalid value! Supply cannot be {supply}")]
    InvalidSupply { supply: u32 },

    #[error("Invalid value! Amount cannot be {amount}")]
    InvalidAmount { amount: u128 },

    #[error("The collection has already reached the supply ({supply})")]
    MintedOut { supply: u32 },

    #[error("Insufficient amount! Required {required} {denom}, sent {sent}")]
    InsufficientFunds {
        denom: String,
        required: u128,
        sent: u128,
    },
}

pub type ContractResult<T = ()> = Result<T, ContractError>;
pub type ContractResponse = ContractResult<Response>;
