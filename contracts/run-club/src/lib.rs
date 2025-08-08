#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WithdrawalRule {
    Equal,
    Unlimited,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Club {
    pub id: u64,
    pub name: String,
    pub organizer: Address,
    pub members: Vec<Address>,
    pub usdc_deposited: i128,
    pub usdc_per_km: i128,
    pub withdrawal_rule: WithdrawalRule,
    pub month_end_timestamp: u64,
    pub is_active: bool,
    pub token_address: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Club(u64),
    ClubCounter,
    UserKmTokens(Address, u64),
    UserClubs(Address),
}

#[contract]
pub struct RunClubContract;

#[contractimpl]
impl RunClubContract {
    pub fn initialize(env: Env) {}

    pub fn get_club(env: Env, club_id: u64) -> Club;

    pub fn activate(env: Env, club_id: u64, organizer: Address);

    pub fn get_active_clubs(env: Env) -> Vec<u64>;

    pub fn add_member(env: Env, club_id: u64, member: Address);

    pub fn get_members(env: Env, club_id: u64) -> Vec<Address>;

    pub fn deposit_usdc(env: Env, club_id: u64, organizer: Address, amount: i128);

    pub fn add_km_tokens(env: Env, club_id: u64, user: Address, km_amount: i128);

    pub fn get_user_km_tokens(env: Env, user: Address, club_id: u64) -> i128;

    pub fn is_club_period_ended(env: Env, club_id: u64) -> bool;

    pub fn get_total_km_tokens(env: Env, club_id: u64) -> i128;

    pub fn calculate_usdc_reward(env: Env, club_id: u64, user: Address) -> i128;

    pub fn redeem_usdc(env: Env, club_id: u64, user: Address, destination: Address) -> i128;

    pub fn get_redemption_info(env: Env, club_id: u64, user: Address) -> (i128, i128, bool);

    pub fn get_user_clubs(env: Env, user: Address) -> Vec<u64>;
}
