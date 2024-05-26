<script setup lang="ts">
defineOptions({
  inheritAttrs: false,
});

const isLoading = useInjectLoading();
</script>

<template>
  <slot v-if="isLoading" name="fallback">
    <skeleton-element v-bind="$attrs" />
  </slot>
  <slot v-else />
</template>
