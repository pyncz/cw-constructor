<script setup lang="ts">
interface Props {
  address?: string
  owner?: string
  gridClass?: string
  wrapperClass?: string
};
const props = withDefaults(defineProps<Props>(), {
  gridClass: 'grid grid-cols-autofill-32 w-full',
});

const { address, owner } = toRefs(props);
const { data, isLoading, fetchNextPage, isFetching, hasNextPage } = UseCw721TokensInfinite.useQuery(address, {
  owner,
});

useProvideLoading(isLoading);
</script>

<template>
  <section class="flex flex-col gap-4 items-start">
    <div class="w-full" :class="wrapperClass">
      <skeleton-group>
        <div v-if="data?.tokens.length" :class="gridClass">
          <slot v-for="tokenId of data.tokens" :key="tokenId" v-bind="{ tokenId }" />
        </div>
        <slot v-else name="empty" />

        <template #fallback>
          <div :class="gridClass">
            <slot v-for="i in 3" :key="i" name="skeleton" />
          </div>
        </template>
      </skeleton-group>
    </div>
    <transition
      mode="out-in"
      enter-active-class="duration-sm"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="duration-sm"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <button
        v-if="hasNextPage || isLoading"
        class="link"
        :disabled="isLoading || isFetching || !hasNextPage"
        @click="fetchNextPage()"
      >
        Load More
      </button>
      <span v-else class="text-main-500">
        {{ data?.tokens.length ?? 0 }} total
      </span>
    </transition>
  </section>
</template>
