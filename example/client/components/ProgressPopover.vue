<script setup lang="ts">
const { status, payload, reset } = useProgressModal();
</script>

<template>
  <div class="fixed bottom-0 inset-x-0 p-2 sm:p-4 pointer-events-none flex justify-end">
    <transition
      enter-active-class="duration-md"
      enter-from-class="opacity-0 scale-75 translate-y-40"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="duration-md"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 scale-75 translate-y-40"
    >
      <div v-if="status" class="w-full xs:w-auto xs:min-w-64 max-w-full flex gap-em sm:max-w-96 bg-main-800 p-4 sm:py-2 rounded border border-main-700 pointer-events-auto text-sm text-main-100 leading-sm">
        <div class="size-em inline-flex items-center justify-center">
          <span v-if="status === ProgressStatus.Error">×</span>
          <span v-else-if="status === ProgressStatus.Success">✓</span>
          <spinner v-else-if="status === ProgressStatus.Pending" class="text-main-500" />
        </div>

        <div class="pt-px space-y-0.5 flex-1">
          <template v-if="payload?.title || payload?.description">
            <b v-if="payload.title">{{ payload.title }}</b>
            <p v-if="payload.description" class="text-main-200 text-xs">
              {{ payload.description }}
            </p>
          </template>
          <b v-else>
            <template v-if="status === ProgressStatus.Error">
              Transaction failed.
            </template>
            <template v-else-if="status === ProgressStatus.Success">
              Transaction complete!
            </template>
            <template v-else-if="status === ProgressStatus.Pending">
              Transaction pending...
            </template>
          </b>

          <error-snippet v-if="payload?.error" :error="payload?.error" class="text-7/8" />

          <div v-if="payload?.link || payload?.retry" class="flex gap-em text-xs py-0.5">
            <nuxt-link
              v-if="payload.link"
              :href="payload.link.uri"
              class="link"
              @click="reset()"
            >
              {{ payload.link.label }}
            </nuxt-link>
            <button v-if="payload.retry" class="link" @click="payload.retry()">
              Retry
            </button>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>
