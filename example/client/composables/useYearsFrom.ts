/**
 * Make string like `2069-2420`, from provided year till the current one
 * or just year if they are the same.
 *
 * @param start Year to start from (a number or a ref)
 *
 * @returns Years string
 */
export const useYearsFrom = (
  start?: MaybeRefOrGetter<number | undefined>,
) => {
  const currentYear = new Date().getFullYear();

  return computed(() => {
    const startYear = toValue(start);
    return startYear
      ? startYear + (currentYear > startYear ? `-${currentYear}` : '')
      : currentYear.toString();
  });
};
