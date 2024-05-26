import { useQuery } from '@tanstack/vue-query';
import type { ArgumentsType, DeepMaybeRef } from '@vueuse/core';
import type { QueryOptions } from '~/types';

type Msg = Partial<ArgumentsType<ReturnType<typeof useCw721Contract>['nftInfo']>[0]>;

export const UseCw721NftInfo = {
  getKey: (
    address: MaybeRefOrGetter<string | undefined>,
    msg: DeepMaybeRef<Msg>,
  ) => {
    return ['cw', toRef(address), 'nft_info', { tokenId: toRef(msg.tokenId) }];
  },

  useQuery: <T = any>(
    address: MaybeRefOrGetter<string | undefined>,
    msg: DeepMaybeRef<Msg>,
    options?: QueryOptions,
  ) => {
    const { nftInfo } = useCw721Contract();

    return useQuery({
      ...options,
      queryKey: UseCw721NftInfo.getKey(address, msg),
      queryFn: async () => {
        return await nftInfo<T>({
          tokenId: toValue(msg.tokenId)!,
        }, {
          contractAddress: toValue(address)!,
        });
      },
      enabled: () => !!toValue(address) && !!toValue(msg.tokenId) && (toValue(options?.enabled) ?? true),
    });
  },
};
