<script setup lang="ts">
import type { Attribute, MergedAttribute } from '~/types';

const route = useRoute();
const tokenId = route.params.tokenId as string;

const { address } = useConnect();

const { info, config, baseInfo, mergedInfo, isLoading, owner } = useInfoByTokenId(tokenId);
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
  <div class="py-8 xs:py-12 sm:py-8 md:py-4 lg:py-0 [--w-col:30rem] 2xl:[--w-col:40rem] [--w-img:18rem] grid items-start grid-cols-1 lg:grid-cols-2 xl:grid-cols-[minmax(var(--w-col),1fr)_2fr] gap-8 lg:gap-12 xl:gap-16">
    <section class="lg:sticky top-0 lg:py-8 grid items-start grid-cols-1 md:grid-cols-[var(--w-img)_1fr] gap-x-12 gap-y-8 lg:gap-x-14 lg:gap-y-12">
      <div class="lg:col-span-2 2xl:col-span-1">
        <skeleton-group class="aspect-square w-full mx-auto md:mx-0 max-w-[--w-img]">
          <item-image :images="info?.extension?.images" class="w-full aspect-square mx-auto md:mx-0 max-w-[--w-img]" />
        </skeleton-group>
      </div>

      <div class="text-center md:text-start space-y-4 md:col-span-2">
        <h1>
          <skeleton-group class="max-w-48">
            {{ info?.extension?.name }}
            <span class="text-main-200 font-light ">#{{ tokenId }}</span>
          </skeleton-group>
        </h1>
        <p v-if="isLoading || info?.extension?.description" class="font-light text-sm text-main-100 max-w-lg mx-auto md:mx-0">
          <skeleton-group>
            {{ info?.extension?.description }}
            <template #fallback>
              <div class="space-y-[0.5em]">
                <skeleton-element />
                <skeleton-element />
                <skeleton-element class="w-1/2" />
              </div>
            </template>
          </skeleton-group>
        </p>
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

      <div class="w-full max-w-md md:max-w-xs mx-auto md:mx-0 space-y-3 md:col-span-2">
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

    <div class="lg:sticky top-0 lg:py-8 space-y-6">
      <div v-for="{ name, allowed_contracts: [traitAddress] } of config?.slots" :key="name" class="flex gap-4">
        <trait-mint-card :name :address="traitAddress" />
        <tokens-list
          :owner="address"
          :address="traitAddress"
          class="border border-main-300 flex-1"
          wrapper-class="overflow-auto flex gap-4 w-full border border-main-100 flex-1"
        >
          <template #default="{ tokenId: traitTokenId }">
            <trait-card :token-id="traitTokenId" :address="traitAddress" />
          </template>
          <template #skeleton>
            <skeleton-element class="aspect-square" />
          </template>
          <template #empty>
            <p class="text-main-200 flex items-center max-w-56 px-4 h-48 pt-6">
              No tokens owned for this trait.
            </p>
          </template>
        </tokens-list>
      </div>
    </div>
  </div>
</template>
