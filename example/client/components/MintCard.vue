<script setup lang="ts">
const props = defineProps<{
  address?: string
}>();

const { isConnected } = useConnect();

// Get mint config
const { data: minterRes, isLoading: isMinterLoading } = UseCw721Minter.useQuery(() => props.address);
const { data: minterConfig, isLoading: isMinterConfigLoading } = UseCwMinterConfig.useQuery(() => minterRes.value?.minter);

// TODO: Consider burned items
// - either allow to re-mint burned ids in minter's `mint` implementation
// - or query minter's counter of mints instead of using `num_tokens` to determine the cap
const { data: numTokensRes, isLoading: isNumTokensLoading } = UseCw721NumTokens.useQuery(() => props.address);

const isLoading = computed(() => !props.address || isMinterLoading.value || isMinterConfigLoading.value || isNumTokensLoading.value);
useProvideLoading(isLoading);

const { mutate: mint, isPending } = useCwMinterMintMutation();
</script>

<template>
  <div class="text-lg space-y-6 sm:space-y-8">
    <img src="/img/placeholder.png" alt="Mysterious Fiend placeholder" class="w-full aspect-square rounded" />

    <div class="grid gap-x-4 gap-y-2 xs:grid-cols-4 text-sm sm:text-base">
      <attribute-item label="Price" class="xs:col-span-2">
        <skeleton-group>
          <amount-representation
            v-if="minterConfig?.price"
            :amount="minterConfig.price.amount"
            :denom="minterConfig.price.denom"
          />
          <template v-else>
            Free
          </template>
        </skeleton-group>
      </attribute-item>
      <attribute-item label="Supply">
        <skeleton-group>
          {{ minterConfig?.supply ?? '-' }}
        </skeleton-group>
      </attribute-item>
      <attribute-item label="Minted">
        <skeleton-group>
          {{ numTokensRes?.count }}
        </skeleton-group>
      </attribute-item>
    </div>

    <div class="space-y-1">
      <button
        class="button-primary w-full font-semibold"
        :disabled="!isConnected || !minterRes?.minter || !minterConfig || isPending"
        @click="mint({ contractAddress: minterRes?.minter!, funds: minterConfig?.price ? [minterConfig.price] : undefined })"
      >
        mint
      </button>

      <transition
        enter-active-class="duration-xs"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="duration-xs"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <p v-if="!isConnected" class="text-xs text-main-200">
          Connect first.
        </p>
      </transition>
    </div>
  </div>
</template>
