/* eslint-disable camelcase */

import { useCw721Contract } from './useCw721Contract';
import { type CallOptions, type Coin, NoContractAddressError } from '~/types';

export const useCwMinterContract = (address?: MaybeRefOrGetter<string | undefined>) => {
  const { query, execute, executeMultiple } = useContract(address);

  const { contractInfo: cw721ContractInfo } = useCw721Contract();

  // Queries
  const contractInfo = async <T>(
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
    options?: CallOptions,
  ) => {
    const { cw721 } = await contractInfo(options);
    if (!cw721) {
      throw new NoContractAddressError('cw721 contract to mint is not set!');
    }
    const { name: collectionName } = await cw721ContractInfo({ contractAddress: cw721 });

    return await execute({
      mint: {},
    }, { ...options, memo: `Mint ${collectionName}` });
  };

  return {
    query,
    execute,
    executeMultiple,

    contractInfo,

    mint,
  };
};
