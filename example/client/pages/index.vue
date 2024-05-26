<script setup lang="ts">
// Fetch constructor setup to get base token address
const { data: constructorConfig, isLoading: isLoadingConfig } = UseCwConstructorConfig.useQuery();

// Get current user's tokens
const { address } = useConnect();
const { data, isLoading: isLoadingTokens } = UseCw721Tokens.useQuery(() => constructorConfig.value?.base_token, {
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
            <item-preview v-for="tokenId of data?.tokens" :key="tokenId" :token-id />
          </div>
          <button class="link">
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
