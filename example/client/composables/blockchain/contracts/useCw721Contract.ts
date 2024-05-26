/* eslint-disable camelcase */

import type { CallOptions, CwCursorPagination, Expiration } from '~/types';

export interface Approval {
  spender: string
  expires: Expiration
}

export interface OwnerOfResponse {
  owner: string
  approvals: Approval[]
}

export interface NftInfoResponse<T = any> {
  token_uri: null | string
  extension: null | T
}

export interface Cw721ContractInfo {
  name: string
  symbol: string
}

export const useCw721Contract = (address?: MaybeRefOrGetter<string | undefined>) => {
  const { query, execute, executeMultiple } = useContract(address);

  // Queries
  const contractInfo = async (
    options?: CallOptions,
  ): Promise<Cw721ContractInfo> => {
    return await query({
      contract_info: {},
    }, options);
  };

  const minter = async (
    options?: CallOptions,
  ): Promise<{
    minter?: string
  }> => {
    return await query({
      minter: {},
    }, options);
  };

  const ownerOf = async (
    { tokenId }: { tokenId: string },
    options?: CallOptions,
  ): Promise<OwnerOfResponse> => {
    return await query({
      owner_of: {
        token_id: tokenId,
        include_expired: false,
      },
    }, options);
  };

  const approval = async (
    { tokenId, spender }: { tokenId: string, spender: string },
    options?: CallOptions,
  ): Promise<{
    approval?: Approval
  }> => {
    // Return empty object if there's no approval
    try {
      return await query({
        approval: {
          token_id: tokenId,
          spender,
          include_expired: false,
        },
      }, options);
    } catch (e) {
      return {};
    }
  };

  const approvals = async (
    { tokenId }: { tokenId: string },
    options?: CallOptions,
  ): Promise<{
    approvals: Approval[]
  }> => {
    return await query({
      approvals: {
        token_id: tokenId,
        include_expired: false,
      },
    }, options);
  };

  const allOperators = async (
    { owner, limit, start_after }: CwCursorPagination & { owner: string },
    options?: CallOptions,
  ): Promise<{
    operators: Approval[]
  }> => {
    return await query({
      all_operators: {
        owner,
        include_expired: false,
        start_after,
        limit,
      },
    }, options);
  };

  const operator = async (
    { owner, operator }: { owner: string, operator: string },
    options?: CallOptions,
  ): Promise<{
    approval?: Approval
  }> => {
    const { operators } = await allOperators({ owner }, options);
    const approval = operators.find((o) => o.spender === operator);
    return { approval };
  };

  const numTokens = async (
    options?: CallOptions,
  ): Promise<{
    count: number
  }
  > => {
    return await query({
      num_tokens: {},
    }, options);
  };

  const tokens = async (
    { owner, start_after, limit }: CwCursorPagination & { owner: string },
    options?: CallOptions,
  ): Promise<{
    tokens: string[]
  }> => {
    return await query({
      tokens: {
        owner,
        start_after,
        limit,
      },
    }, options);
  };

  const allTokens = async (
    payload?: CwCursorPagination,
    options?: CallOptions,
  ): Promise<{
    tokens: string[]
  }> => {
    const { start_after, limit } = payload ?? {};
    return await query({
      all_tokens: {
        start_after,
        limit,
      },
    }, options);
  };

  // - metadata queries
  const nftInfo = async <T = any>(
    { tokenId }: { tokenId: string },
    options?: CallOptions,
  ): Promise<NftInfoResponse<T>> => {
    return await query({
      nft_info: {
        token_id: tokenId,
      },
    }, options);
  };

  const allNftInfo = async <T = any>(
    { tokenId }: { tokenId: string },
    options?: CallOptions,
  ): Promise<{
    info: NftInfoResponse<T>
    access: OwnerOfResponse
  }> => {
    return await query({
      all_nft_info: {
        token_id: tokenId,
      },
    }, options);
  };

  // - aggregated queries
  const nftTitle = async (
    { tokenId }: { tokenId: string },
    options?: CallOptions,
  ): Promise<string> => {
    const { extension } = await nftInfo({ tokenId }, options);
    const name = (extension as any)?.name;
    if (name) {
      return name;
    }
    const { name: collectionName } = await contractInfo(options);
    return `${collectionName} #${tokenId}`;
  };

  // Transactions
  const transferNft = async (
    { recipient, tokenId }: { recipient: string, tokenId: string },
    options?: CallOptions,
  ) => {
    const title = await nftTitle({ tokenId }, options);
    return await execute({
      transfer_nft: {
        recipient,
        token_id: tokenId,
      },
    }, { ...options, memo: `Transfer ${title} to ${recipient}` });
  };

  const sendNft = async (
    { contract, tokenId, msg }: { contract: string, tokenId: string, msg: string },
    options?: CallOptions,
  ) => {
    const title = await nftTitle({ tokenId }, options);
    return await execute({
      send_nft: {
        contract,
        token_id: tokenId,
        msg,
      },
    }, { ...options, memo: `Send ${title} to ${contract}` });
  };

  // Approve txs
  const approve = async (
    { spender, tokenId, expiration }: { spender: string, tokenId: string, expiration?: Expiration },
    options?: CallOptions,
  ) => {
    const title = await nftTitle({ tokenId }, options);
    return await execute({
      approve: {
        spender,
        token_id: tokenId,
        expires: expiration,
      },
    }, { ...options, memo: `Approve ${spender} to spend ${title}` });
  };

  const approveBatch = async (
    { spender, tokenIds, expiration }: { spender: string, tokenIds: string[], expiration?: Expiration },
    options?: CallOptions,
  ) => {
    const { name } = await contractInfo(options);
    const title = `${name} ${tokenIds.map((id) => `#${id}`).join(',')}`;

    return await executeMultiple(
      tokenIds.map((tokenId) => ({
        msg: {
          approve: {
            spender,
            token_id: tokenId,
            expires: expiration,
          },
        },
      })),
      { ...options, memo: `Approve ${spender} to spend ${title}` },
    );
  };

  const approveAll = async ({ operator, expiration }: { operator: string, expiration?: Expiration }, options?: CallOptions) => {
    const { symbol } = await contractInfo(options);
    return await execute({
      approve_all: {
        operator,
        expires: expiration,
      },
    }, { ...options, memo: `Approve ${operator} to spend ${symbol} tokens` });
  };

  // Revoke txs
  const revoke = async ({ spender, tokenId }: { spender: string, tokenId: string }, options?: CallOptions) => {
    const title = await nftTitle({ tokenId }, options);
    return await execute({
      revoke: {
        spender,
        token_id: tokenId,
      },
    }, { ...options, memo: `Revoke approval from ${spender} to spend ${title}` });
  };

  const revokeBatch = async ({ spender, tokenIds }: { spender: string, tokenIds: string[] }, options?: CallOptions) => {
    const { name } = await contractInfo(options);
    const title = `${name} ${tokenIds.map((id) => `#${id}`).join(',')}`;

    return await executeMultiple(
      tokenIds.map((tokenId) => ({
        msg: {
          revoke: {
            spender,
            token_id: tokenId,
          },
        },
      })),
      { ...options, memo: `Revoke approval from ${spender} to spend ${title}` },
    );
  };

  const revokeAll = async ({ operator }: { operator: string }, options?: CallOptions) => {
    const { symbol } = await contractInfo(options);
    return await execute({
      revoke_all: {
        operator,
      },
    }, { ...options, memo: `Revoke approval from ${operator} to spend ${symbol} tokens` });
  };

  const burn = async ({ tokenId }: { tokenId: string }, options?: CallOptions) => {
    const title = await nftTitle({ tokenId }, options);
    return await execute({
      burn: {
        token_id: tokenId,
      },
    }, { ...options, memo: `Burn ${title} token` });
  };

  return {
    query,
    execute,
    executeMultiple,

    contractInfo,
    minter,
    ownerOf,
    approval,
    approvals,
    operator,
    allOperators,
    numTokens,
    tokens,
    allTokens,
    nftInfo,
    allNftInfo,
    nftTitle,

    transferNft,
    sendNft,
    approve,
    approveBatch,
    approveAll,
    revoke,
    revokeBatch,
    revokeAll,
    burn,
  };
};
