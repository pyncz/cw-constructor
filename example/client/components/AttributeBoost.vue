<script setup lang="ts">
import type { TraitAttribute } from '~/types';

const props = defineProps<TraitAttribute>();

const { variable: accentVar } = useAttributeAccentColor(() => props.trait_type);

const label = computed(() => {
  const signedValue = /^[+-].*/.test(props.value) ? props.value : `+${props.value}`;
  const units = props.display_type === 'boost_percentage' ? '%' : '';
  return `${signedValue}${units}`;
});

const tooltip = computed(() => `${label.value} to ${props.trait_type}`);
</script>

<template>
  <with-tooltip class="leading-sm text-[rgb(var(--accent))] font-light opacity-80 hover:opacity-100 !border-[rgb(var(--accent)_/_var(--tw-border-opacity))]" :tooltip :style="{ '--accent': accentVar }">
    {{ label }}
  </with-tooltip>
</template>
