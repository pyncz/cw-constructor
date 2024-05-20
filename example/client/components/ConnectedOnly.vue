<script setup lang="ts">
const { isConnected } = useConnect();
</script>

<template>
  <client-only>
    <transition
      mode="out-in"
      enter-active-class="duration-xs"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="duration-xs"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <slot v-if="isConnected" />
      <slot v-else name="fallback" />
    </transition>
  </client-only>
</template>
