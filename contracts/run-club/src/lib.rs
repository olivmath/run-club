#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Vec,
};

// mod club_manage;


/// Regras de resgate para os clubes
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WithdrawalRule {
    /// Distribuição igual entre todos os membros
    Equal,
    /// Resgate ilimitado baseado em tokens KM
    Unlimited,
}

/// Estrutura que representa um clube de corrida
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
}

/// Chaves para armazenamento de dados
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Club(u64),
    ClubCounter,
    UserKmTokens(Address, u64), // (user, club_id)
    UserClubs(Address),
}

/// Contrato principal do Run Club
#[contract]
pub struct RunClubContract;

#[contractimpl]
impl RunClubContract {
    /// Inicializa o contrato
    pub fn initialize(env: Env) {
        env.storage().persistent().set(&DataKey::ClubCounter, &0u64);
    }

    /// Cria um novo clube de corrida
    pub fn create_club(
        env: Env,
        organizer: Address,
        name: String,
        usdc_per_km: i128,
        withdrawal_rule: WithdrawalRule,
        duration_days: u32,    ) -> u64 {
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
            (symbol_short!("club_new"),),
            (club_id, organizer),
        );

        club_id
    }

    /// Obtém informações de um clube
    pub fn get_club(env: Env, club_id: u64) -> Club {
        env.storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found")
    }

    /// Ativa um clube (apenas organizador)
    pub fn activate(env: Env, club_id: u64, organizer: Address) {
        organizer.require_auth();
        
        let mut club: Club = env
            .storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found");

        if club.organizer != organizer {
            panic!("Only organizer can activate club");
        }

        if club.is_active {
            panic!("Club is already active");
        }

        club.is_active = true;
        env.storage().persistent().set(&DataKey::Club(club_id), &club);

        // Emitir evento
        env.events().publish(
            (symbol_short!("club_act"),),
            (club_id, organizer),
        );
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

    /// Adiciona um membro ao clube
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
            (symbol_short!("mem_add"),),
            (club_id, member),
        );
    }

    /// Obtém todos os membros de um clube
    pub fn get_members(env: Env, club_id: u64) -> Vec<Address> {
        let club = Self::get_club(env.clone(), club_id);
        club.members
    }

    /// Deposita USDC no clube e o ativa
    pub fn deposit_usdc(env: Env, club_id: u64, organizer: Address, amount: i128) {
        organizer.require_auth();
        
        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let mut club: Club = env
            .storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found");

        if club.organizer != organizer {
            panic!("Only organizer can deposit USDC");
        }

        club.usdc_deposited += amount;
        club.is_active = true; // Ativa o clube quando USDC é depositado

        env.storage().persistent().set(&DataKey::Club(club_id), &club);

        // Emitir evento
        env.events().publish(
            (symbol_short!("usdc_dep"),),
            (club_id, organizer, amount),
        );
    }

    /// Adiciona tokens KM para um usuário em um clube específico
    pub fn add_km_tokens(env: Env, club_id: u64, user: Address, km_amount: i128) {
        // Verificar se o clube existe e está ativo
        let club: Club = env
            .storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found");

        if !club.is_active {
            panic!("Club is not active");
        }

        // Verificar se o usuário é membro do clube
        let mut is_member = false;
        for member in club.members.iter() {
            if member == user {
                is_member = true;
                break;
            }
        }

        if !is_member {
            panic!("User is not a member of this club");
        }

        // Adicionar tokens KM
        let current_tokens = env
            .storage()
            .persistent()
            .get(&DataKey::UserKmTokens(user.clone(), club_id))
            .unwrap_or(0i128);

        let new_tokens = current_tokens + km_amount;
        env.storage()
            .persistent()
            .set(&DataKey::UserKmTokens(user.clone(), club_id), &new_tokens);

        // Emitir evento
        env.events().publish(
            (symbol_short!("km_added"),),
            (club_id, user, km_amount),
        );
    }

    /// Obtém a quantidade de tokens KM de um usuário em um clube
    pub fn get_user_km_tokens(env: Env, user: Address, club_id: u64) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::UserKmTokens(user, club_id))
            .unwrap_or(0i128)
    }

    /// Verifica se o período do clube terminou
    pub fn is_club_period_ended(env: Env, club_id: u64) -> bool {
        let club: Club = env
            .storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found");

        let current_timestamp = env.ledger().timestamp();
        current_timestamp >= club.month_end_timestamp
    }

    /// Calcula o total de tokens KM de todos os membros do clube
    pub fn get_total_km_tokens(env: Env, club_id: u64) -> i128 {
        let club: Club = env
            .storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found");

        let mut total_km = 0i128;
        for member in club.members.iter() {
            let member_km = env
                .storage()
                .persistent()
                .get(&DataKey::UserKmTokens(member.clone(), club_id))
                .unwrap_or(0i128);
            total_km += member_km;
        }
        total_km
    }

    /// Calcula a recompensa USDC para um usuário baseado na regra de distribuição
    pub fn calculate_usdc_reward(env: Env, club_id: u64, user: Address) -> i128 {
        let club: Club = env
            .storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found");

        if !Self::is_club_period_ended(env.clone(), club_id) {
            panic!("Club period has not ended yet");
        }

        let user_km_tokens = Self::get_user_km_tokens(env.clone(), user.clone(), club_id);
        
        if user_km_tokens == 0 {
            return 0i128;
        }

        match club.withdrawal_rule {
            WithdrawalRule::Equal => {
                // Distribuição igual: divide o USDC total igualmente entre membros com KM tokens
                let total_km = Self::get_total_km_tokens(env.clone(), club_id);
                if total_km == 0 {
                    return 0i128;
                }
                
                // Contar quantos membros têm KM tokens
                let mut members_with_tokens = 0u32;
                for member in club.members.iter() {
                    let member_km = Self::get_user_km_tokens(env.clone(), member.clone(), club_id);
                    if member_km > 0 {
                        members_with_tokens += 1;
                    }
                }
                
                if members_with_tokens == 0 {
                    return 0i128;
                }
                
                club.usdc_deposited / (members_with_tokens as i128)
            },
            WithdrawalRule::Unlimited => {
                // Distribuição proporcional aos KM tokens
                let total_km = Self::get_total_km_tokens(env.clone(), club_id);
                if total_km == 0 {
                    return 0i128;
                }
                
                (club.usdc_deposited * user_km_tokens) / total_km
            }
        }
    }

    /// Executa o resgate de tokens KM por USDC
    pub fn redeem_usdc(env: Env, club_id: u64, user: Address, destination: Address) -> i128 {
        user.require_auth();
        
        let mut club: Club = env
            .storage()
            .persistent()
            .get(&DataKey::Club(club_id))
            .expect("Club not found");

        if !club.is_active {
            panic!("Club is not active");
        }

        if !Self::is_club_period_ended(env.clone(), club_id) {
            panic!("Club period has not ended yet");
        }

        // Verificar se o usuário é membro do clube
        let mut is_member = false;
        for member in club.members.iter() {
            if member == user {
                is_member = true;
                break;
            }
        }

        if !is_member {
            panic!("User is not a member of this club");
        }

        let user_km_tokens = Self::get_user_km_tokens(env.clone(), user.clone(), club_id);
        
        if user_km_tokens == 0 {
            panic!("User has no KM tokens to redeem");
        }

        // Calcular recompensa USDC
        let usdc_reward = Self::calculate_usdc_reward(env.clone(), club_id, user.clone());
        
        if usdc_reward > club.usdc_deposited {
            panic!("Insufficient USDC in club");
        }

        // Queimar/zerar os tokens KM do usuário
        env.storage()
            .persistent()
            .set(&DataKey::UserKmTokens(user.clone(), club_id), &0i128);

        // Reduzir o USDC do clube
        club.usdc_deposited -= usdc_reward;
        env.storage().persistent().set(&DataKey::Club(club_id), &club);

        // Emitir evento de resgate
        env.events().publish(
            (symbol_short!("usdc_red"),),
            (club_id, user, destination, usdc_reward),
        );

        usdc_reward
    }

    /// Obtém informações de resgate para um usuário
    pub fn get_redemption_info(env: Env, club_id: u64, user: Address) -> (i128, i128, bool) {
        let user_km_tokens = Self::get_user_km_tokens(env.clone(), user.clone(), club_id);
        let usdc_reward = if Self::is_club_period_ended(env.clone(), club_id) {
            Self::calculate_usdc_reward(env.clone(), club_id, user)
        } else {
            0i128
        };
        let period_ended = Self::is_club_period_ended(env.clone(), club_id);
        
        (user_km_tokens, usdc_reward, period_ended)
    }
}