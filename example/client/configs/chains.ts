export const chains = {
  'archway-1': {
    coin: 'ARCH',
    denom: 'aarch',
    decimals: 18,
    rpc: 'https://rpc.mainnet.archway.io',
  },
  'constantine-3': {
    coin: 'CONST',
    denom: 'aconst',
    decimals: 18,
    rpc: 'https://rpc.constantine.archway.io',
  },
  'titus-2': {
    coin: 'TITUS',
    denom: 'atitus',
    decimals: 18,
    rpc: 'https://rpc.titus.archway.io',
  },
};

export type ChainId = keyof typeof chains;

// Specify in type it's at least one chain
export const chainIds = Object.keys(chains) as [ChainId, ...ChainId[]];
