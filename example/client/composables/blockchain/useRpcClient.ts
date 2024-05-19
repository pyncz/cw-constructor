import { ArchwayClient } from '@archwayhq/arch3.js';
import { createSharedComposable } from '@vueuse/core';

export const useRpcClient = createSharedComposable(() => {
  const { config: { rpc } } = useChain();

  const { data: client, dataAsync: clientAsync, isLoading, isReady } = useComputedAsync(async () => {
    return await ArchwayClient.connectWithBatchClient(rpc);
  });

  return {
    client,
    clientAsync,
    isLoading,
    isReady,
  };
});
