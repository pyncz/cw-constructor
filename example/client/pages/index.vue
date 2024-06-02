<script setup lang="ts">
const { address, isConnecting } = useConnect();

// Fetch constructor setup to get base token address
const { data: constructorConfig, isLoading: isLoadingConfig } = UseCwConstructorConfig.useQuery();

const isLoading = computed(() => isConnecting.value || isLoadingConfig.value);
useProvideLoading(isLoading);
</script>

<template>
  <section class="grid gap-12 grid-cols-1 sm:grid-cols-2 md:grid-cols-[minmax(0,2fr)_minmax(0,3fr)]">
    <div>
      <item-mint-card :address="constructorConfig?.base_token" class="mx-auto max-w-xs sm:my-16" />
    </div>
    <div class="space-y-5">
      <h3>Your frens</h3>
      <connected-only>
        <tokens-list :address="constructorConfig?.base_token" :owner="address">
          <template #default="{ tokenId }">
            <item-image-card :token-id />
          </template>
          <template #empty>
            <div class="placeholder">
              You don't have any frens ;(
            </div>
          </template>
          <template #skeleton>
            <div class="aspect-square p-1">
              <skeleton-element class="rounded h-full" />
            </div>
          </template>
        </tokens-list>
        <template #fallback>
          <div class="placeholder">
            Connect to see your frens.
          </div>
        </template>
      </connected-only>
    </div>
  </section>
</template>
