use crate::models::{Config, TokenConfig};
use cw_storage_plus::Item;

pub const CONFIG: Item<Config> = Item::new("config");

pub const TRAITS: Item<Vec<TokenConfig>> = Item::new("traits");
