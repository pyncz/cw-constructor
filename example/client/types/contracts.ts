import type { Coin } from '@cosmjs/proto-signing';

export type { Coin };
export type Msg = Record<string, any>;

export interface CallOptions {
  contractAddress?: string
}

export interface ExecuteMultipleOptions extends CallOptions {
  memo?: string
  gasAdjustment?: number
}

export interface ExecuteOptions extends ExecuteMultipleOptions {
  funds?: Coin[]
}

export interface ExecInstruction extends CallOptions {
  msg: Msg
  funds?: readonly Coin[]
}

export type Expiration =
  | {
    /** Timestamp in nanoseconds */
    at_time: `${number}`
  }
  | {
    /** Block height */
    at_height: number
  }
  | {
    // Make sure it doesn't allow something like `{ never: 42 }`, like `{ never: {} }` type does
    never: Record<string, never>
  };

export interface CwCursorPagination {
  start_after?: string
  limit?: number
}
