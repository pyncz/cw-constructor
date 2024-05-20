/* eslint-disable camelcase */

import type { Cw721ContractInfo, NftInfoResponse } from './useCw721Contract';
import { useCw721Contract } from './useCw721Contract';
import type { CallOptions } from '~/types';

export interface TokenInfo<T = any> {
  contract: Cw721ContractInfo
  token: NftInfoResponse<T>
}

export interface TokenConfig {
  address: string
  token_id: string
}

export const useCwConstructorContract = (address?: MaybeRefOrGetter<string | undefined>) => {
  const { query, execute, executeMultiple } = useContract(address);

  const { nftTitle } = useCw721Contract();

  // Queries
  const contractInfo = async (
    options?: CallOptions,
  ): Promise<{
    base_token: string
    slots: {
      name: string
      allowed_contracts: string[]
      allow_multiple: boolean
    }[]
    admins: string[]
  }> => {
    return await query({
      contract_info: {},
    }, options);
  };

  const traits = async (
    { slot, tokenId }: { slot?: string, tokenId?: string },
    options?: CallOptions,
  ): Promise<{
    traits: {
      token_id: string
      token: TokenConfig
      slot: string
    }[]
  }> => {
    return await query({
      traits: {
        token_id: tokenId,
        slot,
      },
    }, options);
  };

  const tokens = async (
    { address, tokenId }: { address?: string, tokenId?: string },
    options?: CallOptions,
  ): Promise<{
    tokens: string[]
  }> => {
    return await query({
      tokens: {
        address,
        token_id: tokenId,
      },
    }, options);
  };

  const tokenInfo = async <E = any, T = any, M = any>({ tokenId }: { tokenId: string }, options?: CallOptions): Promise<{
    info: NftInfoResponse<M>
    base_token: TokenInfo<E>
    traits: {
      token_id: string
      token: TokenConfig
      slot: string
      info: TokenInfo<T>
    }[]
  }> => {
    return await query({
      info: {
        token_id: tokenId,
      },
    }, options);
  };

  // Transactions
  const equip = async (
    { tokenId, trait }: { tokenId: string, trait: { address: string, tokenId: string } },
    options?: CallOptions,
  ) => {
    const traitTokenTitle = await nftTitle({ tokenId: trait.tokenId }, { contractAddress: trait.address });
    return await execute({
      equip: {
        token_id: tokenId,
        traits: [{
          address: trait.address,
          token_id: trait.tokenId,
        }],
      },
    }, { ...options, memo: `Equip ${traitTokenTitle} on #${tokenId}` });
  };

  const unequip = async (
    { trait }: { trait: { address: string, tokenId: string } },
    options?: CallOptions,
  ) => {
    const traitTokenTitle = await nftTitle({ tokenId: trait.tokenId }, { contractAddress: trait.address });
    return await execute({
      unequip: {
        traits: [{
          address: trait.address,
          token_id: trait.tokenId,
        }],
      },
    }, { ...options, memo: `Unequip ${traitTokenTitle}` });
  };

  return {
    query,
    execute,
    executeMultiple,

    contractInfo,
    traits,
    tokens,
    tokenInfo,

    equip,
    unequip,
  };
};
