import { useMutation } from '@tanstack/vue-query';
import { type ExecuteOptions, InsufficientBalanceError, NotConnectedError } from '~/types';

interface Variables extends ExecuteOptions {
  // require contract address provided
  contractAddress: string
}

export const useCwMinterMintMutation = () => {
  const { $queryClient: queryClient } = useNuxtApp();
  const { address } = useConnect();
  const { mint } = useCwMinterContract();
  const { setStart, setSuccess, setError } = useProgressModal();
  const { config: { denom: nativeDenom } } = useChain();

  const mutation = useMutation({
    mutationFn: async (variables: Variables) => {
      // check balance
      if (!address.value) {
        throw new NotConnectedError();
      }
      const { price } = await UseCwMinterConfig.fetch(variables.contractAddress, { retry: false });
      if (price) {
        const { amount: balance } = await UseBalance.fetch(address.value, price.denom);
        if (BigInt(balance) < BigInt(price.amount)) {
          throw new InsufficientBalanceError(`Insufficient ${price.denom} balance!`);
        }
      }

      return await mint(variables);
    },
    onMutate: async () => {
      setStart({
        title: 'Minting token...',
      });
    },
    onSuccess: async ({ events }, variables) => {
      const tokenId = events
        .find((event) => event.type === 'wasm')
        ?.attributes.find((attr) => attr.key === 'token_id')
        ?.value;

      const { cw721, price } = await UseCwMinterConfig.fetch(variables.contractAddress, { retry: false });

      // Invalidate queries
      await Promise.allSettled([
        // balances
        queryClient.invalidateQueries({ queryKey: UseNativeBalance.getKey(address) }),
        ...(price && price.denom !== nativeDenom
          ? [
            queryClient.invalidateQueries({ queryKey: UseBalance.getKey(address, price.denom) }),
          ]
          : []),
        // cw
        queryClient.invalidateQueries({ queryKey: UseCw721NumTokens.getKey(cw721!) }),
        queryClient.invalidateQueries({ queryKey: UseCw721Tokens.getKey(cw721!, { owner: address }) }),
        queryClient.invalidateQueries({ queryKey: UseCw721TokensInfinite.getKey(cw721!, { owner: address }) }),
        queryClient.invalidateQueries({ queryKey: UseCw721AllNftInfo.getKey(cw721!, { tokenId }) }),
        queryClient.invalidateQueries({ queryKey: UseCw721NftInfo.getKey(cw721!, { tokenId }) }),
        queryClient.invalidateQueries({ queryKey: UseCwConstructorInfo.getKey({ tokenId }) }),
        queryClient.invalidateQueries({ queryKey: UseCwConstructorTokens.getKey({ address: cw721!, tokenId }) }),
      ]);

      setSuccess({
        title: 'Successfully minted!',
        link: tokenId
          ? {
            label: 'Show',
            uri: `/tokens/${tokenId}`,
          }
          : undefined,
      }, { autoreset: true });
    },
    onError: async (error, variables) => {
      setError({
        title: 'Mint Failed.',
        retry: () => mutation.mutate(variables),
        error,
      }, { autoreset: true });
    },
  });

  return mutation;
};
