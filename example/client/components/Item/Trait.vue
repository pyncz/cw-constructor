<script setup lang="ts">
defineProps<{
  name: string
  address: string
  tokenId: string
  baseTokenId: string
  withControls?: boolean
}>();

const { public: { constructorAddress } } = useRuntimeConfig();
const { mutate: unequip, isPending: isPendingUnequip } = useCwConstructorUnequipMutation();
</script>

<template>
  <div class="flex flex-col xs:flex-row xs:items-center gap-2 group/trait xs:border-b border-main-700 sm:border-b-0 py-2 xs:px-2 xs:h-[4.5rem] sm:h-auto">
    <trait-label :name class="flex-1 sm:flex-none">
      <span class="text-main-100 duration-sm group-hover/trait:text-main-0">
        <slot />
      </span>
    </trait-label>
    <button
      v-if="withControls"
      class="max-sm:button-secondary button-inline-secondary text-7/8 duration-md sm:opacity-0 group-hover/trait:opacity-100"
      :disabled="isPendingUnequip"
      @click="unequip({
        tokenId: baseTokenId,
        contractAddress: constructorAddress,
        trait: {
          tokenId,
          address,
        },
      })"
    >
      unequip
    </button>
  </div>
</template>
