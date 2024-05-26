import BigNumber from 'bignumber.js';

/**
 * Parse denom value into units
 * @param amount BigNumber's input value - string, number or another BN instance
 * @param decimals Number of decimal places to shift by
 * @returns BigNumber's stringified representation parsed to units
 * @example ```js
 *  toAmountUnits('100', 3); // '100000'
 * ```
 */
export const toAmountUnits = (amount: BigNumber.Value, decimals = 18) => {
  return BigNumber(amount).shiftedBy(decimals).toString();
};

/**
 * Format units as denom value
 * @param amount BigNumber's input value - string, number or another BN instance
 * @param decimals Number of decimal places to shift by
 * @returns BigNumber's stringified representation formatted from units
 * @example ```js
 *  fromAmountUnits('100', 3); // '0.1'
 * ```
 */
export const fromAmountUnits = (amount: BigNumber.Value, decimals = 18) => {
  return BigNumber(amount).shiftedBy(-decimals).toString();
};

/**
 * Format units as denom value
 * @param amount BigNumber's input value - string, number or another BN instance
 * @param precision Number of decimal places show
 * @returns BigNumber's stringified representation, formatted with delimiters and trimmed decimals
 * @example ```js
 *  formatAmount('123456789.123456789', 3); // '123,456,789.123'
 * ```
 */
export const formatAmount = (amount: BigNumber.Value, precision?: number) => {
  return BigNumber(amount).toFormat(precision);
};
