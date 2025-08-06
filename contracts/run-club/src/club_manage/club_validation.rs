use soroban_sdk::{Address, Env};

use crate::{Club, DataKey, RunClubContract};

impl RunClubContract {
    /// Verifica se um clube existe
    pub fn club_exists(env: Env, club_id: u64) -> bool {
        env.storage().persistent().has(&DataKey::Club(club_id))
    }

    /// Verifica se um clube está ativo
    pub fn is_club_active(env: Env, club_id: u64) -> bool {
        if let Some(club) = env
            .storage()
            .persistent()
            .get::<DataKey, Club>(&DataKey::Club(club_id))
        {
            club.is_active
        } else {
            false
        }
    }

    /// Verifica se um clube ainda está dentro do período válido
    pub fn is_club_period_valid(env: Env, club_id: u64) -> bool {
        if let Some(club) = env
            .storage()
            .persistent()
            .get::<DataKey, Club>(&DataKey::Club(club_id))
        {
            let current_timestamp = env.ledger().timestamp();
            current_timestamp <= club.month_end_timestamp
        } else {
            false
        }
    }

    /// Verifica se um usuário é o organizador de um clube
    pub fn is_club_organizer(env: Env, club_id: u64, user: Address) -> bool {
        if let Some(club) = env
            .storage()
            .persistent()
            .get::<DataKey, Club>(&DataKey::Club(club_id))
        {
            club.organizer == user
        } else {
            false
        }
    }

    /// Verifica se um clube tem membros
    pub fn has_members(env: Env, club_id: u64) -> bool {
        if let Some(club) = env
            .storage()
            .persistent()
            .get::<DataKey, Club>(&DataKey::Club(club_id))
        {
            !club.members.is_empty()
        } else {
            false
        }
    }
}
