<script setup lang="ts">
// Restore connection on mounted
const { reconnect } = useConnect();
onMounted(() => {
  reconnect({ gasAdjustment: GAS_ADJUSTMENT });
});

// Meta
useHeadSafe({
  htmlAttrs: {
    lang: 'en',
  },
  link: [
    {
      rel: 'icon',
      type: 'image/x-icon',
      href: '/favicon.ico',
    },
  ],
});

// SEO
const { public: { appUri } } = useRuntimeConfig();
const { name, description, author } = useAppConfig();
defineOgImage({
  url: `${appUri}/img/og.jpg`,
});

useSeoMeta({
  // page params
  title: name,

  // seo title
  ogTitle: name,
  twitterTitle: name,
  // seo description
  description,
  ogDescription: description,
  twitterDescription: description,

  // other meta
  ogUrl: appUri,
  applicationName: name,
  twitterCard: 'summary_large_image',
  twitterSite: author.twitterHandle,
  themeColor: '#0B0A10',
});
</script>

<template>
  <nuxt-layout>
    <nuxt-page />
  </nuxt-layout>
</template>
