<script setup lang="ts">
defineProps<{
  bottomLabel?: boolean
  label?: string
}>();
</script>

<template>
  <dl class="flex" :class="bottomLabel ? 'flex-col' : 'flex-col-reverse'">
    <dd class="truncate">
      <slot />
    </dd>
    <dt class="truncate text-3/4 text-main-200">
      <slot name="label">
        {{ label }}
      </slot>
    </dt>
  </dl>
</template>
