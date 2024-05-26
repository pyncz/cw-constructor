import { provideLocal } from '@vueuse/core';
import { IS_LOADING_INJECTION_KEY } from './consts';

export const useProvideLoading = (isLoading: MaybeRefOrGetter<boolean>) => {
  provideLocal(IS_LOADING_INJECTION_KEY, toRef(isLoading));
};
