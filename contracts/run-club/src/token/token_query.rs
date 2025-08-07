use soroban_sdk::{Address, Env};
use crate::DataKey;

pub fn balance(env: &Env, id: Address) -> i128 {
    env.storage().instance().get(&DataKey::Balance(id)).unwrap_or(0)
}

pub fn total_supply(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
}