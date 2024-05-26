import type { MaybePromise } from './utils';

type Cb<T = void> = (_data: T) => MaybePromise<void>;

export interface CallbackOptions<T = any> {
  onSuccess?: Cb<T>
  onError?: (_error: unknown) => MaybePromise<void>
  onSettled?: Cb
}
