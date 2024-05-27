import { useMounted } from '@vueuse/core';

export const useWallets = () => {
  const isMouted = useMounted();

  return computed(() => {
    return wallets.map((w) => ({
      ...w,
      isInstalled: isMouted.value ? !!w.getProvider() : undefined,
    }));
  });
};
