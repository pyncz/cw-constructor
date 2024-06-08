<script setup lang="ts">
import type { Attribute, MergedAttribute } from '~/types';

const route = useRoute();
const tokenId = route.params.tokenId as string;

const { address } = useConnect();

const { suspense, info, config, baseInfo, mergedInfo, traitsInfoBySlots, isLoading, owner } = useInfoByTokenId(tokenId);
onServerPrefetch(async () => {
  let prefetchedData: any;
  try {
    const res = await suspense();
    prefetchedData = res.data;
  } catch (e) {}

  if (!prefetchedData) {
    throw createNuxtNotFoundError();
  }
});

useProvideLoading(isLoading);

const isOwnedByCurrentUser = computed(() => owner.value ? owner.value === address.value : undefined);

const numericAttributes = computed(() => {
  return info.value?.extension?.attributes.reduce((attrs, attr) => {
    if (attr.display_type) {
      const base = baseInfo.value?.extension?.attributes.find((a) => a.trait_type === attr.trait_type);
      attrs.push({ attr, base });
    }
    return attrs;
  }, [] as { attr: MergedAttribute, base?: Attribute }[]);
});
</script>

<template>
  <div class="py-8 xs:py-12 sm:py-8 md:py-4 lg:py-0 [--w-col:30rem] 2xl:[--w-col:40rem] [--w-img:18rem] grid items-start grid-cols-1 lg:grid-cols-2 xl:grid-cols-[minmax(var(--w-col),1fr)_2fr] gap-12 sm:gap-16 lg:gap-12 xl:gap-16">
    <section class="lg:sticky top-0 lg:py-8 grid items-start grid-cols-1 md:grid-cols-[var(--w-img)_1fr] gap-x-12 gap-y-8 lg:gap-x-14">
      <div class="lg:col-span-2 2xl:col-span-1">
        <skeleton-group class="aspect-square w-full mx-auto md:mx-0 max-w-[--w-img]">
          <item-image :images="info?.extension?.images" class="w-full aspect-square mx-auto md:mx-0 max-w-[--w-img]" />
        </skeleton-group>
      </div>

      <div class="text-center md:text-start space-y-4 md:col-span-2 lg:mt-4">
        <h1>
          <skeleton-group class="max-w-48 mx-auto md:mx-0">
            {{ info?.extension?.name }}
            <span class="text-main-200 font-light ">#{{ tokenId }}</span>
          </skeleton-group>
        </h1>
        <p v-if="isLoading || info?.extension?.description" class="font-light text-sm text-main-100 max-w-lg mx-auto md:mx-0">
          <skeleton-group>
            {{ info?.extension?.description }}
            <template #fallback>
              <div class="gap-[0.5em] flex flex-col items-center md:items-start">
                <skeleton-element class="w-full" />
                <skeleton-element class="w-full" />
                <skeleton-element class="w-1/2" />
              </div>
            </template>
          </skeleton-group>
        </p>
        <attribute-record v-if="address && !isOwnedByCurrentUser" label="Owned by">
          Not you.
        </attribute-record>
      </div>

      <div class="w-full max-w-md md:max-w-xs mx-auto md:mx-0 md:row-start-1 md:col-start-2 lg:col-span-2 lg:row-start-auto lg:col-start-auto 2xl:col-span-1 2xl:row-start-1 2xl:col-start-2">
        <skeleton-group>
          <item-attribute-numeric
            v-for="{ attr, base } of numericAttributes"
            :key="attr.trait_type"
            :attribute="attr"
            :base-attribute="base"
            class="border-b border-main-700 py-2 md:py-1"
          />
          <template #fallback>
            <div class="space-y-4 md:space-y-2 py-2 md:py-1">
              <item-attribute-numeric-skeleton />
              <item-attribute-numeric-skeleton />
              <item-attribute-numeric-skeleton />
            </div>
          </template>
        </skeleton-group>
      </div>

      <div v-if="mergedInfo?.traits.length" class="w-full max-w-md md:max-w-xs mx-auto md:mx-0 space-y-3 md:col-span-2">
        <h6 class="text-main-200">
          Equipped
        </h6>
        <div>
          <item-trait
            v-for="trait of mergedInfo?.traits"
            :key="trait.slot"
            :name="trait.slot"
            class="border-b border-main-700 py-2 md:py-1 px-3"
            :with-controls="isOwnedByCurrentUser"
          >
            <attribute-record :label="trait.slot" reversed>
              {{ trait.info.token.extension?.name }}
            </attribute-record>
          </item-trait>
        </div>
      </div>
    </section>

    <div class="lg:sticky top-0 lg:py-8 space-y-6 sm:space-y-4 lg:space-y-2 overflow-hidden -mx-container lg:mx-0">
      <div
        v-for="{
          name,
          allowed_contracts: [traitAddress],
          allow_multiple: allowMultiple,
        } of config?.slots"
        :key="name"
        class="flex flex-col xs:flex-row border-y border-main-900 xs:border-y-0"
      >
        <trait-mint-card
          :name
          :address="traitAddress"
          class="w-full xs:w-24 xs:min-h-[18.75rem] !bg-main-900/20 xs:!bg-main-800"
        />
        <tokens-list
          :owner="address"
          :address="traitAddress"
          class="flex-1 px-4 overflow-hidden bg-main-900/50 xs:bg-transparent pb-3"
          grid-class="overflow-auto flex xs:px-4"
          wrapper-class="!w-[calc(100%+2*theme(spacing.4))] -mx-4 border-b border-main-800"
        >
          <template #default="{ tokenId: traitTokenId }">
            <trait-card
              :token-id="traitTokenId"
              :address="traitAddress"
              :base-token-id="tokenId"
              :slot-taken="traitsInfoBySlots[name]?.length && !allowMultiple"
              class="min-h-60"
            />
          </template>
          <template #skeleton>
            <skeleton-element class="aspect-square" />
          </template>
          <template #empty>
            <p class="text-main-200 flex items-center max-w-64 px-8 h-60">
              No tokens owned for this trait.
            </p>
          </template>
        </tokens-list>
      </div>
    </div>
  </div>
</template>
