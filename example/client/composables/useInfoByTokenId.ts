import type { Extension, MergedExtension } from '~/types';

export const useInfoByTokenId = (tokenId: MaybeRefOrGetter<string | undefined>) => {
  // Get token info
  // - merged with traits from constructor contract, if exists
  const { data: mergedInfo, isLoading: isLoadingMergedInfo } = UseCwConstructorInfo.useQuery({
    tokenId: toRef(tokenId),
  }, {
    throwOnError: false,
  });
  // - original one from cw721 contract
  const { data: config, isLoading: isLoadingConfig } = UseCwConstructorConfig.useQuery();
  const { data: allBaseInfo, isLoading: isLoadingBaseInfo } = UseCw721AllNftInfo.useQuery<Extension>(() => config.value?.base_token, {
    tokenId: toRef(tokenId),
  });

  const baseInfo = computed(() => allBaseInfo.value?.info);
  const owner = computed(() => allBaseInfo.value?.access.owner);

  const info = computed<NftInfoResponse<MergedExtension> | undefined>(() => {
    if (mergedInfo.value) {
      return mergedInfo.value.info;
    }
    return baseInfo.value
      ? {
        ...baseInfo.value,
        extension: baseInfo.value.extension
          ? {
            ...baseInfo.value.extension,
            images: [baseInfo.value.extension.image],
          }
          : null,
      }
      : undefined;
  });

  const isLoading = computed(() => isLoadingMergedInfo.value || isLoadingConfig.value || isLoadingBaseInfo.value);

  return {
    info,
    mergedInfo,
    owner,
    isLoadingMergedInfo,
    baseInfo,
    isLoadingBaseInfo,
    config,
    isLoadingConfig,
    isLoading,
  };
};
