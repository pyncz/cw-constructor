import { computedAsync } from '@vueuse/core';

export const useComputedAsync = <T>(fn: (..._args: any[]) => Promise<T>) => {
  const dataAsync = computed(fn);

  const isLoading = ref(false);
  const isReady = computed(() => !isLoading.value);

  const data = computedAsync(() => dataAsync.value, undefined, { evaluating: isLoading });

  return {
    data,
    dataAsync,
    isLoading,
    isReady,
  };
};
