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

    /// Get all members of a club
    pub fn get_members(env: Env, club_id: u64) -> Vec<Address> {
        let club = Self::get_club(env.clone(), club_id);
        club.members
    }
}