import type { EmbeddedProviderAccessor } from './providers';

export interface WalletConfig {
  id: string
  name: string
  getProvider: EmbeddedProviderAccessor
}
