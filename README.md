# Stellar Run Club - Soroban Smart Contracts

This directory contains the Soroban smart contracts for the Stellar Run Club application.

## 🚀 Deployment Status

**Contract Successfully Deployed!**

- **Contract ID**: `CDIJN5LCNOVQZOBIRLAD32PAR2EHYMS7T7YSE6TWSK7RE4CMGT67CC3M`
- **Network**: Stellar Testnet
- **WASM Hash**: `507ae36b7ccf47daeed866cf776cc6ea62fcb3ef4469f80ca20d56981f2f336f`
- **Explorer**: [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CDIJN5LCNOVQZOBIRLAD32PAR2EHYMS7T7YSE6TWSK7RE4CMGT67CC3M)

## 📋 Contract Functions

The Run Club contract provides the following functionality:

### Core Functions

- `initialize()` - Initialize the contract
- `create_club()` - Create a new running club with USDC incentives
- `activate()` - Activate a club for participation
- `add_member()` - Add members to a club
- `deposit_usdc()` - Deposit USDC into club treasury

### Token Management

- `add_km_tokens()` - Convert tracked kilometers into KM tokens
- `get_user_km_tokens()` - Get user's KM token balance for a club
- `get_total_km_tokens()` - Get total KM tokens in a club

### Rewards & Withdrawals

- `calculate_usdc_reward()` - Calculate USDC rewards based on KM tokens
- `redeem_usdc()` - Withdraw USDC rewards to custodial wallet
- `get_redemption_info()` - Get redemption details for a user

### Query Functions

- `get_club()` - Get club information
- `get_active_clubs()` - List all active clubs
- `get_members()` - Get club members
- `is_club_period_ended()` - Check if club period has ended

## 🛠️ Development

### Building the Contract

```bash
# Navigate to the contract directory
cd contracts/run-club

# Build the contract
stellar contract build
```

### Deploying the Contract

```bash
# Deploy to testnet
./deploy.sh
```

### Testing the Contract

```bash
# Run basic functionality tests
./test_contract.sh
```

## 📁 Project Structure

```text
.
├── contracts/
│   └── run-club/
│       ├── src/
│       │   ├── lib.rs              # Main contract implementation
│       │   └── club_manage/        # Club management modules
│       ├── tests/                  # Contract tests
│       ├── Cargo.toml             # Contract dependencies
│       └── Makefile               # Build scripts
├── deploy.sh                      # Deployment script
├── test_contract.sh              # Testing script
├── Cargo.toml                    # Workspace configuration
└── README.md                     # This file
```

## 🏃‍♂️ Contract Features

### Club Management

- Create running clubs with customizable USDC incentives
- Set withdrawal rules (Equal distribution or Unlimited based on KM tokens)
- Manage club membership and activation

### Token Economy

- Convert tracked kilometers to KM tokens (1:1 ratio)
- USDC rewards distributed based on KM token holdings
- Support for both equal and performance-based reward distribution

### Security Features

- Organizer-only functions for club management
- Time-based club periods with automatic expiration
- Secure USDC withdrawal to custodial wallets
- Input validation and error handling

## 🔧 Requirements

- Rust 1.70+
- Stellar CLI
- Soroban SDK 22.0.0
- Configured Stellar testnet identity

## 📖 Usage Examples

See the `test_contract.sh` script for examples of:

- Initializing the contract
- Creating clubs
- Adding members
- Depositing USDC
- Tracking kilometers and earning KM tokens
- Calculating and redeeming USDC rewards

### Descrição do Projeto e Fluxo da Aplicação

O projeto é um aplicativo de rastreamento de corrida, similar ao Nike Run Club, disponível para iPhone e Apple Watch, que incentiva competições em grupos chamados "clubes". No aplicativo, os usuários criam clubes, depositam incentivos financeiros em USDC (uma moeda digital segura), competem correndo para ganhar tokens chamados KM (baseados em quilômetros corridos) e, ao final de cada mês, podem resgatar esses tokens por USDC. A experiência é simples e amigável, projetada para usuários comuns, sem mencionar termos técnicos como blockchain. Abaixo está o fluxo da aplicação, descrito em etapas, sem detalhes técnicos:

1. **Usuário Faz Login**:

   - O usuário acessa o aplicativo usando CLERK com uma conta de rede social (como Google, Facebook ou Apple).
   - Após o login, o backend cria automaticamente uma "carteira digital" para o usuário, que será usada para armazenar tokens do club. O usuário não precisa configurar nada; ele apenas vê um QR code único no aplicativo, que serve como identificador da carteira.

2. **Usuário Cria um Clube**:

   - No app, o usuário (organizador) acessa a seção de criação de clubes.
   - Ele escolhe um nome para o clube (que será o nome do token), define o valor total em USDC que será oferecido como incentivo, determina a proporção entre quilômetros corridos e tokens (por exemplo, 1 km = 1 KM token), e seleciona as regras de resgate: todos recebem uma parte igual do USDC depositado ou quem correr mais pode resgatar mais, desde que tenha tokens do club suficientes.
   - O clube é criado, e para ativa-lo o usuário precisa fazer o deposito em USDC.
   - Depois de ativar o clube ele pode adicionar outros usuários ao clube, convidando-os por meio de links ou códigos compartilhados.
   - Cada usuário convidado precisa fazer login no aplicativo e aceitar o convite.

3. **Quando o Usuário Deposita USDC no Clube**:

   - O organizador do clube deposita o valor em USDC que escolheu como incentivo, usando uma opção de pagamento segura no aplicativo (como transferência bancária com QRcode mostrando a carteira do backend).
   - O valor é armazenado em uma carteira segura gerenciada pelo backend, pronta para ser usada como prêmio no final do mês.

4. **USDC é Alocado no Soroswap**:

   - O valor depositado em USDC é automaticamente colocado em um sistema seguro de troca (Soroswap), que garante que o dinheiro esteja disponível para ser distribuído aos vencedores. Esse sistema mantém o USDC pronto para ser convertido ou transferido conforme as regras do clube e gerando retornos até ser sacado manualmente.

5. **Usuários Correm e Ganham Tokens do Clube**:

   - Os usuários do clube usam o iPhone ou o Apple Watch para rastrear suas corridas. O aplicativo registra a distância percorrida (em quilômetros) durante cada corrida.
   - Cada quilômetro corrido é convertido em tokens do clube, conforme a proporção definida pelo organizador (por exemplo, 1 km = 1 KM token). Esses tokens são automaticamente adicionados à carteira do usuário no aplicativo.
   - Os usuários também podem ver seu progresso no clube, acompanhando o número de KM tokens acumulados e o valor total em USDC depositado.

6. **Competição e Ranking**:

   - Durante o mês, o aplicativo exibe um ranking no iPhone, mostrando quantos tokens do clube cada membro do clube acumulou com base nas corridas.
   - Os usuários podem acompanhar sua posição no ranking e comparar seu progresso com outros membros do clube.

7. **Resgate de USDC no Final do Mês**:

   - Ao final do mês, os usuários podem resgatar seus tokens do clube por USDC, conforme as regras do clube:
     - **Distribuição igual**: O valor total em USDC é dividido igualmente entre todos os membros com tokens do clube.
     - **Distribuição ilimitada**: Usuários com mais tokens do clube podem resgatar uma parte maior do USDC, até o limite disponível.
   - Para resgatar, o usuário entra no club, vê seu saldo de tokens do clube e solicita o resgate. Ele fornece um endereço de destino (outra carteira digital) ou usa uma opção integrada no aplicativo para receber o valor.

8. **Saque para Carteira Externa**:

   - Na seção de configurações do aplicativo, o usuário pode optar por transferir seu USDC para uma carteira digital externa, informando um endereço seguro.
   - O aplicativo processa a transferência de forma segura, e o usuário recebe o valor sem precisar entender os detalhes técnicos.

9. **Visualização de QR Code**:

   - A qualquer momento, o usuário pode visualizar sua carteira no aplicativo por meio de um QR code único, que serve como identificador. Esse QR code pode ser usado para verificar a carteira ou compartilhar com o suporte, se necessário.

10. **Nova Competição**:
    - Após o término do mês, o organizador pode iniciar um novo ciclo, criando outro clube ou renovando o atual, repetindo o processo de depósito, corrida e resgate.

### Resumo

O fluxo do aplicativo é projetado para ser intuitivo: o usuário faz login, cria ou entra em um clube, corre usando o Apple Watch, ganha KM tokens, acompanha seu progresso no iPhone e resgata USDC no final do mês. Todo o processo é apresentado como uma experiência segura e simples, com foco na competição e nos incentivos, sem expor detalhes técnicos como blockchain ou Soroswap. O organizador tem controle sobre as regras do clube, enquanto os participantes desfrutam de uma experiência gamificada e recompensadora.

# STATUS WORK

- [x] Club manage
- [ ] Token
- [ ] Token Factory
- [ ] USDC manage
- [ ] Soroswap Connect
