import { useInfiniteQuery } from '@tanstack/vue-query';
import type { ArgumentsType, DeepMaybeRef } from '@vueuse/core';
import type { QueryOptions } from '~/types';

type Method = ReturnType<typeof useCw721Contract>['tokens'];
type FullMsg = Partial<ArgumentsType<Method>[0]>;

type Msg = Omit<FullMsg, 'start_after' | 'limit'>;
type Input = DeepMaybeRef<Msg> & {
  limit?: FullMsg['limit']
};

type MethodsResponse = Awaited<ReturnType<Method>>;
type PageParam = string | undefined;
type PageResponse = MethodsResponse & {
  nextCursor?: string
};

export const UseCw721TokensInfinite = {
  getKey: (
    address: MaybeRefOrGetter<string | undefined>,
    msg: Input,
  ) => {
    return [...UseCw721Tokens.getKey(address, msg), 'infinite'];
  },

  useQuery: (
    address: MaybeRefOrGetter<string | undefined>,
    msg: Input,
    options?: QueryOptions,
  ) => {
    const { tokens } = useCw721Contract();
    const limit = msg.limit ?? DEFAULT_PAGE_LIMIT;

    const state = useInfiniteQuery({
      ...options,
      queryKey: UseCw721TokensInfinite.getKey(address, msg),
      queryFn: async ({ pageParam }) => {
        const res = await tokens({
          owner: toValue(msg.owner)!,
          // eslint-disable-next-line camelcase
          start_after: pageParam,
          // Query additional one to check if there's a next page
          limit: limit + 1,
        }, {
          contractAddress: toValue(address)!,
        });
        const hasNextPage = !!res.tokens.at(limit);
        return {
          tokens: res.tokens.slice(0, limit), // fiter out that additional +1
          nextCursor: hasNextPage ? res.tokens.at(limit - 1) : undefined,
        };
      },
      initialPageParam: undefined as PageParam,
      getNextPageParam: (lastPage: PageResponse) => lastPage.nextCursor,
      enabled: () => !!toValue(address) && !!toValue(msg.owner) && (toValue(options?.enabled) ?? true),
    });

    const pages = computed(() => state.data.value?.pages);
    const pageParams = computed(() => state.data.value?.pageParams);

    const data = computed(() => pages.value ? { tokens: pages.value.flatMap((page) => page.tokens) } : undefined);

    return {
      ...state,
      pages,
      pageParams,
      data,
    };
  },
};
