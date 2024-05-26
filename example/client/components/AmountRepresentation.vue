<script setup lang="ts">
const props = defineProps<{
  amount: string
  denom: string
}>();

const { formatted, symbol } = useDenomAmount(() => props);

const tooltip = computed(() => `${props.amount}${props.denom}`);
</script>

<template>
  <div class="inline-flex leading-5 items-center gap-[0.5ch] border-b border-dashed duration-sm cursor-help border-main-200 hover:border-main-100" :title="tooltip">
    <p>{{ formatted }}</p>
    <small class="text-main-200 text-3/4">{{ symbol }}</small>
  </div>
</template>
