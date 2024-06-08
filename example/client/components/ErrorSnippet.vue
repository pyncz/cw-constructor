<script setup lang="ts">
defineProps<{
  error: Error
}>();
</script>

<template>
  <pre class="whitespace-pre-line p-em text-main-200 bg-main-700 border border-main-600 rounded overflow-auto">
    {{ error.message ?? error }}
  </pre>
</template>
