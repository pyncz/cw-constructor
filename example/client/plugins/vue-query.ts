import type { DehydratedState, VueQueryPluginOptions } from '@tanstack/vue-query';
import { QueryClient, VueQueryPlugin, dehydrate, hydrate } from '@tanstack/vue-query';

export default defineNuxtPlugin((nuxt) => {
  const vueQueryState = useState<DehydratedState | null>('vue-query');

  // Modify your Vue Query global settings here
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: 60_000, // 1 min
        networkMode: 'offlineFirst',
        throwOnError: process.env.NODE_ENV === 'development',
      },
    },
  });
  const options: VueQueryPluginOptions = { queryClient };

  nuxt.vueApp.use(VueQueryPlugin, options);

  if (process.server) {
    nuxt.hooks.hook('app:rendered', () => {
      vueQueryState.value = dehydrate(queryClient);
    });
  }

  if (process.client) {
    nuxt.hooks.hook('app:created', () => {
      hydrate(queryClient, vueQueryState.value);
    });
  }

  return {
    provide: { queryClient },
  };
});
