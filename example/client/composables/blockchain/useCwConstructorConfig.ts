import { useQuery } from '@tanstack/vue-query';
import type { QueryOptions } from '~/types';

export const UseCwConstructorConfig = {
  getKey: () => {
    const { public: { constructorAddress } } = useRuntimeConfig();
    return ['cw', constructorAddress, 'contract_info'];
  },

  useQuery: (options?: QueryOptions) => {
    const { public: { constructorAddress } } = useRuntimeConfig();
    const { contractInfo } = useCwConstructorContract(constructorAddress);

    return useQuery({
      ...options,
      queryKey: UseCwConstructorConfig.getKey(),
      queryFn: async () => {
        return await contractInfo();
      },
    });
  },
};
