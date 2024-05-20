import type { Ref } from 'vue';
import { ref } from 'vue';
import type { MaybePromise } from '~/types';

export const useAsync = <Returns, Params extends any[]>(
  fn: (..._a: Params) => MaybePromise<Returns>,
): {
  isPending: Ref<boolean>
  execute: (..._a: Params) => Promise<Returns>
} => {
  const isPending = ref(false);

  const execute = async (...args: Params) => {
    isPending.value = true;
    try {
      const res = await fn(...args);
      return res;
    } finally {
      isPending.value = false;
    }
  };

  return {
    isPending,
    execute,
  };
};
