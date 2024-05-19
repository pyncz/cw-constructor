import type { WalletConfig } from '~/types';

export const wallets: WalletConfig[] = [
  {
    id: 'keplr',
    name: 'Keplr',
    getProvider: () => (window as any)?.keplr,
  },
  {
    id: 'leap',
    name: 'Leap',
    getProvider: () => (window as any)?.leap,
  },
];
