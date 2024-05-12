import { ArchwayClient, SigningArchwayClient } from '@archwayhq/arch3.js';
import { DirectSecp256k1HdWallet } from '@cosmjs/proto-signing';

export const getClient = async (endpoint: string) => {
  return await ArchwayClient.connect(endpoint);
};

export const getSigningClient = async (endpoint: string, mnemonic: string) => {
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic (mnemonic, { prefix: 'archway' });
  const [account] = await wallet.getAccounts();
  if (account) {
    const client = await SigningArchwayClient.connectWithSigner(endpoint, wallet);
    return { address: account.address, client };
  }
  throw new Error('No connected account!');
};
