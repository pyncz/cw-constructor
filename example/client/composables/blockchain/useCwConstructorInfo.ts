import { useQuery } from '@tanstack/vue-query';
import type { ArgumentsType, DeepMaybeRef } from '@vueuse/core';
import type { Extension, MergedExtension, QueryOptions, TraitExtension } from '~/types';

type Msg = Partial<ArgumentsType<ReturnType<typeof useCwConstructorContract>['tokenInfo']>[0]>;

export const UseCwConstructorInfo = {
  getKey: (
    msg: DeepMaybeRef<Msg>,
  ) => {
    const { public: { constructorAddress } } = useRuntimeConfig();
    return ['cw', constructorAddress, 'info', msg];
  },

  useQuery: (
    msg: DeepMaybeRef<Msg>,
    options?: QueryOptions,
  ) => {
    const { public: { constructorAddress } } = useRuntimeConfig();
    const { tokenInfo } = useCwConstructorContract(constructorAddress);
    const { resolve } = useIpfsGateway();

    return useQuery({
      ...options,
      queryKey: UseCwConstructorInfo.getKey(msg),
      queryFn: async () => {
        const res = await tokenInfo<Extension, TraitExtension, MergedExtension>({ tokenId: toValue(msg.tokenId)! });
        return resolve(res);
      },
      enabled: () => !!toValue(msg.tokenId) && (toValue(options?.enabled) ?? true),
    });
  },
};
