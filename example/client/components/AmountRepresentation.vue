<script setup lang="ts">
const props = defineProps<{
  amount: string
  denom: string
}>();

const { formatted, symbol } = useDenomAmount(() => props);

const tooltip = computed(() => `${props.amount}${props.denom}`);
</script>

<template>
  <with-tooltip :tooltip>
    <div class="inline-flex leading-sm items-center gap-[0.5ch]">
      <p>{{ formatted }}</p>
      <small class="text-main-200 text-3/4">{{ symbol }}</small>
    </div>
  </with-tooltip>
</template>
