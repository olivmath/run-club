pub mod token_operation;
pub mod token_query;
pub mod token_validation;
use crate::token_factory;

pub fn create_token(env: &Env, admin: &Address, name: &String, symbol: &String) -> Address {
    token_factory::create_token(env, admin, name, symbol)
}


