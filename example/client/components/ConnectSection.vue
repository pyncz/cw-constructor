<script setup lang="ts">
import { GAS_ADJUSTMENT } from '~/configs';

const wallets = useWallets();
const { connect, isConnecting, address, disconnect } = useConnect();

const { formatted: formattedAddress } = useAddress(address);
</script>

<template>
  <div v-if="address" class="flex flex-col items-end">
    <small class="text-main-200">
      Connected
      <a role="button" class="link" @click="disconnect()">Disconnect</a>
    </small>
    <div class="text-main-100 text-sm">
      <code>{{ formattedAddress }}</code>
    </div>
  </div>
  <div v-else class="flex flex-col items-end">
    <small class="text-main-200">Connect with</small>
    <div class="text-main-100 flex gap-4 px-px">
      <a
        v-for="w of wallets"
        :key="w.id"
        role="button"
        class="link text-sm text-main-100 hover:text-main-0"
        :disabled="isConnecting || !w.isInstalled"
        @click="connect(w, { gasAdjustment: GAS_ADJUSTMENT })"
      >{{ w.name }}</a>
    </div>
  </div>
</template>
