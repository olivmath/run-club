//! Teste TC-001: Criação, Ativação e Adição de Membros a um Clube
//!
//! Objetivo: Verificar a funcionalidade de criação de um novo clube, sua ativação e a adição de membros.
//!
//! Cenário:
//! 1. Usuário A (Organizador) cria o "Clube Alpha"
//! 2. Usuário A ativa o clube
//! 3. Usuário B e Usuário C são convidados e aceitam o convite
//! 4. Verificar que o clube está ativo e os membros foram adicionados corretamente

#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use run_club::{RunClubContract, RunClubContractClient, WithdrawalRule};

#[test]
fn test_tc001_create_activate_and_add_members_to_club() {
    let env = Env::default();
    env.mock_all_auths();

    // Registrar o contrato
    let contract_id = env.register(RunClubContract, ());
    let client = RunClubContractClient::new(&env, &contract_id);

    // Inicializar o contrato
    client.initialize();

    // === CONFIGURAÇÃO DOS USUÁRIOS ===
    // Usuário A (Organizador)
    let organizer = Address::generate(&env);

    // Usuário B e Usuário C (Membros)
    let member_b = Address::generate(&env);
    let member_c = Address::generate(&env);

    // === PASSO 1-3: CRIAÇÃO DO CLUBE ===
    // Usuário A navega para a seção de criação de clubes e preenche os detalhes:
    // Nome: "Clube Alpha"
    // Proporção KM/Token: 1 KM = 1 KM Token (usdc_per_km = 1)
    // Regra de Resgate: "Distribuição Igual"
    let club_name = String::from_str(&env, "Clube Alpha");
    let usdc_per_km = 1i128; // 1 USDC por KM
    let withdrawal_rule = WithdrawalRule::Equal;
    let duration_days = 30u32; // 30 dias de duração

    // Criar o clube
    let club_id = client.create_club(
        &organizer,
        &club_name,
        &usdc_per_km,
        &withdrawal_rule,
        &duration_days,
    );

    // Verificar que o clube foi criado com sucesso
    assert_eq!(club_id, 1u64);

    // Obter informações do clube criado
    let club = client.get_club(&club_id);
    assert_eq!(club.id, 1u64);
    assert_eq!(club.name, club_name);
    assert_eq!(club.organizer, organizer);
    assert_eq!(club.usdc_deposited, 0i128); // Ainda não foi depositado
    assert_eq!(club.usdc_per_km, usdc_per_km);
    assert_eq!(club.withdrawal_rule, withdrawal_rule);
    assert_eq!(club.is_active, false); // Ainda não está ativo
    assert_eq!(club.members.len(), 0); // Ainda não tem membros

    // === PASSO 4: ATIVAÇÃO DO CLUBE ===
    // Usuário A ativa o clube
    client.activate(&club_id, &organizer);

    // Verificar que o clube foi ativado
    let club_after_activation = client.get_club(&club_id);
    assert_eq!(club_after_activation.is_active, true); // Clube agora está ativo

    // Verificar que o clube aparece na lista de clubes ativos
    let active_clubs = client.get_active_clubs();
    assert_eq!(active_clubs.len(), 1);
    assert_eq!(active_clubs.get(0).unwrap(), club_id);

    // === PASSO 5-7: CONVITE E ADIÇÃO DE MEMBROS ===
    // Usuário A convida Usuário B e Usuário C para o "Clube Alpha"
    // Usuário B aceita o convite
    client.add_member(&club_id, &member_b);

    // Usuário C aceita o convite
    client.add_member(&club_id, &member_c);

    // === VERIFICAÇÃO DOS RESULTADOS ESPERADOS ===

    // 1. O "Clube Alpha" foi criado com sucesso e aparece na lista de clubes ativos
    let final_club = client.get_club(&club_id);
    assert_eq!(final_club.name, club_name);
    assert_eq!(final_club.is_active, true);

    let active_clubs_final = client.get_active_clubs();
    assert!(active_clubs_final.contains(club_id));

    // 3. Usuário B e Usuário C aparecem como membros do "Clube Alpha"
    let members = client.get_members(&club_id);
    assert_eq!(members.len(), 2);
    assert!(members.contains(&member_b));
    assert!(members.contains(&member_c));
}
