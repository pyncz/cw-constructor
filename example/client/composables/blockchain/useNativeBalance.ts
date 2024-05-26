import { UseBalance } from './useBalance';
import type { FetchOptions, QueryOptions } from '~/types';

export const UseNativeBalance = {
  getKey: (address: MaybeRefOrGetter<string | undefined>) => {
    const { config: { denom } } = useChain();
    return UseBalance.getKey(address, denom);
  },

  useQuery: (
    address: MaybeRefOrGetter<string | undefined>,
    options?: QueryOptions,
  ) => {
    const { config: { denom } } = useChain();
    return UseBalance.useQuery(address, denom, options);
  },

  fetch: async (address: string, options?: FetchOptions) => {
    const { config: { denom } } = useChain();
    return await UseBalance.fetch(address, denom, options);
  },
};
