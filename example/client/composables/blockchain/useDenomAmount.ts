import type { Coin } from '~/types';
import { DEFAULT_AMOUNT_DISPLAY_PRECISION } from '~/configs';
import { formatAmount } from '~/utils';

export const useDenomAmount = (coin: MaybeRefOrGetter<Coin | undefined>) => {
  // TODO: Fetch metadata (incl. decimals aka "exponent") from `/cosmos/bank/v1beta1/denoms_metadata/{denom}`?
  const { config: { symbol: nativeSymbol } } = useChain();
  const denomConfig = {
    decimals: 18,
    symbol: nativeSymbol,
  };
  const { decimals, symbol } = toRefs(denomConfig);

  /**
   * Converted from base denom to main coin amount
   */
  const amount = computed(() => {
    const amountInBaseDenom = toValue(coin)?.amount;
    return amountInBaseDenom ? fromAmountUnits(amountInBaseDenom) : undefined;
  });

  const formatted = computed(() => {
    return amount.value ? formatAmount(amount.value, DEFAULT_AMOUNT_DISPLAY_PRECISION) : undefined;
  });

  return {
    decimals,
    amount,
    symbol,
    formatted,
  };
};
