<script setup lang="ts">
defineProps<{
  name: string
  withControls?: boolean
}>();
</script>

<template>
  <div class="flex flex-col items-start sm:flex-row gap-y-1 gap-x-2 group/trait">
    <trait-label :name class="flex-1 sm:flex-none">
      <slot />
    </trait-label>
    <button v-if="withControls" class="button-inline-secondary text-em sm:text-7/8 duration-md sm:opacity-0 group-hover/trait:opacity-100">
      unequip
    </button>
  </div>
</template>
