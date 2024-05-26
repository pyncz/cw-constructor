import type { WalletConfig } from '~/types';

export const wallets: WalletConfig[] = [
  {
    id: 'keplr',
    name: 'Keplr',
    getProvider: () => (window as any)?.keplr,
    subscribeToKeystoreChange: (cb: (e?: any) => void) => {
      window.addEventListener('keplr_keystorechange', cb);
    },
  },
  {
    id: 'leap',
    name: 'Leap',
    getProvider: () => (window as any)?.leap,
    subscribeToKeystoreChange: (cb: (e?: any) => void) => {
      window.addEventListener('leap_keystorechange', cb);
    },
  },
];
