import { type TraitInfo, UseCw721OwnerOf } from './blockchain';
import type { TraitExtension } from '~/types';

export const useInfoByTokenId = (tokenId: MaybeRefOrGetter<string | undefined>) => {
  // Get token info
  const { suspense, data: mergedInfo, isLoading: isLoadingMergedInfo } = UseCwConstructorInfo.useQuery({
    tokenId: toRef(tokenId),
  });

  const baseInfo = computed(() => mergedInfo.value?.base_token.token);
  const info = computed(() => mergedInfo.value?.info);

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

  // Get base token's owner
  const { data: config, isLoading: isLoadingConfig } = UseCwConstructorConfig.useQuery();
  const { data: ownerInfo, isLoading: isLoadingOwner } = UseCw721OwnerOf.useQuery(() => config.value?.base_token, {
    tokenId: toRef(tokenId),
  });

  const owner = computed(() => ownerInfo.value?.owner);

  const isLoading = computed(() => isLoadingMergedInfo.value || isLoadingConfig.value || isLoadingOwner.value);

  return {
    info,
    mergedInfo,
    traitsInfoBySlots,
    owner,
    isLoadingMergedInfo,
    baseInfo,
    suspense,
    isLoadingOwner,
    config,
    isLoadingConfig,
    isLoading,
  };
};
