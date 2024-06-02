<script setup lang="ts">
defineProps<{
  svgClass?: string
}>();
</script>

<template>
  <div class="relative shrink-0 inline-flex items-center justify-center">
    <svg
      class="absolute inset-0"
      :class="svgClass"
      viewBox="0 0 100 100"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M75 6.69873L100 50L75 93.3013H25L0 50L25 6.69873L75 6.69873Z" fill="currentColor" />
    </svg>

    <div class="text-center">
      <slot />
    </div>
  </div>
</template>
