<script setup lang="ts">
// Fetch constructor setup to get base token address
const { data: constructorConfig, isLoading: isLoadingConfig } = UseCwConstructorConfig.useQuery();

// Get current user's tokens
const { address } = useConnect();
const { data, isLoading: isLoadingTokens, fetchNextPage, isFetching, hasNextPage } = UseCw721TokensInfinite.useQuery(() => constructorConfig.value?.base_token, {
  owner: address,
});

const isLoading = computed(() => isLoadingConfig.value || isLoadingTokens.value);
useProvideLoading(isLoading);
</script>

<template>
  <section class="grid gap-12 grid-cols-1 sm:grid-cols-2 md:grid-cols-[minmax(0,2fr)_minmax(0,3fr)]">
    <div>
      <mint-card :address="constructorConfig?.base_token" class="mx-auto max-w-xs sm:my-16" />
    </div>
    <div class="space-y-5">
      <h3>Your frens</h3>
      <connected-only>
        <div class="space-y-4">
          <div class="grid grid-cols-autofill-32 gap-1">
            <skeleton-group>
              <item-preview v-for="tokenId of data?.tokens" :key="tokenId" :token-id />
              <template #fallback>
                <skeleton-element v-for="i in 3" :key="i" class="rounded aspect-square" />
              </template>
            </skeleton-group>
          </div>
          <button class="link" :disabled="isLoading || isFetching || !hasNextPage" @click="fetchNextPage()">
            Load More
          </button>
        </div>
        <template #fallback>
          <div class="placeholder">
            Connect to see your frens.
          </div>
        </template>
      </connected-only>
    </div>
  </section>
</template>
