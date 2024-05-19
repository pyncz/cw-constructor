<script setup lang="ts">
const wallets = useWallets();
const { connect, isConnecting, address, disconnect } = useConnect();
</script>

<template>
  <div v-if="address">
    <small class="text-main-200">
      Connected
      <a role="button" class="link" @click="disconnect()">Disconnect</a>
    </small>
    <div class="text-main-100 text-sm">
      <code>{{ address }}</code>
    </div>
  </div>
  <div v-else>
    <small class="text-main-200">Connect with</small>
    <div class="flex gap-4 px-px">
      <a
        v-for="w of wallets"
        :key="w.id"
        role="button"
        class="link text-sm"
        :disabled="isConnecting || !w.isInstalled"
        @click="connect(w)"
      >{{ w.name }}</a>
    </div>
  </div>
</template>
