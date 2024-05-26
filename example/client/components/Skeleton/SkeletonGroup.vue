<script setup lang="ts">
const isLoading = useInjectLoading();
</script>

<template>
  <slot v-if="isLoading" name="fallback">
    <skeleton-element />
  </slot>
  <slot v-else />
</template>
