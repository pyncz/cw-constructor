import { injectLocal } from '@vueuse/core';
import { IS_LOADING_INJECTION_KEY } from './consts';

export const useInjectLoading = () => {
  return injectLocal(IS_LOADING_INJECTION_KEY, ref(false));
};
