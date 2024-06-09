import type { EmbeddedProviderAccessor } from './providers';

export interface WalletConfig {
  id: string
  name: string
  getProvider: EmbeddedProviderAccessor
  subscribeToKeystoreChange: (_cb: (e?: any) => void) => void
  download: string
}
