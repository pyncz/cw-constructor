import { useQuery } from '@tanstack/vue-query';
import type { QueryOptions } from '~/types';

export const UseCw721Minter = {
  getKey: (
    address: MaybeRefOrGetter<string | undefined>,
  ) => {
    return ['cw', toRef(address), 'minter'];
  },

  useQuery: (
    address: MaybeRefOrGetter<string | undefined>,
    options?: QueryOptions,
  ) => {
    const { minter } = useCw721Contract();

    return useQuery({
      ...options,
      queryKey: UseCw721Minter.getKey(address),
      queryFn: async () => {
        return await minter({
          contractAddress: toValue(address)!,
        });
      },
      enabled: () => !!toValue(address) && (toValue(options?.enabled) ?? true),
    });
  },
};
