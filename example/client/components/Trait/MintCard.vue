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
  <div class="card flex flex-col gap-4">
    <div class="xs:text-center flex flex-col xs:items-center xs:justify-center flex-1 gap-y-2 sm:gap-y-1 pb-1 xs:pb-0">
      <div class="flex xs:flex-col gap-x-2 items-center xs:py-1">
        <trait-icon :name class="size-8 text-main-100" />
        <div class="flex flex-col">
          <span>{{ name }}</span>
          <span class="hidden xs:inline sm:hidden text-5/8 text-main-200 leading-1">
            <skeleton-group>
              <template v-if="minterConfig?.supply">
                {{ tokensMinted ?? 0 }}/{{ minterConfig.supply }}
              </template>
            </skeleton-group>
          </span>
        </div>
      </div>
      <div class="flex xs:flex-col gap-x-6 gap-y-2 text-main-100 xs:text-main-200 text-3/4">
        <attribute-record label="Minted" reversed class="xs:hidden sm:flex">
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
