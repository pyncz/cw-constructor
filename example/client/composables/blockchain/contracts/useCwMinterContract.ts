/* eslint-disable camelcase */

import { useCw721Contract } from './useCw721Contract';
import { type CallOptions, type Coin, type ExecuteOptions, NoContractAddressError } from '~/types';

export const useCwMinterContract = (address?: MaybeRefOrGetter<string | undefined>) => {
  const { query, execute, executeMultiple } = useContract(address);

  const { contractInfo: cw721ContractInfo } = useCw721Contract();

  // Queries
  const contractInfo = async <T = any>(
    options?: CallOptions,
  ): Promise<{
    cw721?: string | null
    supply?: number | null
    price?: Coin | null
    extensions: {
      weight: number
      value: T
    }[]
    admins: string[]
  }> => {
    return await query({
      contract_info: {},
    }, options);
  };

  // Transactions
  const mint = async (
    options?: ExecuteOptions,
  ) => {
    const { cw721, price } = await contractInfo(options);
    if (!cw721) {
      throw new NoContractAddressError('cw721 contract to mint is not set!');
    }
    const { name: collectionName } = await cw721ContractInfo({ contractAddress: cw721 });

    return await execute({
      mint: {},
    }, { ...options, memo: `Mint ${collectionName}${price ? ` for ${price.amount}${price.denom}` : ''}` });
  };

  return {
    query,
    execute,
    executeMultiple,

    contractInfo,

    mint,
  };
};
