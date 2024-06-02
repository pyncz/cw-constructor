<script setup lang="ts">
import { IconHorns, IconWeapon, IconWings } from '../Icon';

const props = defineProps<{
  name: string
}>();

const icon = computed(() => {
  switch (props.name) {
    case 'horns':
      return IconHorns;
    case 'weapon':
      return IconWeapon;
    case 'wings':
      return IconWings;
  }
  return undefined;
});
</script>

<template>
  <component
    :is="icon"
    v-if="icon"
    :title="name"
  />
</template>
