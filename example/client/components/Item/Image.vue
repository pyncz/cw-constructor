<script setup lang="ts">
defineProps<{
  images?: string[]
}>();
</script>

<template>
  <div class="relative rounded">
    <img
      v-for="image of images"
      :key="image"
      class="absolute inset-0 size-full"
      :src="image"
      alt=""
    />
  </div>
</template>
