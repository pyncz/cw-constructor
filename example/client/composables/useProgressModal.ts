import { createSharedComposable } from '@vueuse/core';

export enum ProgressStatus {
  Pending = 'PENDING',
  Error = 'ERROR',
  Success = 'SUCCESS',
}

interface Payload {
  title?: string
  description?: string
  error?: Error
  link?: {
    label: string
    uri: string
  }
  retry?: () => void
};

interface Options {
  autoreset?: boolean | number
}

const DEFAULT_AUTORESET_TIME = 8000;

export const useProgressModal = createSharedComposable(() => {
  // State
  const status = ref<ProgressStatus>();
  const payload = ref<Payload>();

  // Autoreset helpers
  let autoresetTimeout: NodeJS.Timeout | undefined;
  const clearAutoresetTimeout = () => {
    if (autoresetTimeout) {
      clearTimeout(autoresetTimeout);
      autoresetTimeout = undefined;
    }
  };

  const reset = () => {
    status.value = undefined;
    payload.value = undefined;
    clearAutoresetTimeout();
  };

  const autoreset = (timeout = DEFAULT_AUTORESET_TIME) => {
    clearAutoresetTimeout();
    autoresetTimeout = setTimeout(reset, timeout);
  };

  // Setters
  const setStart = (p?: Payload) => {
    status.value = ProgressStatus.Pending;
    payload.value = p;
    clearAutoresetTimeout();
  };

  const setError = (p?: Payload, options?: Options) => {
    status.value = ProgressStatus.Error;
    payload.value = p;
    if (options?.autoreset) {
      autoreset(options.autoreset === true ? undefined : options.autoreset);
    }
  };

  const setSuccess = (p?: Payload, options?: Options) => {
    status.value = ProgressStatus.Success;
    payload.value = p;
    if (options?.autoreset) {
      autoreset(options.autoreset === true ? undefined : options.autoreset);
    }
  };

  return {
    status,
    payload,

    setStart,
    setError,
    setSuccess,
    reset,
  };
});
