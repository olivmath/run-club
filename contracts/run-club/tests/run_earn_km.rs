//! Teste TC-002: Membro de Clube Ativo Corre e Recebe Tokens KM
//!
//! Objetivo: Verificar se um membro de um clube ativo ganha corretamente tokens KM ao registrar uma corrida.
//!
//! Pré-condições:
//! • "Clube Alpha" está ativo com 100 USDC depositados (conforme TC-001)
//! • Usuário B é membro do "Clube Alpha" e possui 0 KM tokens
//! • Usuário B tem seu Apple Watch configurado e conectado ao aplicativo
//!
//! Cenário:
//! 1. Usuário B inicia uma corrida usando o Apple Watch
//! 2. Usuário B completa uma corrida de 5 km
//! 3. Usuário B finaliza e sincroniza a corrida com o aplicativo no iPhone
//!
//! Resultados Esperados:
//! • A corrida de 5 km é registrada com sucesso no histórico de atividades do Usuário B
//! • A carteira digital do Usuário B é atualizada para mostrar 5 KM tokens (baseado na proporção 1 KM = 1 KM Token)
//! • O ranking do "Clube Alpha" é atualizado, refletindo os 5 KM tokens do Usuário B

#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use run_club::{RunClubContract, RunClubContractClient, WithdrawalRule};

#[test]
fn test_tc002_member_runs_and_earns_km_tokens() {
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

    // Usuário B (Membro que vai correr)
    let member_b = Address::generate(&env);

    // === PRÉ-CONDIÇÃO: CRIAR E ATIVAR O "CLUBE ALPHA" (baseado em TC-001) ===
    let club_name = String::from_str(&env, "Clube Alpha");
    let usdc_per_km = 1i128; // 1 USDC por KM (proporção 1 KM = 1 KM Token)
    let withdrawal_rule = WithdrawalRule::Equal;
    let duration_days = 30u32;

    // Criar o clube
    let club_id = client.create_club(
        &organizer,
        &club_name,
        &usdc_per_km,
        &withdrawal_rule,
        &duration_days,
    );

    // Ativar o clube
    client.activate(&club_id, &organizer);

    // Depositar 100 USDC no clube
    let usdc_amount = 100i128;
    client.deposit_usdc(&club_id, &organizer, &usdc_amount);

    // Adicionar Usuário B como membro do clube
    client.add_member(&club_id, &member_b);

    // === VERIFICAÇÃO DAS PRÉ-CONDIÇÕES ===
    // Verificar que o clube está ativo com 100 USDC depositados
    let club = client.get_club(&club_id);
    assert_eq!(club.is_active, true);
    assert_eq!(club.usdc_deposited, 100i128);
    assert_eq!(club.name, club_name);

    // Verificar que Usuário B é membro do clube
    let members = client.get_members(&club_id);
    assert!(members.contains(&member_b));

    // Verificar que Usuário B possui 0 KM tokens inicialmente
    let initial_km_tokens = client.get_user_km_tokens(&member_b, &club_id);
    assert_eq!(initial_km_tokens, 0i128);

    // === PASSO 1-3: USUÁRIO B CORRE E REGISTRA 5 KM ===
    // Simular que o Usuário B:
    // 1. Inicia uma corrida usando o Apple Watch
    // 2. Completa uma corrida de 5 km
    // 3. Finaliza e sincroniza a corrida com o aplicativo no iPhone
    //
    // No contexto do smart contract, isso resulta na adição de 5 KM tokens
    let km_run = 5i128; // 5 km corridos
    client.add_km_tokens(&club_id, &member_b, &km_run);

    // === VERIFICAÇÃO DOS RESULTADOS ESPERADOS ===

    // 1. A corrida de 5 km é registrada com sucesso (verificado pela adição de tokens)
    // 2. A carteira digital do Usuário B é atualizada para mostrar 5 KM tokens
    let final_km_tokens = client.get_user_km_tokens(&member_b, &club_id);
    assert_eq!(final_km_tokens, 5i128);

    // Verificar que a proporção está correta (1 KM = 1 KM Token)
    assert_eq!(final_km_tokens, km_run);

    // 3. O ranking do "Clube Alpha" é atualizado, refletindo os 5 KM tokens do Usuário B
    // (O ranking seria implementado em funcionalidades futuras, mas podemos verificar
    // que os tokens foram corretamente atribuídos ao usuário no clube)
    
    // Verificar que o clube ainda está ativo e os dados estão corretos
    let updated_club = client.get_club(&club_id);
    assert_eq!(updated_club.is_active, true);
    assert_eq!(updated_club.usdc_deposited, 100i128);
    assert_eq!(updated_club.usdc_per_km, 1i128);

    // === PÓS-CONDIÇÕES ===
    // Usuário B possui 5 KM tokens associados ao "Clube Alpha"
    let post_condition_tokens = client.get_user_km_tokens(&member_b, &club_id);
    assert_eq!(post_condition_tokens, 5i128);

    // Verificar que o usuário ainda é membro do clube
    let final_members = client.get_members(&club_id);
    assert!(final_members.contains(&member_b));
    assert_eq!(final_members.len(), 1); // Apenas o Usuário B é membro
}

#[test]
fn test_tc002_multiple_runs_accumulate_km_tokens() {
    // Teste adicional: verificar que múltiplas corridas acumulam tokens KM
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RunClubContract, ());
    let client = RunClubContractClient::new(&env, &contract_id);
    client.initialize();

    let organizer = Address::generate(&env);
    let member_b = Address::generate(&env);

    // Criar e configurar clube
    let club_name = String::from_str(&env, "Clube Alpha");
    let club_id = client.create_club(
        &organizer,
        &club_name,
        &1i128,
        &WithdrawalRule::Equal,
        &30u32,
    );

    client.activate(&club_id, &organizer);
    client.deposit_usdc(&club_id, &organizer, &100i128);
    client.add_member(&club_id, &member_b);

    // Primeira corrida: 3 km
    client.add_km_tokens(&club_id, &member_b, &3i128);
    assert_eq!(client.get_user_km_tokens(&member_b, &club_id), 3i128);

    // Segunda corrida: 2 km (total deve ser 5 km)
    client.add_km_tokens(&club_id, &member_b, &2i128);
    assert_eq!(client.get_user_km_tokens(&member_b, &club_id), 5i128);

    // Terceira corrida: 4 km (total deve ser 9 km)
    client.add_km_tokens(&club_id, &member_b, &4i128);
    assert_eq!(client.get_user_km_tokens(&member_b, &club_id), 9i128);
}

#[test]
#[should_panic(expected = "User is not a member of this club")]
fn test_tc002_non_member_cannot_earn_km_tokens() {
    // Teste adicional: verificar que não-membros não podem ganhar tokens KM
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RunClubContract, ());
    let client = RunClubContractClient::new(&env, &contract_id);
    client.initialize();

    let organizer = Address::generate(&env);
    let non_member = Address::generate(&env);

    // Criar e ativar clube
    let club_name = String::from_str(&env, "Clube Alpha");
    let club_id = client.create_club(
        &organizer,
        &club_name,
        &1i128,
        &WithdrawalRule::Equal,
        &30u32,
    );

    client.activate(&club_id, &organizer);
    client.deposit_usdc(&club_id, &organizer, &100i128);

    // Tentar adicionar tokens KM para um não-membro deve falhar
    client.add_km_tokens(&club_id, &non_member, &5i128);
}