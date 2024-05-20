<script setup lang="ts">
const { isConnected } = useConnect();
</script>

<template>
  <div class="text-lg space-y-8">
    <img src="/img/placeholder.png" alt="Mysterious Fiend placeholder" class="w-full aspect-square rounded" />

    <div class="space-y-1">
      <button class="button-primary w-full font-semibold" :disabled="!isConnected">
        mint
      </button>

      <transition
        enter-active-class="duration-xs"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="duration-xs"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <p v-if="!isConnected" class="text-xs text-main-200">
          Connect first.
        </p>
      </transition>
    </div>
  </div>
</template>
