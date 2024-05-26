// TODO: Remove `symbol` once `/cosmos/bank/v1beta1/denoms_metadata/{denom}` response is fixed
export const chains = {
  'archway-1': {
    denom: 'aarch',
    rpc: 'https://rpc.mainnet.archway.io',
    symbol: 'ARCH',
  },
  'constantine-3': {
    denom: 'aconst',
    rpc: 'https://rpc.constantine.archway.io',
    symbol: 'CONST',
  },
  'titus-2': {
    denom: 'atitus',
    rpc: 'https://rpc.titus.archway.io',
    symbol: 'TITUS',
  },
};

export type ChainId = keyof typeof chains;

// Specify in type it's at least one chain
export const chainIds = Object.keys(chains) as [ChainId, ...ChainId[]];
