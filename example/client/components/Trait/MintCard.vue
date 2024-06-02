<script setup lang="ts">
const props = defineProps<{
  name: string
  address?: string
}>();

const { minterConfig, minterAddress, tokensMinted, funds, isLoading } = useMintConfig(() => props.address);
useProvideLoading(isLoading);

const { isConnected } = useConnect();
const { mutate: mint, isPending } = useCwMinterMintMutation();
</script>

<template>
  <div class="card flex flex-col gap-4 !bg-main-800">
    <div class="text-center flex flex-col items-center justify-center flex-1 gap-2 sm:gap-1">
      <div class="flex flex-col items-center py-1">
        <trait-icon :name class="size-8 text-main-100" />
        {{ name }}
        <span class="sm:hidden text-5/8 text-main-200 leading-1">
          <skeleton-group>
            <template v-if="minterConfig?.supply">
              {{ tokensMinted ?? 0 }}/{{ minterConfig.supply }}
            </template>
          </skeleton-group>
        </span>
      </div>
      <div class="flex flex-col gap-2 text-main-200 text-3/4">
        <attribute-record label="Minted" reversed class="hidden sm:flex">
          <skeleton-group>
            <template v-if="minterConfig?.supply">
              {{ tokensMinted ?? 0 }}/{{ minterConfig.supply }}
            </template>
          </skeleton-group>
        </attribute-record>
        <attribute-record label="Price" reversed>
          <skeleton-group>
            <amount-representation
              v-if="minterConfig?.price"
              :amount="minterConfig.price.amount"
              :denom="minterConfig.price.denom"
            />
          </skeleton-group>
        </attribute-record>
      </div>
    </div>
    <button
      class="button-primary w-full text-7/8 font-medium"
      :disabled="!isConnected || !minterAddress || !minterConfig || isPending"
      @click="mint({ contractAddress: minterAddress!, funds })"
    >
      mint
    </button>
  </div>
</template>
