<script setup lang="ts">
import IconHorns from '../Icon/Horns.vue';
import IconWeapon from '../Icon/Weapon.vue';
import IconWings from '../Icon/Wings.vue';

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
