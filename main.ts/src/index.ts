import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}




/**
 * Regras de resgate para os clubes
 */
export type WithdrawalRule = {tag: "Equal", values: void} | {tag: "Unlimited", values: void};


/**
 * Estrutura que representa um clube de corrida
 */
export interface Club {
  id: u64;
  is_active: boolean;
  members: Array<string>;
  month_end_timestamp: u64;
  name: string;
  organizer: string;
  usdc_deposited: i128;
  usdc_per_km: i128;
  withdrawal_rule: WithdrawalRule;
}

/**
 * Chaves para armazenamento de dados
 */
export type DataKey = {tag: "Club", values: readonly [u64]} | {tag: "ClubCounter", values: void} | {tag: "UserKmTokens", values: readonly [string, u64]} | {tag: "UserClubs", values: readonly [string]};

export interface Client {
  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Inicializa o contrato
   */
  initialize: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a create_club transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Cria um novo clube de corrida
   */
  create_club: ({organizer, name, usdc_per_km, withdrawal_rule, duration_days}: {organizer: string, name: string, usdc_per_km: i128, withdrawal_rule: WithdrawalRule, duration_days: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a get_club transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Obtém informações de um clube
   */
  get_club: ({club_id}: {club_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Club>>

  /**
   * Construct and simulate a activate transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Ativa um clube (apenas organizador)
   */
  activate: ({club_id, organizer}: {club_id: u64, organizer: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a get_active_clubs transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Obtém lista de clubes ativos
   */
  get_active_clubs: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<u64>>>

  /**
   * Construct and simulate a add_member transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Adiciona um membro ao clube
   */
  add_member: ({club_id, member}: {club_id: u64, member: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a get_members transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Obtém todos os membros de um clube
   */
  get_members: ({club_id}: {club_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<string>>>

  /**
   * Construct and simulate a deposit_usdc transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Deposita USDC no clube e o ativa
   */
  deposit_usdc: ({club_id, organizer, amount}: {club_id: u64, organizer: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a add_km_tokens transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Adiciona tokens KM para um usuário em um clube específico
   */
  add_km_tokens: ({club_id, user, km_amount}: {club_id: u64, user: string, km_amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a get_user_km_tokens transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Obtém a quantidade de tokens KM de um usuário em um clube
   */
  get_user_km_tokens: ({user, club_id}: {user: string, club_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a is_club_period_ended transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Verifica se o período do clube terminou
   */
  is_club_period_ended: ({club_id}: {club_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a get_total_km_tokens transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Calcula o total de tokens KM de todos os membros do clube
   */
  get_total_km_tokens: ({club_id}: {club_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a calculate_usdc_reward transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Calcula a recompensa USDC para um usuário baseado na regra de distribuição
   */
  calculate_usdc_reward: ({club_id, user}: {club_id: u64, user: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a redeem_usdc transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Executa o resgate de tokens KM por USDC
   */
  redeem_usdc: ({club_id, user, destination}: {club_id: u64, user: string, destination: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a get_redemption_info transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Obtém informações de resgate para um usuário
   */
  get_redemption_info: ({club_id, user}: {club_id: u64, user: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<readonly [i128, i128, boolean]>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy(null, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAgAAACBSZWdyYXMgZGUgcmVzZ2F0ZSBwYXJhIG9zIGNsdWJlcwAAAAAAAAAOV2l0aGRyYXdhbFJ1bGUAAAAAAAIAAAAAAAAAK0Rpc3RyaWJ1acOnw6NvIGlndWFsIGVudHJlIHRvZG9zIG9zIG1lbWJyb3MAAAAABUVxdWFsAAAAAAAAAAAAACZSZXNnYXRlIGlsaW1pdGFkbyBiYXNlYWRvIGVtIHRva2VucyBLTQAAAAAACVVubGltaXRlZAAAAA==",
        "AAAAAQAAACxFc3RydXR1cmEgcXVlIHJlcHJlc2VudGEgdW0gY2x1YmUgZGUgY29ycmlkYQAAAAAAAAAEQ2x1YgAAAAkAAAAAAAAAAmlkAAAAAAAGAAAAAAAAAAlpc19hY3RpdmUAAAAAAAABAAAAAAAAAAdtZW1iZXJzAAAAA+oAAAATAAAAAAAAABNtb250aF9lbmRfdGltZXN0YW1wAAAAAAYAAAAAAAAABG5hbWUAAAAQAAAAAAAAAAlvcmdhbml6ZXIAAAAAAAATAAAAAAAAAA51c2RjX2RlcG9zaXRlZAAAAAAACwAAAAAAAAALdXNkY19wZXJfa20AAAAACwAAAAAAAAAPd2l0aGRyYXdhbF9ydWxlAAAAB9AAAAAOV2l0aGRyYXdhbFJ1bGUAAA==",
        "AAAAAgAAACJDaGF2ZXMgcGFyYSBhcm1hemVuYW1lbnRvIGRlIGRhZG9zAAAAAAAAAAAAB0RhdGFLZXkAAAAABAAAAAEAAAAAAAAABENsdWIAAAABAAAABgAAAAAAAAAAAAAAC0NsdWJDb3VudGVyAAAAAAEAAAAAAAAADFVzZXJLbVRva2VucwAAAAIAAAATAAAABgAAAAEAAAAAAAAACVVzZXJDbHVicwAAAAAAAAEAAAAT",
        "AAAAAAAAABVJbmljaWFsaXphIG8gY29udHJhdG8AAAAAAAAKaW5pdGlhbGl6ZQAAAAAAAAAAAAA=",
        "AAAAAAAAAB1DcmlhIHVtIG5vdm8gY2x1YmUgZGUgY29ycmlkYQAAAAAAAAtjcmVhdGVfY2x1YgAAAAAFAAAAAAAAAAlvcmdhbml6ZXIAAAAAAAATAAAAAAAAAARuYW1lAAAAEAAAAAAAAAALdXNkY19wZXJfa20AAAAACwAAAAAAAAAPd2l0aGRyYXdhbF9ydWxlAAAAB9AAAAAOV2l0aGRyYXdhbFJ1bGUAAAAAAAAAAAANZHVyYXRpb25fZGF5cwAAAAAAAAQAAAABAAAABg==",
        "AAAAAAAAACBPYnTDqW0gaW5mb3JtYcOnw7VlcyBkZSB1bSBjbHViZQAAAAhnZXRfY2x1YgAAAAEAAAAAAAAAB2NsdWJfaWQAAAAABgAAAAEAAAfQAAAABENsdWI=",
        "AAAAAAAAACNBdGl2YSB1bSBjbHViZSAoYXBlbmFzIG9yZ2FuaXphZG9yKQAAAAAIYWN0aXZhdGUAAAACAAAAAAAAAAdjbHViX2lkAAAAAAYAAAAAAAAACW9yZ2FuaXplcgAAAAAAABMAAAAA",
        "AAAAAAAAAB1PYnTDqW0gbGlzdGEgZGUgY2x1YmVzIGF0aXZvcwAAAAAAABBnZXRfYWN0aXZlX2NsdWJzAAAAAAAAAAEAAAPqAAAABg==",
        "AAAAAAAAABtBZGljaW9uYSB1bSBtZW1icm8gYW8gY2x1YmUAAAAACmFkZF9tZW1iZXIAAAAAAAIAAAAAAAAAB2NsdWJfaWQAAAAABgAAAAAAAAAGbWVtYmVyAAAAAAATAAAAAA==",
        "AAAAAAAAACNPYnTDqW0gdG9kb3Mgb3MgbWVtYnJvcyBkZSB1bSBjbHViZQAAAAALZ2V0X21lbWJlcnMAAAAAAQAAAAAAAAAHY2x1Yl9pZAAAAAAGAAAAAQAAA+oAAAAT",
        "AAAAAAAAACBEZXBvc2l0YSBVU0RDIG5vIGNsdWJlIGUgbyBhdGl2YQAAAAxkZXBvc2l0X3VzZGMAAAADAAAAAAAAAAdjbHViX2lkAAAAAAYAAAAAAAAACW9yZ2FuaXplcgAAAAAAABMAAAAAAAAABmFtb3VudAAAAAAACwAAAAA=",
        "AAAAAAAAADtBZGljaW9uYSB0b2tlbnMgS00gcGFyYSB1bSB1c3XDoXJpbyBlbSB1bSBjbHViZSBlc3BlY8OtZmljbwAAAAANYWRkX2ttX3Rva2VucwAAAAAAAAMAAAAAAAAAB2NsdWJfaWQAAAAABgAAAAAAAAAEdXNlcgAAABMAAAAAAAAACWttX2Ftb3VudAAAAAAAAAsAAAAA",
        "AAAAAAAAADtPYnTDqW0gYSBxdWFudGlkYWRlIGRlIHRva2VucyBLTSBkZSB1bSB1c3XDoXJpbyBlbSB1bSBjbHViZQAAAAASZ2V0X3VzZXJfa21fdG9rZW5zAAAAAAACAAAAAAAAAAR1c2VyAAAAEwAAAAAAAAAHY2x1Yl9pZAAAAAAGAAAAAQAAAAs=",
        "AAAAAAAAAChWZXJpZmljYSBzZSBvIHBlcsOtb2RvIGRvIGNsdWJlIHRlcm1pbm91AAAAFGlzX2NsdWJfcGVyaW9kX2VuZGVkAAAAAQAAAAAAAAAHY2x1Yl9pZAAAAAAGAAAAAQAAAAE=",
        "AAAAAAAAADlDYWxjdWxhIG8gdG90YWwgZGUgdG9rZW5zIEtNIGRlIHRvZG9zIG9zIG1lbWJyb3MgZG8gY2x1YmUAAAAAAAATZ2V0X3RvdGFsX2ttX3Rva2VucwAAAAABAAAAAAAAAAdjbHViX2lkAAAAAAYAAAABAAAACw==",
        "AAAAAAAAAE1DYWxjdWxhIGEgcmVjb21wZW5zYSBVU0RDIHBhcmEgdW0gdXN1w6FyaW8gYmFzZWFkbyBuYSByZWdyYSBkZSBkaXN0cmlidWnDp8OjbwAAAAAAABVjYWxjdWxhdGVfdXNkY19yZXdhcmQAAAAAAAACAAAAAAAAAAdjbHViX2lkAAAAAAYAAAAAAAAABHVzZXIAAAATAAAAAQAAAAs=",
        "AAAAAAAAACdFeGVjdXRhIG8gcmVzZ2F0ZSBkZSB0b2tlbnMgS00gcG9yIFVTREMAAAAAC3JlZGVlbV91c2RjAAAAAAMAAAAAAAAAB2NsdWJfaWQAAAAABgAAAAAAAAAEdXNlcgAAABMAAAAAAAAAC2Rlc3RpbmF0aW9uAAAAABMAAAABAAAACw==",
        "AAAAAAAAADBPYnTDqW0gaW5mb3JtYcOnw7VlcyBkZSByZXNnYXRlIHBhcmEgdW0gdXN1w6FyaW8AAAATZ2V0X3JlZGVtcHRpb25faW5mbwAAAAACAAAAAAAAAAdjbHViX2lkAAAAAAYAAAAAAAAABHVzZXIAAAATAAAAAQAAA+0AAAADAAAACwAAAAsAAAAB" ]),
      options
    )
  }
  public readonly fromJSON = {
    initialize: this.txFromJSON<null>,
        create_club: this.txFromJSON<u64>,
        get_club: this.txFromJSON<Club>,
        activate: this.txFromJSON<null>,
        get_active_clubs: this.txFromJSON<Array<u64>>,
        add_member: this.txFromJSON<null>,
        get_members: this.txFromJSON<Array<string>>,
        deposit_usdc: this.txFromJSON<null>,
        add_km_tokens: this.txFromJSON<null>,
        get_user_km_tokens: this.txFromJSON<i128>,
        is_club_period_ended: this.txFromJSON<boolean>,
        get_total_km_tokens: this.txFromJSON<i128>,
        calculate_usdc_reward: this.txFromJSON<i128>,
        redeem_usdc: this.txFromJSON<i128>,
        get_redemption_info: this.txFromJSON<readonly [i128, i128, boolean]>
  }
}