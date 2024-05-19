import type { Coin } from '@cosmjs/proto-signing';

export type { Coin };
export type Msg = Record<string, any>;

export interface CallOptions {
  contractAddress?: string
}

export interface ExecInstruction extends CallOptions { msg: Msg; funds?: readonly Coin[] }
