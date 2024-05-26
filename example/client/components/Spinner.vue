<script setup lang="ts">
import { useInterval } from '@vueuse/core';

const frames = '▖▌▘▀▝▐▗▄';

const counter = useInterval(100);
const index = computed(() => counter.value % frames.length);
</script>

<template>
  <span>
    {{ frames[index] }}
  </span>
</template>
