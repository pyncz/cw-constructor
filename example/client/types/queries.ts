export interface FetchOptions {
  retry?: MaybeRef<boolean | number | undefined>
}

export interface QueryOptions extends FetchOptions {
  throwOnError?: boolean
  enabled?: MaybeRef<boolean | undefined>
};
