<script setup lang="ts">
const { status, payload } = useTxFlow();
</script>

<template>
  <div v-if="status" class="bg-main-800 py-2">
    <div class="container">
      <i>icon</i>

      <span v-if="payload?.title || payload?.description">
        <h5 v-if="payload.title">{{ payload.title }}</h5>
        <p v-if="payload.description">{{ payload.description }}</p>
      </span>
      <h5 v-else>
        <template v-if="status === ProgressStatus.Error">
          Transaction failed.
        </template>
        <template v-else-if="status === ProgressStatus.Success">
          Transaction complete!
        </template>
        <template v-else-if="status === ProgressStatus.Pending">
          Transaction pending...
        </template>
      </h5>

      <nuxt-link v-if="payload?.link" :href="payload.link.uri" class="link">
        {{ payload.link.label }}
      </nuxt-link>
      <button v-if="payload?.retry" class="link" @click="payload.retry()">
        Retry
      </button>
    </div>
  </div>
</template>
