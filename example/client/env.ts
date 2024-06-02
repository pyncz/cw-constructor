import { z } from 'zod';
import { type ChainId, chainIds } from './utils';

export const envSchema = z
  .object({
    APP_URI: z.string().url(),
    CHAIN_ID: z.enum<ChainId, typeof chainIds>(chainIds),
    CONSTRUCTOR_ADDRESS: z.string(),
    IPFS_GATEWAY_URI: z.string().url().optional(),
    IPFS_GATEWAY_ACCESS_TOKEN: z.string().optional(),
  })
  .transform((data) => ({
    appUri: data.APP_URI,
    chainId: data.CHAIN_ID,
    constructorAddress: data.CONSTRUCTOR_ADDRESS,
    ipfs: {
      gatewayUri: data.IPFS_GATEWAY_URI,
      accessToken: data.IPFS_GATEWAY_ACCESS_TOKEN,
    },
  }));

export type EnvSchema = z.infer<typeof envSchema>;

export const parseEnv = () => {
  return envSchema.parse(process.env);
};

export const env = parseEnv();
