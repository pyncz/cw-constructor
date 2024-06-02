<script setup lang="ts">
import type { Attribute, MergedAttribute } from '~/types';

const props = defineProps<{
  attribute: MergedAttribute
  baseAttribute?: Attribute
}>();

const { rgb: accent } = useAttributeAccentColor(() => props.attribute.trait_type);
</script>

<template>
  <div class="gap-em flex items-center" :style="{ '--accent': accent }">
    <shape-hex class="-ml-1 font-comic size-14 text-[--accent]" svg-class="fill-[--accent] opacity-[--accents-opacity]">
      <span class="text-xl relative top-0.5">{{ attribute.value }}</span>
    </shape-hex>

    <attribute-record reversed>
      {{ attribute.trait_type }}

      <template #label>
        <span v-if="baseAttribute" class="text-[--accent]">
          base {{ baseAttribute.value }}
        </span>
      </template>
    </attribute-record>
  </div>
</template>
