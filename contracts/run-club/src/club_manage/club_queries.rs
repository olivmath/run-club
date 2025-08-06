use soroban_sdk::{Address, Env, Vec};

use crate::{Club, DataKey, RunClubContract};

impl RunClubContract {
    /// Obtém informações de um clube
    pub fn get_club(env: Env, club_id: u64) -> Club {
        env.storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found")
    }

    /// Obtém lista de clubes ativos
    pub fn get_active_clubs(env: Env) -> Vec<u64> {
        let mut active_clubs = Vec::new(&env);
        let club_counter = env
            .storage()
            .persistent()
            .get(&DataKey::ClubCounter)
            .unwrap_or(0u64);

        for club_id in 1..=club_counter {
            if let Some(club) = env.storage().persistent().get::<DataKey, Club>(&DataKey::Club(club_id)) {
                if club.is_active {
                    active_clubs.push_back(club_id);
                }
            }
        }

        active_clubs
    }

    /// Get all members of a club
    pub fn get_members(env: Env, club_id: u64) -> Vec<Address> {
        let club = Self::get_club(env.clone(), club_id);
        club.members
    }
}