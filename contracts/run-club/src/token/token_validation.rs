use soroban_sdk::{Address, Env};
use crate::DataKey;

pub fn validate_admin(env: &Env, caller: Address) {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    if caller != admin {
        panic!("not authorized");
    }
}

pub fn check_sufficient_balance(env: &Env, account: Address, amount: i128) {
    let balance: i128 = env.storage().instance().get(&DataKey::Balance(account)).unwrap_or(0);
    if balance < amount {
        panic!("insufficient balance");
    }
}