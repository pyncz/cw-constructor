import { useQuery } from '@tanstack/vue-query';
import type { QueryOptions } from '~/types';

export const UseCw721NumTokens = {
  getKey: (
    address: MaybeRefOrGetter<string | undefined>,
  ) => {
    return ['cw', toRef(address), 'num_tokens'];
  },

  useQuery: (
    address: MaybeRefOrGetter<string | undefined>,
    options?: QueryOptions,
  ) => {
    const { numTokens } = useCw721Contract();

    return useQuery({
      ...options,
      queryKey: UseCw721NumTokens.getKey(address),
      queryFn: async () => {
        return await numTokens({
          contractAddress: toValue(address)!,
        });
      },
      enabled: () => !!toValue(address) && (toValue(options?.enabled) ?? true),
    });
  },
};
