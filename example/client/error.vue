<script setup lang="ts">
import type { NuxtError } from 'nuxt/app';

defineProps<{
  error: NuxtError
}>();
</script>

<template>
  <NuxtLayout>
    <div class="space-y-2">
      <div>
        <h5>
          {{ error.statusCode }}
        </h5>
        <pre class="text-main-100">{{ error.message }}</pre>
      </div>
      <div>
        <nuxt-link to="/" class="link">
          Go back
        </nuxt-link>
      </div>
    </div>
  </NuxtLayout>
</template>
