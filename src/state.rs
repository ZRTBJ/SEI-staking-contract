use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub collection_address: Addr,
    pub owner: Addr,
    pub lock_time: u64,
    pub enabled: bool,
    pub total_supply: u64
}

pub const CONFIG: Item<Config> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingInfo {
    pub address: Addr,
    pub token_ids: Vec<String>,
}

pub const STAKING: Map<Addr, StakingInfo> = Map::new("staking");

pub const UNLOCK_TIME: Map<String, u64> = Map::new("unlock_time");