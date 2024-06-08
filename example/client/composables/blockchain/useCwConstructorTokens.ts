import { useQuery } from '@tanstack/vue-query';
import type { ArgumentsType, DeepMaybeRef } from '@vueuse/core';
import type { QueryOptions } from '~/types';

type Msg = Partial<ArgumentsType<ReturnType<typeof useCwConstructorContract>['tokens']>[0]>;

export const UseCwConstructorTokens = {
  getKey: (
    msg: DeepMaybeRef<Msg>,
  ) => {
    const { public: { constructorAddress } } = useRuntimeConfig();
    return ['cw', constructorAddress, 'tokens', msg];
  },

  useQuery: (
    msg: DeepMaybeRef<Msg>,
    options?: QueryOptions,
  ) => {
    const { public: { constructorAddress } } = useRuntimeConfig();
    const { tokens } = useCwConstructorContract(constructorAddress);

    return useQuery({
      ...options,
      queryKey: UseCwConstructorTokens.getKey(msg),
      queryFn: async () => {
        return await tokens({ address: toValue(msg.address), tokenId: toValue(msg.tokenId) });
      },
    });
  },
};
