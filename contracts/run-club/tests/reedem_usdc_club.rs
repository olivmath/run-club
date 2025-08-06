//! Teste TC-005: Resgate de Tokens KM por USDC no Final do Período do Clube
//!
//! Objetivo: Verificar o processo de resgate de tokens KM por USDC ao final do período de um clube ativo, considerando diferentes regras de distribuição.
//!
//! Pré-condições:
//! • "Clube Alpha" está ativo com 100 USDC depositados e regra de "Distribuição Igual"
//! • Usuário B possui 5 KM tokens associados ao "Clube Alpha"
//! • Usuário C possui 0 KM tokens associados ao "Clube Alpha"
//! • O período de competição do "Clube Alpha" chegou ao fim
//!
//! Cenário:
//! 1. Usuário B acessa a seção de carteira no aplicativo
//! 2. Usuário B seleciona o "Clube Alpha" para resgate
//! 3. Usuário B solicita o resgate de seus KM tokens por USDC
//! 4. Usuário B fornece um endereço de destino para o USDC
//!
//! Resultados Esperados:
//! • O sistema calcula a recompensa de USDC para o Usuário B com base na regra de "Distribuição Igual"
//! • Os 5 KM tokens do Usuário B são queimados ou marcados como resgatados
//! • O valor de USDC calculado é transferido para o endereço de destino fornecido pelo Usuário B
//! • O status do "Clube Alpha" é atualizado para indicar que o período de resgate foi concluído

#![cfg(test)]

use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env, String};

use run_club::{RunClubContract, RunClubContractClient, WithdrawalRule};

#[test]
fn test_tc005_redeem_km_tokens_for_usdc_equal_distribution() {
    let env = Env::default();
    env.mock_all_auths();

    // Registrar o contrato
    let contract_id = env.register(RunClubContract, ());
    let client = RunClubContractClient::new(&env, &contract_id);

    // Inicializar o contrato
    client.initialize();

    // === CONFIGURAÇÃO DOS USUÁRIOS ===
    let organizer = Address::generate(&env);
    let member_b = Address::generate(&env);
    let member_c = Address::generate(&env);
    let destination_address = Address::generate(&env);

    // === PRÉ-CONDIÇÃO: CRIAR E CONFIGURAR O "CLUBE ALPHA" ===
    let club_name = String::from_str(&env, "Clube Alpha");
    let usdc_per_km = 1i128;
    let withdrawal_rule = WithdrawalRule::Equal; // Regra de "Distribuição Igual"
    let duration_days = 30u32;

    // Criar o clube
    let club_id = client.create_club(
        &organizer,
        &club_name,
        &usdc_per_km,
        &withdrawal_rule,
        &duration_days,
    );

    // Ativar o clube e depositar 100 USDC
    client.activate(&club_id, &organizer);
    let usdc_amount = 100i128;
    client.deposit_usdc(&club_id, &organizer, &usdc_amount);

    // Adicionar membros ao clube
    client.add_member(&club_id, &member_b);
    client.add_member(&club_id, &member_c);

    // Usuário B ganha 5 KM tokens
    client.add_km_tokens(&club_id, &member_b, &5i128);
    
    // Usuário C não ganha tokens KM (permanece com 0)

    // === VERIFICAÇÃO DAS PRÉ-CONDIÇÕES ===
    let club = client.get_club(&club_id);
    assert_eq!(club.is_active, true);
    assert_eq!(club.usdc_deposited, 100i128);
    assert_eq!(club.withdrawal_rule, WithdrawalRule::Equal);

    let member_b_km = client.get_user_km_tokens(&member_b, &club_id);
    let member_c_km = client.get_user_km_tokens(&member_c, &club_id);
    assert_eq!(member_b_km, 5i128);
    assert_eq!(member_c_km, 0i128);

    // Verificar que o período ainda não terminou
    assert_eq!(client.is_club_period_ended(&club_id), false);

    // === SIMULAR O FIM DO PERÍODO DO CLUBE ===
    // Avançar o tempo para simular o fim do período
    env.ledger().with_mut(|li| {
        li.timestamp = club.month_end_timestamp + 1;
    });

    // Verificar que o período terminou
    assert_eq!(client.is_club_period_ended(&club_id), true);

    // === PASSO 1-4: USUÁRIO B RESGATA SEUS KM TOKENS ===
    
    // Verificar informações de resgate antes do resgate
    let (km_tokens, usdc_reward, period_ended) = client.get_redemption_info(&club_id, &member_b);
    assert_eq!(km_tokens, 5i128);
    assert_eq!(period_ended, true);
    
    // Com regra "Equal" e apenas Usuário B tendo tokens, ele deve receber todo o USDC
    assert_eq!(usdc_reward, 100i128);

    // Executar o resgate
    let redeemed_amount = client.redeem_usdc(&club_id, &member_b, &destination_address);

    // === VERIFICAÇÃO DOS RESULTADOS ESPERADOS ===

    // 1. O sistema calculou corretamente a recompensa USDC (100 USDC para Usuário B)
    assert_eq!(redeemed_amount, 100i128);

    // 2. Os 5 KM tokens do Usuário B foram queimados/zerados
    let member_b_km_after = client.get_user_km_tokens(&member_b, &club_id);
    assert_eq!(member_b_km_after, 0i128);

    // 3. O pool de USDC do clube foi reduzido
    let club_after = client.get_club(&club_id);
    assert_eq!(club_after.usdc_deposited, 0i128); // Todo o USDC foi resgatado

    // === PÓS-CONDIÇÕES ===
    // Usuário B recebeu o USDC correspondente (verificado pelo valor retornado)
    // Os KM tokens do Usuário B foram zerados (verificado acima)
    // O pool de USDC do clube foi esvaziado (verificado acima)

    // Verificar que Usuário C ainda tem 0 tokens e não pode resgatar
    let member_c_km_after = client.get_user_km_tokens(&member_c, &club_id);
    assert_eq!(member_c_km_after, 0i128);
}

#[test]
fn test_tc005_redeem_with_multiple_members_equal_distribution() {
    // Teste adicional: múltiplos membros com tokens KM e distribuição igual
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RunClubContract, ());
    let client = RunClubContractClient::new(&env, &contract_id);
    client.initialize();

    let organizer = Address::generate(&env);
    let member_b = Address::generate(&env);
    let member_c = Address::generate(&env);
    let destination_b = Address::generate(&env);
    let destination_c = Address::generate(&env);

    // Criar clube com distribuição igual
    let club_name = String::from_str(&env, "Clube Beta");
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
    client.add_member(&club_id, &member_c);

    // Ambos os membros ganham tokens KM (quantidades diferentes)
    client.add_km_tokens(&club_id, &member_b, &5i128);
    client.add_km_tokens(&club_id, &member_c, &3i128);

    // Simular fim do período
    let club = client.get_club(&club_id);
    env.ledger().with_mut(|li| {
        li.timestamp = club.month_end_timestamp + 1;
    });

    // Com distribuição igual, cada membro deve receber 50 USDC (100/2)
    let (_, reward_b, _) = client.get_redemption_info(&club_id, &member_b);
    let (_, reward_c, _) = client.get_redemption_info(&club_id, &member_c);
    assert_eq!(reward_b, 50i128);
    assert_eq!(reward_c, 50i128);

    // Resgatar para ambos os membros
    let redeemed_b = client.redeem_usdc(&club_id, &member_b, &destination_b);
    let redeemed_c = client.redeem_usdc(&club_id, &member_c, &destination_c);

    assert_eq!(redeemed_b, 50i128);
    assert_eq!(redeemed_c, 50i128);

    // Verificar que os tokens foram zerados
    assert_eq!(client.get_user_km_tokens(&member_b, &club_id), 0i128);
    assert_eq!(client.get_user_km_tokens(&member_c, &club_id), 0i128);

    // Verificar que o pool foi esvaziado
    let final_club = client.get_club(&club_id);
    assert_eq!(final_club.usdc_deposited, 0i128);
}

#[test]
fn test_tc005_redeem_with_unlimited_distribution() {
    // Teste adicional: distribuição proporcional (Unlimited)
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RunClubContract, ());
    let client = RunClubContractClient::new(&env, &contract_id);
    client.initialize();

    let organizer = Address::generate(&env);
    let member_b = Address::generate(&env);
    let member_c = Address::generate(&env);
    let destination_b = Address::generate(&env);

    // Criar clube com distribuição proporcional
    let club_name = String::from_str(&env, "Clube Gamma");
    let club_id = client.create_club(
        &organizer,
        &club_name,
        &1i128,
        &WithdrawalRule::Unlimited,
        &30u32,
    );

    client.activate(&club_id, &organizer);
    client.deposit_usdc(&club_id, &organizer, &100i128);
    client.add_member(&club_id, &member_b);
    client.add_member(&club_id, &member_c);

    // Member B: 8 KM, Member C: 2 KM (total: 10 KM)
    client.add_km_tokens(&club_id, &member_b, &8i128);
    client.add_km_tokens(&club_id, &member_c, &2i128);

    // Simular fim do período
    let club = client.get_club(&club_id);
    env.ledger().with_mut(|li| {
        li.timestamp = club.month_end_timestamp + 1;
    });

    // Com distribuição proporcional:
    // Member B: (8/10) * 100 = 80 USDC
    // Member C: (2/10) * 100 = 20 USDC
    let (_, reward_b, _) = client.get_redemption_info(&club_id, &member_b);
    let (_, reward_c, _) = client.get_redemption_info(&club_id, &member_c);
    assert_eq!(reward_b, 80i128);
    assert_eq!(reward_c, 20i128);

    // Resgatar para Member B
    let redeemed_b = client.redeem_usdc(&club_id, &member_b, &destination_b);
    assert_eq!(redeemed_b, 80i128);

    // Verificar estado após resgate
    assert_eq!(client.get_user_km_tokens(&member_b, &club_id), 0i128);
    let club_after = client.get_club(&club_id);
    assert_eq!(club_after.usdc_deposited, 20i128); // Restam 20 USDC para Member C
}

#[test]
#[should_panic(expected = "Club period has not ended yet")]
fn test_tc005_cannot_redeem_before_period_ends() {
    // Teste adicional: não pode resgatar antes do período terminar
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RunClubContract, ());
    let client = RunClubContractClient::new(&env, &contract_id);
    client.initialize();

    let organizer = Address::generate(&env);
    let member_b = Address::generate(&env);
    let destination = Address::generate(&env);

    let club_name = String::from_str(&env, "Clube Delta");
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
    client.add_km_tokens(&club_id, &member_b, &5i128);

    // Tentar resgatar antes do período terminar deve falhar
    client.redeem_usdc(&club_id, &member_b, &destination);
}

#[test]
#[should_panic(expected = "User has no KM tokens to redeem")]
fn test_tc005_cannot_redeem_without_km_tokens() {
    // Teste adicional: não pode resgatar sem tokens KM
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(RunClubContract, ());
    let client = RunClubContractClient::new(&env, &contract_id);
    client.initialize();

    let organizer = Address::generate(&env);
    let member_b = Address::generate(&env);
    let destination = Address::generate(&env);

    let club_name = String::from_str(&env, "Clube Epsilon");
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
    // Não adicionar tokens KM para o membro

    // Simular fim do período
    let club = client.get_club(&club_id);
    env.ledger().with_mut(|li| {
        li.timestamp = club.month_end_timestamp + 1;
    });

    // Tentar resgatar sem tokens KM deve falhar
    client.redeem_usdc(&club_id, &member_b, &destination);
}

