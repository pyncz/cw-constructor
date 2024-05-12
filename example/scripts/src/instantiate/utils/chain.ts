export const getChainConfig = (chainId: string) => {
  switch (chainId) {
    case 'archway-1':
      return { denom: 'aarch', node: 'https://rpc.mainnet.archway.io:443', chainId };
    case 'titus-2':
      return { denom: 'atitus', node: 'https://rpc.titus.archway.io:443', chainId };
    case 'constantine-3':
      return { denom: 'aconst', node: 'https://rpc.constantine.archway.io:443', chainId };
    default:
      throw new Error('Invalid CHAIN_ID!');
  }
};
