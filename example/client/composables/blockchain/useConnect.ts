import type { OfflineSigner } from '@cosmjs/proto-signing';
import { computedAsync, createSharedComposable, useLocalStorage } from '@vueuse/core';
import { SigningArchwayClient, type SigningArchwayClientOptions } from '@archwayhq/arch3.js';
import { useAsync } from '../helpers/useAsync';
import { NotInstalledError, type WalletConfig } from '~/types';
import { wallets } from '~/configs';

type ConnectOptions = SigningArchwayClientOptions;

export const useConnect = createSharedComposable(() => {
  const { chainId, config: { rpc } } = useChain();

  const connectedWalletId = useLocalStorage<string | undefined>('connected-wallet-id', undefined);

  const signer = ref<OfflineSigner>();
  const client = ref<SigningArchwayClient>();

  const { execute: connect, isPending: isConnecting } = useAsync(async (wallet: WalletConfig, options?: ConnectOptions) => {
    const provider = wallet.getProvider();
    if (!provider) {
      throw new NotInstalledError();
    }
    await provider.enable(chainId);
    signer.value = await provider.getOfflineSignerAuto(chainId, {
      preferNoSetFee: true,
      preferNoSetMemo: true,
      disableBalanceCheck: true,
    });

    client.value = await SigningArchwayClient.connectWithSigner(rpc, signer.value, options);
    connectedWalletId.value = wallet.id;
  });

  const reconnect = async (options?: ConnectOptions) => {
    if (connectedWalletId.value) {
      const connectedWallet = wallets.find((w) => w.id === connectedWalletId.value);
      // Try to restore connection if wallet is installed
      if (connectedWallet?.getProvider()) {
        try {
          return await connect(connectedWallet, options);
        } catch (e) {}
      }
      // If failed to connect or not installed, clear stored connection method
      connectedWalletId.value = undefined;
    }
  };

  const accounts = computedAsync(async () => {
    return await signer.value?.getAccounts();
  });
  const address = computed(() => {
    return accounts.value?.at(0)?.address;
  });

  const isConnected = computed(() => !!address.value);

  const disconnect = () => {
    signer.value = undefined;
    client.value = undefined;
    connectedWalletId.value = undefined;
  };

  return {
    signer,
    client,
    isConnected,
    accounts,
    address,
    connect,
    reconnect,
    disconnect,
    isConnecting,
  };
});
