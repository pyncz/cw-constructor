<script setup lang="ts">
import type { TraitExtension } from '~/types';

defineOptions({
  inheritAttrs: false,
});

const props = defineProps<{
  address: string
  tokenId: string
  baseTokenId: string
  slotTaken?: boolean
}>();

const { address, tokenId } = toRefs(props);

const { public: { constructorAddress } } = useRuntimeConfig();
const { data: info, isLoading: isLoadingInfo } = UseCw721AllNftInfo.useQuery<TraitExtension>(address, {
  tokenId,
});
const { data: tokensData, isLoading: isLoadingBaseTokens } = UseCwConstructorTokens.useQuery({ address, tokenId }, { throwOnError: false });
const equippedOn = computed(() => tokensData.value?.tokens.at(0));

const isEquipped = computed(() => !!tokensData.value?.tokens.length);
const isEquippedOnCurrent = computed(() => props.baseTokenId === equippedOn.value);

const isLoading = computed(() => isLoadingInfo.value || isLoadingBaseTokens.value);
useProvideLoading(isLoading);

const images = computed(() => [
  '/img/template.png',
  ...(info.value?.info.extension?.image
    ? [info.value.info.extension.image]
    : []),
]);

const numericAttributes = computed(() => info.value?.info.extension?.attributes.filter((a) => !!a.display_type));
const nameTooltip = computed(() => `${info.value?.info.extension?.name} #${props.tokenId}`);

const { address: userAddress } = useConnect();
const isOwnedByCurrentUser = computed(() => {
  return info.value
    ? userAddress.value === info.value.access.owner
    : undefined;
});

// Actions
const { mutate: equip, isPending: isPendingEquip } = useCwConstructorEquipMutation();
const { mutate: unequip, isPending: isPendingUnequip } = useCwConstructorUnequipMutation();
const isPending = computed(() => isPendingEquip.value || isPendingUnequip.value);
</script>

<template>
  <skeleton-group>
    <div
      v-bind="$attrs"
      class="card group/card flex flex-col gap-3"
      :class="{ frame: isEquippedOnCurrent }"
    >
      <item-image :images class="w-full aspect-square" />

      <div class="flex-1 flex flex-col gap-3">
        <div class="flex-1 space-y-1 -mx-0.5">
          <div class="duration-sm leading-xs text-main-100 group-hover/card:text-main-0 break-words line-clamp-2" :title="nameTooltip">
            {{ info?.info.extension?.name }}
          </div>
          <div class="flex gap-2 text-7/8">
            <attribute-boost v-for="attr of numericAttributes" :key="attr.trait_type" v-bind="attr" />
          </div>
        </div>

        <div class="text-7/8 font-medium">
          <template v-if="isOwnedByCurrentUser">
            <button
              v-if="isEquippedOnCurrent"
              :disabled="isPending"
              class="button-secondary w-full"
              @click="unequip({
                tokenId: equippedOn,
                contractAddress: constructorAddress,
                trait: {
                  tokenId,
                  address,
                },
              })"
            >
              unequip
            </button>
            <button
              v-else-if="isEquipped"
              class="button-secondary w-full"
              disabled
            >
              <with-tooltip :tooltip="`Equipped on #${equippedOn}`">
                in use
              </with-tooltip>
            </button>
            <button
              v-else
              class="button-primary w-full"
              :disabled="slotTaken || isPending"
              @click="equip({
                tokenId: baseTokenId,
                contractAddress: constructorAddress,
                trait: {
                  tokenId,
                  address,
                },
              })"
            >
              equip
            </button>
          </template>
          <div v-else class="button-secondary w-full" disabled>
            {{ isEquipped ? isEquippedOnCurrent ? 'equipped' : 'in use' : 'not equipped' }}
          </div>
        </div>
      </div>
    </div>

    <template #fallback>
      <div class="p-4 w-36 shrink-0" v-bind="$attrs">
        <skeleton-element class="h-full" />
      </div>
    </template>
  </skeleton-group>
</template>
