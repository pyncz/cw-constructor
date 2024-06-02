export const useAddress = (address: MaybeRefOrGetter<string | undefined>) => {
  const formatted = computed(() => {
    const value = toValue(address);
    return value ? truncateAddress(value) : undefined;
  });

  return { formatted };
};
