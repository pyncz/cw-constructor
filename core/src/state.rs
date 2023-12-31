use crate::models::{config::ContractInfo, traits::Trait};
use cw_storage_plus::Item;

pub const CONFIG: Item<ContractInfo> = Item::new("config");

pub const TRAITS: Item<Vec<Trait>> = Item::new("traits");
