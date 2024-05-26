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

const AUTORESET_TIME = 8000;

export const useTxFlow = createSharedComposable(() => {
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

  const autoreset = () => {
    clearAutoresetTimeout();
    autoresetTimeout = setTimeout(reset, AUTORESET_TIME);
  };

  // Setters
  const setStart = (p?: Payload) => {
    status.value = ProgressStatus.Pending;
    payload.value = p;
    clearAutoresetTimeout();
  };

  const setError = (p?: Payload) => {
    status.value = ProgressStatus.Error;
    payload.value = p;
    autoreset();
  };

  const setSuccess = (p?: Payload) => {
    status.value = ProgressStatus.Success;
    payload.value = p;
    autoreset();
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
