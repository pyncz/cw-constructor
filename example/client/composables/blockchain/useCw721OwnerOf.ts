import { useQuery } from '@tanstack/vue-query';
import type { ArgumentsType, DeepMaybeRef } from '@vueuse/core';
import type { QueryOptions } from '~/types';

type Msg = Partial<ArgumentsType<ReturnType<typeof useCw721Contract>['ownerOf']>[0]>;

export const UseCw721OwnerOf = {
  getKey: (
    address: MaybeRefOrGetter<string | undefined>,
    msg: DeepMaybeRef<Msg>,
  ) => {
    return ['cw', toRef(address), 'owner_of', { tokenId: toRef(msg.tokenId) }];
  },

  useQuery: (
    address: MaybeRefOrGetter<string | undefined>,
    msg: DeepMaybeRef<Msg>,
    options?: QueryOptions,
  ) => {
    const { ownerOf } = useCw721Contract();

    return useQuery({
      ...options,
      queryKey: UseCw721OwnerOf.getKey(address, msg),
      queryFn: async () => {
        return await ownerOf({
          tokenId: toValue(msg.tokenId)!,
        }, {
          contractAddress: toValue(address)!,
        });
      },
      enabled: () => !!toValue(address) && !!toValue(msg.tokenId) && (toValue(options?.enabled) ?? true),
    });
  },
};
