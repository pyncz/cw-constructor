<script setup lang="ts">
import type { TraitExtension } from '~/types';

defineOptions({
  inheritAttrs: false,
});

const props = defineProps<{
  address: string
  tokenId: string
}>();

const { address, tokenId } = toRefs(props);

const { data, isLoading } = UseCw721AllNftInfo.useQuery<TraitExtension>(address, {
  tokenId,
});
useProvideLoading(isLoading);

const images = computed(() => [
  '/img/template.png',
  ...(data.value?.info.extension?.image
    ? [data.value.info.extension.image]
    : []),
]);

const numericAttributes = computed(() => data.value?.info.extension?.attributes.filter((a) => !!a.display_type));

const { address: userAddress } = useConnect();
const isOwnedByCurrentUser = computed(() => {
  return data.value
    ? userAddress.value === data.value.access.owner
    : undefined;
});
</script>

<template>
  <skeleton-group>
    <div class="card group/card flex flex-col gap-3" v-bind="$attrs">
      <item-image :images class="w-full aspect-square" />

      <div class="flex-1 flex flex-col gap-2 -mx-0.5">
        <div class="flex-1 space-y-1">
          <div class="duration-sm leading-xs text-main-100 group-hover/card:text-main-0 break-words line-clamp-2" :title="data?.info.extension?.name">
            {{ data?.info.extension?.name }}
          </div>
          <div class="flex gap-2 text-7/8">
            <attribute-boost v-for="attr of numericAttributes" :key="attr.trait_type" v-bind="attr" />
          </div>
        </div>

        <div v-if="isOwnedByCurrentUser">
          yours
        </div>
        <div v-else>
          not yours
        </div>
      </div>
    </div>

    <template #fallback>
      <skeleton-element class="rounded aspect-square" />
    </template>
  </skeleton-group>
</template>
