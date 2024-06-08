import { useMutation } from '@tanstack/vue-query';
import type { ArgumentsType } from '@vueuse/core';
import type { ExecuteOptions } from '~/types';

type Msg = ArgumentsType<ReturnType<typeof useCwConstructorContract>['equip']>[0];

interface Variables extends ExecuteOptions, Msg {
  // require contract address provided
  contractAddress: string
}

export const useCwConstructorEquipMutation = () => {
  const { $queryClient: queryClient } = useNuxtApp();
  const { address } = useConnect();
  const { equip } = useCwConstructorContract();
  const { setStart, setSuccess, setError } = useProgressModal();
  const { config: { denom: nativeDenom } } = useChain();

  const mutation = useMutation({
    mutationFn: async ({ tokenId, trait, ...options }: Variables) => {
      return await equip({ tokenId, trait }, options);
    },
    onMutate: async () => {
      setStart({
        title: 'Equipping trait...',
      });
    },
    onSuccess: async (_, variables) => {
      const { tokenId, trait } = variables;

      // Invalidate queries
      await Promise.allSettled([
        // balance
        queryClient.invalidateQueries({ queryKey: UseBalance.getKey(address, nativeDenom) }),
        // cw
        queryClient.invalidateQueries({ queryKey: UseCwConstructorInfo.getKey({ tokenId }) }),
        queryClient.invalidateQueries({ queryKey: UseCwConstructorTokens.getKey(trait) }),
      ]);

      setSuccess({
        title: 'Successfully equipped!',
      }, { autoreset: true });
    },
    onError: async (error, variables) => {
      setError({
        title: 'Equip Failed.',
        retry: () => mutation.mutate(variables),
        error,
      }, { autoreset: true });
    },
  });

  return mutation;
};
