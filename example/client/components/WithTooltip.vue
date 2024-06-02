<script setup lang="ts">
defineProps<{
  tooltip: string
}>();
</script>

<template>
  <span :title="tooltip" class="duration-sm cursor-help border-b border-dashed border-main-100 border-opacity-50 hover:border-opacity-100">
    <slot />
  </span>
</template>
