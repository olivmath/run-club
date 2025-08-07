use soroban_sdk::{Address, Env};
use crate::DataKey;

pub fn mint(env: &Env, to: Address, amount: i128) {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();
    let mut total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
    total_supply += amount;
    env.storage().instance().set(&DataKey::TotalSupply, &total_supply);
    let mut balance: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap_or(0);
    balance += amount;
    env.storage().instance().set(&DataKey::Balance(to.clone()), &balance);
    env.events().publish((symbol_short!("mint"), to), amount);
}

pub fn burn(env: &Env, from: Address, amount: i128) {
    from.require_auth();
    let mut balance: i128 = env.storage().instance().get(&DataKey::Balance(from.clone())).unwrap_or(0);
    if balance < amount {
        panic!("insufficient balance");
    }
    balance -= amount;
    env.storage().instance().set(&DataKey::Balance(from.clone()), &balance);
    let mut total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
    total_supply -= amount;
    env.storage().instance().set(&DataKey::TotalSupply, &total_supply);
    env.events().publish((symbol_short!("burn"), from), amount);
}

pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) {
    from.require_auth();
    let mut from_balance: i128 = env.storage().instance().get(&DataKey::Balance(from.clone())).unwrap_or(0);
    if from_balance < amount {
        panic!("insufficient balance");
    }
    from_balance -= amount;
    env.storage().instance().set(&DataKey::Balance(from.clone()), &from_balance);
    let mut to_balance: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap_or(0);
    to_balance += amount;
    env.storage().instance().set(&DataKey::Balance(to.clone()), &to_balance);
    env.events().publish((symbol_short!("transfer"), from, to), amount);
}