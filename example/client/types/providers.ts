import type { OfflineSigner } from '@cosmjs/proto-signing';

interface SignOptions {
  preferNoSetFee?: boolean
  preferNoSetMemo?: boolean
  disableBalanceCheck?: boolean
}

export interface EmbeddedProvider {
  enable: (_chainIds: string | string[]) => Promise<void>
  getOfflineSignerAuto: (_chainId: string, _signOptions?: SignOptions) => Promise<OfflineSigner>
  defaultOptions: {
    sign?: SignOptions
  }
}

export type EmbeddedProviderAccessor = () => EmbeddedProvider | undefined;
