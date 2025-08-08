use soroban_sdk::{contractclient, contractimpl, contracttype, Address, Env, String, BytesN};
use soroban_token_contract::TokenClient;

#[contracttype]
pub enum DataKey {
    TokenWasmHash,
}

#[contractclient(name = "TokenClient")]
pub trait TokenTrait {
    fn initialize(env: Env, admin: Address, decimal: u32, name: String, symbol: String);
}

pub fn create_token(env: &Env, admin: &Address, name: &String, symbol: &String) -> Address {
    let token_wasm_hash = env.storage().persistent().get(&DataKey::TokenWasmHash).expect("Token Wasm Hash not set");

    let token_id = env.deployer().upload_contract_wasm(token_wasm_hash);

    let token_client = TokenClient::new(env, &token_id);
    token_client.initialize(admin, 7, name, symbol);

    token_id
}

pub fn set_token_wasm_hash(env: Env, wasm_hash: soroban_sdk::BytesN<32>) {
    env.storage().persistent().set(&DataKey::TokenWasmHash, &wasm_hash);
}


