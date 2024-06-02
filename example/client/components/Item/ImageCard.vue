<script setup lang="ts">
defineOptions({
  inheritAttrs: false,
});

const props = defineProps<{
  tokenId: string
}>();

const { info, isLoading } = useInfoByTokenId(() => props.tokenId);
useProvideLoading(isLoading);
</script>

<template>
  <skeleton-group>
    <nuxt-link v-bind="$attrs" :href="`/tokens/${tokenId}`" class="group/card p-4 block relative duration-sm aspect-square border border-dashed border-opacity-0 border-main-500 hover:border-opacity-50 focus-visible:border-main-400 hover:bg-main-800 outline-none">
      <item-image :images="info?.extension?.images" class="[--inset:theme(spacing.1)] inset-[--inset] group-hover/card:[--inset:theme(spacing.3)] group-focus-visible/card:[--inset:theme(spacing.3)] !absolute duration-sm overflow-hidden" />
    </nuxt-link>
    <template #fallback>
      <div class="aspect-square p-1">
        <skeleton-element class="rounded h-full" />
      </div>
    </template>
  </skeleton-group>
</template>
