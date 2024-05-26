import { useQuery } from '@tanstack/vue-query';
import type { FetchOptions, QueryOptions } from '~/types';

export const UseBalance = {
  getKey: (
    address: MaybeRefOrGetter<string | undefined>,
    denom: MaybeRefOrGetter<string | undefined>,
  ) => {
    return ['chain', 'balance', toRef(address), toRef(denom)];
  },

  useQuery: (
    address: MaybeRefOrGetter<string | undefined>,
    denom: MaybeRefOrGetter<string | undefined>,
    options?: QueryOptions,
  ) => {
    const { clientAsync: queryClientAsync } = useRpcClient();

    return useQuery({
      ...options,
      queryKey: UseBalance.getKey(address, denom),
      queryFn: async () => {
        const client = await queryClientAsync.value;
        return await client.getBalance(toValue(address)!, toValue(denom)!);
      },
      enabled: () => !!toValue(address) && !!toValue(denom) && (toValue(options?.enabled) ?? true),
    });
  },

  fetch: async (address: string, denom: string, options?: FetchOptions) => {
    const { $queryClient: queryClient } = useNuxtApp();
    const { clientAsync: queryClientAsync } = useRpcClient();

    return await queryClient.fetchQuery({
      ...options,
      queryKey: UseBalance.getKey(address, denom),
      queryFn: async () => {
        const client = await queryClientAsync.value;
        return await client.getBalance(address, denom);
      },
    });
  },
};
