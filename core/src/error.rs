use cosmwasm_std::{Addr, Response, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{sender} is not contract admin")]
    NotAdmin { sender: Addr },

    #[error("{sender} is authorized for this action")]
    Unauthorized { sender: Addr },

    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),

    #[error("This token is already applied as a trait")]
    TraitTokenAlreadyApplied { address: Addr, token_id: String },

    #[error("Not instantiated!")]
    NotInstantiated {},

    #[error("\"{slot}\" slot of the #{token_id} base token is already taken")]
    TraitSlotTaken { token_id: String, slot: String },

    #[error("Cannot contain multiple configs for one slot")]
    SlotConfigDuplicateName {},

    #[error("Cannot contain the same address in multiple slots' configs")]
    SlotConfigDuplicateAddress {},

    #[error("No slot supports the provided address: {address}")]
    UnknownTraitAddress { address: Addr },

    #[error("Cannot contain duplicate tokens as traits to equip")]
    TraitDuplicateToken {},

    #[error("Cannot contain tokens racing for the same slot of the base token")]
    TraitTokenRace {},
}

pub type ContractResult<T = ()> = Result<T, ContractError>;
pub type ContractResponse = ContractResult<Response>;
