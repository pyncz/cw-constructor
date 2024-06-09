<script setup lang="ts">
const { chainId, config: chainConfig } = useChain();

const { address, isConnecting } = useConnect();

// Fetch constructor setup to get base token address
const { data: constructorConfig, isLoading: isLoadingConfig } = UseCwConstructorConfig.useQuery();

const isLoading = computed(() => isConnecting.value || isLoadingConfig.value);
useProvideLoading(isLoading);
</script>

<template>
  <section class="grid gap-12 grid-cols-1 md:grid-cols-2 lg:grid-cols-[minmax(0,2fr)_minmax(0,3fr)]">
    <div>
      <item-mint-card :address="constructorConfig?.base_token" class="mx-auto max-w-xs md:my-16" />
    </div>
    <div class="space-y-5">
      <h3>Your frens</h3>
      <connected-only>
        <tokens-list :address="constructorConfig?.base_token" :owner="address">
          <template #default="{ tokenId }">
            <item-image-card :token-id />
          </template>
          <template #empty>
            <div class="placeholder">
              You don't have any frens ;(
            </div>
          </template>
          <template #skeleton>
            <div class="aspect-square p-1">
              <skeleton-element class="rounded h-full" />
            </div>
          </template>
        </tokens-list>
        <template #fallback>
          <div class="placeholder text-start space-y-4">
            <div class="space-y-4 max-w-xl">
              <h6 class="text-main-100">
                Connect to see your frens.
              </h6>
              <p class="text-7/8">
                Smart contracts for this dapp are deployed on <code class="whitespace-nowrap">{{ chainId }}</code>.
                This is a <a class="link" href="https://cosmos.network" target="_blank">Cosmos Blockchain</a>, so no EVM-based wallets here, sorry!
              </p>
              <p class="text-7/8">
                To get started:
              </p>
            </div>
            <ol class="pl-8 text-7/8 list-decimal space-y-1" start="0">
              <li>
                Install
                <template v-for="(wallet, i) of wallets" :key="wallet.id">
                  {{ i > 0 ? i === wallets.length - 1 ? ' or ' : ', ' : '' }}
                  <a class="link" :href="wallet.download" target="_blank">{{ wallet.name }}</a>
                </template>
                browser extension.
              </li>
              <li>
                Create an account there.
              </li>
              <li>
                Make sure you have <code class="whitespace-nowrap">{{ chainId }}</code> chain added.
              </li>
              <li v-if="chainConfig.faucet">
                Use <a class="link" :href="chainConfig.faucet" target="_blank">this faucet</a> in order to get some {{ chainConfig.symbol }} tokens.
              </li>
              <li v-else-if="chainConfig.exchange">
                You can get some {{ chainConfig.symbol }} tokens <a class="link" :href="chainConfig.exchange" target="_blank">here</a>.
              </li>
              <li>
                Connect.
              </li>
              <li>
                ?????
              </li>
              <li>
                You're ready to make some fiend frens!
              </li>
            </ol>
          </div>
        </template>
      </connected-only>
    </div>
  </section>
</template>
