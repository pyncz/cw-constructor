import { env } from './env';

export default defineNuxtConfig({
  css: ['@/assets/css/fonts.css'],
  modules: ['nuxt-og-image'],
  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },
  runtimeConfig: {
    public: {
      ...env,
    },
  },
});
