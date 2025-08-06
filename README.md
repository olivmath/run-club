# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.


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