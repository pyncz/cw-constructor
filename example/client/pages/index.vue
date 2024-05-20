<script setup lang="ts">
</script>

<template>
  <section class="grid gap-12 grid-cols-1 sm:grid-cols-2 md:grid-cols-[minmax(0,2fr)_minmax(0,3fr)]">
    <div>
      <mint-card class="mx-auto max-w-xs sm:my-16" />
    </div>
    <div class="space-y-5">
      <h3>Your frens</h3>
      <connected-only>
        <div class="grid gap-4 grid-cols-autofill-32">
          <div>hey</div>
          <div>hey</div>
          <div>hey</div>
        </div>
        <template #fallback>
          <div class="placeholder">
            Connect to see your frens.
          </div>
        </template>
      </connected-only>
    </div>
  </section>
</template>
