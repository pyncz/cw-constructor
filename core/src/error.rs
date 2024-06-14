use cosmwasm_std::{Addr, Response, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // Migration errors
    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("Cannot change contract name from \"{original_name}\" to \"{new_name}\"! The name should be immutable")]
    InvalidMigrationContractName {
        original_name: String,
        new_name: String,
    },

    #[error("Cannot change contract version from {current_version} to {new_version}! New version should be higher than the current one")]
    InvalidMigrationVersion {
        current_version: String,
        new_version: String,
    },

    // Execution errors
    #[error("Unauthorized! {sender} is not contract admin")]
    NotAdmin { sender: Addr },

    #[error("Unauthorized! {sender} is not authorized for this action")]
    Unauthorized { sender: Addr },

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

    #[error("Not equipped!")]
    NotEquipped {},
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}

pub type ContractResult<T = ()> = Result<T, ContractError>;
pub type ContractResponse = ContractResult<Response>;
