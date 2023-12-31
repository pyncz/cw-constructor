use crate::models::config::SlotConfig;
use crate::state::CONFIG;
use cosmwasm_std::{Addr, Deps, StdResult};
use std::collections::HashSet;
use std::hash::Hash;

pub fn all_unique<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(|x| uniq.insert(x))
}

pub fn get_slot_config_by_address(address: &Addr, deps: &Deps) -> StdResult<Option<SlotConfig>> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config
        .slots
        .into_iter()
        .find(|slot| slot.allowed_contracts.contains(address)))
}
