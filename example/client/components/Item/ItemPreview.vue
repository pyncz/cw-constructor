<script setup lang="ts">
import { UseCw721NftInfo } from '~/composables';
import { UseCwConstructorInfo } from '~/composables/blockchain/useCwConstructorInfo';
import type { Extension } from '~/types';

const props = defineProps<{
  tokenId: string
}>();

// Get token image
// - with traits from constructor, if exists
const { data: mergedInfo, isLoading: isLoadingMergedInfo, isError } = UseCwConstructorInfo.useQuery({
  tokenId: toRef(() => props.tokenId),
}, {
  throwOnError: false,
});
// - otherwise just single cw721 info
const { data: config, isLoading: isLoadingConfig } = UseCwConstructorConfig.useQuery();
const { data: baseInfo, isLoading: isLoadingBaseInfo } = UseCw721NftInfo.useQuery<Extension>(() => config.value?.base_token, {
  tokenId: toRef(() => props.tokenId),
}, {
  // fallback to cw721 info only if got not found from constructor
  enabled: isError,
  throwOnError: false,
});

const images = computed(() => {
  if (mergedInfo.value) {
    return mergedInfo.value.info.extension?.images;
  }
  return baseInfo.value?.extension ? [baseInfo.value?.extension.image] : undefined;
});

const isLoading = computed(() => isLoadingMergedInfo.value || isLoadingConfig.value || isLoadingBaseInfo.value);
useProvideLoading(isLoading);
</script>

<template>
  <skeleton-group class="rounded aspect-square">
    <nuxt-link :href="`/tokens/${tokenId}`" class="group/card relative duration-sm aspect-square overflow-hidden rounded border border-opacity-0 border-dashed border-main-200 focus-visible:border-solid focus-visible:border-main-100 focus-visible:border-opacity-100 hover:border-opacity-100 outline-none">
      <div class="absolute -inset-px duration-sm group-hover/card:inset-2 group-focus-visible/card:inset-2 overflow-hidden rounded">
        <img
          v-for="image of images"
          :key="image"
          class="absolute inset-0"
          :src="image"
          alt=""
        />
      </div>
    </nuxt-link>
  </skeleton-group>
</template>
