use cosmwasm_std::{Addr, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },

    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),

    #[error("Tokens from this collection are not allowed to be used as traits")]
    TraitContractNotAllowed { address: Addr },

    #[error("This token is already applied as a trait")]
    TraitTokenAlreadyApplied { address: Addr, token_id: String },
}
