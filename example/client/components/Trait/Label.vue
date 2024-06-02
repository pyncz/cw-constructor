<script setup lang="ts">
defineProps<{
  name: string
}>();
</script>

<template>
  <div class="inline-flex gap-[0.5em]">
    <trait-icon :name class="size-[1.5em] relative -top-0.5" />
    <div>
      <slot>{{ name }}</slot>
    </div>
  </div>
</template>
