export const useChain = () => {
  const { public: { chainId } } = useRuntimeConfig();

  return {
    chainId,
    config: chains[chainId as ChainId],
  };
};
