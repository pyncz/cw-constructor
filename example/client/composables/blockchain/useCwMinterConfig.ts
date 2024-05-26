import { useQuery } from '@tanstack/vue-query';
import type { FetchOptions, QueryOptions } from '~/types';

export const UseCwMinterConfig = {
  getKey: (
    address: MaybeRefOrGetter<string | undefined>,
  ) => {
    return ['cw', toRef(address), 'contract_info'];
  },

  useQuery: (
    address: MaybeRefOrGetter<string | undefined>,
    options?: QueryOptions,
  ) => {
    const { contractInfo } = useCwMinterContract(address);

    return useQuery({
      ...options,
      queryKey: UseCwMinterConfig.getKey(address),
      queryFn: async () => {
        return await contractInfo();
      },
      enabled: () => !!toValue(address) && (toValue(options?.enabled) ?? true),
    });
  },

  fetch: async (address: string, options?: FetchOptions) => {
    const { $queryClient: queryClient } = useNuxtApp();
    const { contractInfo } = useCwMinterContract(address);

    return await queryClient.fetchQuery({
      ...options,
      queryKey: UseCwMinterConfig.getKey(address),
      queryFn: async () => {
        return await contractInfo();
      },
    });
  },
};
