<script setup lang="ts">
const { author } = useAppConfig();
const years = useYearsFrom(2024);
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <div class="flex-1 flex flex-col">
      <header class="py-4 container flex gap-4 justify-between items-center">
        <nuxt-link to="/" class="shrink-0">
          <img src="/img/logo.png" alt="Fiend Frens logo" class="w-auto h-12 -ml-1 hidden xs:block" />
          <img src="/img/logo-symbol.png" alt="Fiend Frens symbol logo" class="size-12 xs:hidden" />
        </nuxt-link>
        <connect-section />
      </header>

      <main class="container py-4 flex-1 flex flex-col">
        <slot />
      </main>
    </div>

    <footer class="py-4 px-container">
      <small class="text-main-200">
        {{ years }}
        © by
        <a :href="author.github" target="_blank" class="link">{{ author.username }}</a>
      </small>
    </footer>
  </div>
</template>
