use soroban_sdk::{Address, Env, String, Vec};

use crate::{Club, DataKey, RunClubContract, WithdrawalRule};

impl RunClubContract {
    /// Cria um novo clube de corrida
    pub fn create_club(
        env: Env,
        organizer: Address,
        name: String,
        usdc_per_km: i128,
        withdrawal_rule: WithdrawalRule,
        duration_days: u32,
    ) -> u64 {
        organizer.require_auth();

        if usdc_per_km <= 0 {
            panic!("USDC per KM must be positive");
        }

        if duration_days == 0 {
            panic!("Duration must be greater than 0");
        }

        // Obter próximo ID do clube
        let club_counter = env
            .storage()
            .persistent()
            .get(&DataKey::ClubCounter)
            .unwrap_or(0u64);
        let club_id = club_counter + 1;

        // Calcular timestamp de fim do mês
        let current_timestamp = env.ledger().timestamp();
        let month_end_timestamp = current_timestamp + (duration_days as u64 * 24 * 60 * 60);

        let club = Club {
            id: club_id,
            name,
            organizer: organizer.clone(),
            members: Vec::new(&env),
            usdc_deposited: 0,
            usdc_per_km,
            withdrawal_rule,
            month_end_timestamp,
            is_active: false, // Será ativado quando USDC for depositado
        };

        // Salvar clube
        env.storage().persistent().set(&DataKey::Club(club_id), &club);
        env.storage().persistent().set(&DataKey::ClubCounter, &club_id);

        // Emitir evento
        env.events().publish(
            (soroban_sdk::symbol_short!("club_new"),),
            (club_id, organizer),
        );

        club_id
    }

    /// Add a new member to the club
    pub fn add_member(env: Env, club_id: u64, member: Address) {
        member.require_auth();

        let mut club: Club = env
            .storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found");

        // Check if member already exists
        for existing_member in club.members.iter() {
            if existing_member == member {
                panic!("Member already exists in club");
            }
        }

        club.members.push_back(member.clone());
        env.storage().persistent().set(&DataKey::Club(club_id), &club);

        // Emit event
        env.events().publish(
            (soroban_sdk::symbol_short!("mem_add"),),
            (club_id, member),
        );
    }

    /// Remove um membro do clube (apenas organizador)
    pub fn remove_member(env: Env, club_id: u64, organizer: Address, member: Address) {
        organizer.require_auth();

        let mut club: Club = env
            .storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found");

        if club.organizer != organizer {
            panic!("Only organizer can remove members");
        }

        // Encontrar e remover o membro
        let mut new_members = Vec::new(&env);
        let mut member_found = false;

        for existing_member in club.members.iter() {
            if existing_member != member {
                new_members.push_back(existing_member);
            } else {
                member_found = true;
            }
        }

        if !member_found {
            panic!("Member not found in club");
        }

        club.members = new_members;
        env.storage().persistent().set(&DataKey::Club(club_id), &club);
    }

    /// Remove a club (only organizer)
    pub fn remove_club(env: Env, club_id: u64, organizer: Address) {
        organizer.require_auth();

        let club: Club = env
            .storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found");

        if club.organizer != organizer {
            panic!("Only organizer can remove club");
        }

        if club.usdc_deposited > 0 {
            panic!("Cannot remove club with deposited USDC");
        }

        // Remove club from storage
        env.storage().persistent().remove(&DataKey::Club(club_id));

        // Emit event
        env.events().publish(
            (soroban_sdk::symbol_short!("club_del"),),
            (club_id, organizer),
        );
    }
}