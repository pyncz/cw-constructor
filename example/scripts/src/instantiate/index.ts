import fs from 'fs';
import { dirname, resolve } from 'path';
import { fileURLToPath } from 'url';

import { config as loadEnv } from 'dotenv';
import { consola } from 'consola';
import { getSigningClient, requireEnv } from '../utils';
import { createRefsResolver, createVarsResolver, getChainConfig, jsonResolver, parse } from './utils';

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootPath = resolve(__dirname, '../..');
const configPath = `${rootPath}/instantiate.config.json`;
const config = JSON.parse(fs.readFileSync(configPath, 'utf-8'));

// Load env
const { parsed: env } = loadEnv();
const SENDER_MNEMONIC = requireEnv('SENDER_MNEMONIC', env);
const CHAIN_ID = requireEnv('CHAIN_ID', env);

const { denom, node: rpc } = getChainConfig(CHAIN_ID);

// Run script
try {
  consola.start('Instantiating contracts...');

  const { address: sender, client } = await getSigningClient(rpc, SENDER_MNEMONIC);

  for (const [contractSlug, rawContractConfig] of Object.entries(config)) {
    consola.info({ message: contractSlug, level: 1 });

    // Include external objects, e.g. tokens' metadata options
    let contractConfig = parse(rawContractConfig as any, jsonResolver) as any;
    // Resolve variables
    contractConfig = parse(contractConfig, createVarsResolver({
      sender,
      denom,
    }));

    const { address, code_id: codeId, admin, msg: rawMsg, after: rawAfter } = contractConfig;

    if (address) {
      consola.log(`Already instantiated as ${address}`);
    } else {
      // Instantiate the contract with message from config
      const msg = parse(rawMsg, createRefsResolver(config));
      const { contractAddress, transactionHash } = await client.instantiate(sender, parseInt(codeId), msg, contractSlug, 'auto', {
        memo: `Instantiate ${contractSlug}`,
        admin,
      });
      // Update contract address / tx hash with the new ones
      config[contractSlug].address = contractAddress;
      config[contractSlug].tx = transactionHash;

      consola.success(`Instantiated as ${contractAddress} at ${transactionHash}`);
    }
    // Execute `after` callback if provided and not executed already at previous tries
    if (rawAfter && !rawAfter.tx) {
      const after = parse(rawAfter, createRefsResolver(config)) as any;
      const { transactionHash } = await client.execute(sender, after.contract, after.msg, 'auto');
      // Update tx hash
      config[contractSlug].after.tx = transactionHash;

      consola.info(`Executed "after" callback at ${transactionHash}`);
    }
  }
} catch (e) {
  consola.error(e);
} finally {
  // Update the config json with results
  fs.writeFileSync(configPath, `${JSON.stringify(config, null, 2)}\n`);
  consola.info('Saved updated config.');
}
