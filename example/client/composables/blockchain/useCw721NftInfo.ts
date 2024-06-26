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
    const { resolve } = useIpfsGateway();

    return useQuery({
      ...options,
      queryKey: UseCw721NftInfo.getKey(address, msg),
      queryFn: async () => {
        const res = await nftInfo<T>({
          tokenId: toValue(msg.tokenId)!,
        }, {
          contractAddress: toValue(address)!,
        });
        return resolve(res);
      },
      enabled: () => !!toValue(address) && !!toValue(msg.tokenId) && (toValue(options?.enabled) ?? true),
    });
  },
};
