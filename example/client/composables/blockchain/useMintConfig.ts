export const useMintConfig = (address: MaybeRefOrGetter<string | undefined>) => {
  // Get mint config
  const { data: minterRes, isLoading: isMinterLoading } = UseCw721Minter.useQuery(address);
  const minterAddress = computed(() => minterRes.value?.minter);

  const { data: minterConfig, isLoading: isMinterConfigLoading } = UseCwMinterConfig.useQuery(minterAddress);
  const funds = computed(() => minterConfig.value?.price ? [minterConfig.value.price] : undefined);

  // TODO: Consider burned items
  // - either allow to re-mint burned ids in minter's `mint` implementation
  // - or query minter's counter of mints instead of using `num_tokens` to determine the cap
  const { data: numTokensRes, isLoading: isNumTokensLoading } = UseCw721NumTokens.useQuery(address);
  const tokensMinted = computed(() => numTokensRes.value?.count);

  const isMintedOut = computed(() => {
    return tokensMinted.value && minterConfig.value?.supply
      ? tokensMinted.value >= minterConfig.value.supply
      : undefined;
  });

  const isLoading = computed(() => !toValue(address) || isMinterLoading.value || isMinterConfigLoading.value || isNumTokensLoading.value);

  return {
    minterAddress,
    minterConfig,
    funds,
    tokensMinted,
    isMintedOut,
    isLoading,
  };
};
