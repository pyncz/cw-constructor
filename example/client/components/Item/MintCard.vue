<script setup lang="ts">
const props = defineProps<{
  address?: string
}>();

const { minterConfig, minterAddress, tokensMinted, funds, isMintedOut, isLoading } = useMintConfig(() => props.address);
useProvideLoading(isLoading);

const { isConnected } = useConnect();
const { mutate: mint, isPending } = useCwMinterMintMutation();
</script>

<template>
  <div class="text-lg space-y-6 sm:space-y-8">
    <img src="/img/placeholder.png" alt="Mysterious Fiend placeholder" class="w-full aspect-square rounded" />

    <div class="grid gap-x-4 gap-y-2 xs:grid-cols-4 text-sm sm:text-base">
      <attribute-record label="Price" class="xs:col-span-2">
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
      </attribute-record>
      <attribute-record label="Supply">
        <skeleton-group>
          {{ minterConfig?.supply ?? '-' }}
        </skeleton-group>
      </attribute-record>
      <attribute-record label="Minted">
        <skeleton-group>
          {{ tokensMinted }}
        </skeleton-group>
      </attribute-record>
    </div>

    <div class="space-y-1">
      <button
        class="button-primary w-full font-semibold"
        :disabled="!isConnected || !minterAddress || !minterConfig || isPending || isMintedOut"
        @click="mint({ contractAddress: minterAddress!, funds })"
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
