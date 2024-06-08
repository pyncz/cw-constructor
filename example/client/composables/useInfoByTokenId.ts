import type { TraitInfo } from './blockchain';
import type { Extension, MergedExtension, TraitExtension } from '~/types';

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
  const { suspense, data: allBaseInfo, isLoading: isLoadingBaseInfo } = UseCw721AllNftInfo.useQuery<Extension>(() => config.value?.base_token, {
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

  const traitsInfoBySlots = computed(() => {
    return mergedInfo.value?.traits.reduce((res, t) => {
      if (res[t.slot]) {
        res[t.slot].push(t);
      } else {
        res[t.slot] = [t];
      }
      return res;
    }, {} as Record<string, TraitInfo<TraitExtension>[]>);
  });

  const isLoading = computed(() => isLoadingMergedInfo.value || isLoadingConfig.value || isLoadingBaseInfo.value);

  return {
    info,
    mergedInfo,
    traitsInfoBySlots,
    owner,
    isLoadingMergedInfo,
    baseInfo,
    suspense,
    isLoadingBaseInfo,
    config,
    isLoadingConfig,
    isLoading,
  };
};
