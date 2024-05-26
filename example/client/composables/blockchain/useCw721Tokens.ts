import { useQuery } from '@tanstack/vue-query';
import type { ArgumentsType, DeepMaybeRef } from '@vueuse/core';
import type { QueryOptions } from '~/types';

type Msg = Partial<ArgumentsType<ReturnType<typeof useCw721Contract>['tokens']>[0]>;

export const UseCw721Tokens = {
  getKey: (
    address: MaybeRefOrGetter<string | undefined>,
    msg: DeepMaybeRef<Msg>,
  ) => {
    return ['cw', toRef(address), 'tokens', { owner: toRef(msg.owner) }];
  },

  useQuery: (
    address: MaybeRefOrGetter<string | undefined>,
    msg: DeepMaybeRef<Msg>,
    options?: QueryOptions,
  ) => {
    const { tokens } = useCw721Contract();

    return useQuery({
      ...options,
      queryKey: UseCw721Tokens.getKey(address, msg),
      queryFn: async () => {
        return await tokens({
          owner: toValue(msg.owner)!,
        }, {
          contractAddress: toValue(address)!,
        });
      },
      enabled: () => !!toValue(address) && !!toValue(msg.owner) && (toValue(options?.enabled) ?? true),
    });
  },
};
