<script setup lang="ts">
defineProps<{
  reversed?: boolean
  label?: string
}>();
</script>

<template>
  <dl class="flex overflow-hidden" :class="reversed ? 'flex-col-reverse' : 'flex-col'">
    <dt class="truncate text-3/4 text-main-200">
      <slot name="label">
        {{ label }}
      </slot>
    </dt>
    <dd class="truncate">
      <slot />
    </dd>
  </dl>
</template>
