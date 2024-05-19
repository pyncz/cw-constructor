import { z } from 'zod';
import { type ChainId, chainIds } from './configs';

export const envSchema = z
  .object({
    APP_URI: z.string().url(),
    CHAIN_ID: z.enum<ChainId, typeof chainIds>(chainIds),
    CONSTRUCTOR_ADDRESS: z.string(),
  })
  .transform((data) => ({
    appUri: data.APP_URI,
    chainId: data.CHAIN_ID,
    constructorAddress: data.CONSTRUCTOR_ADDRESS,
  }));

export type EnvSchema = z.infer<typeof envSchema>;

export const parseEnv = () => {
  return envSchema.parse(process.env);
};

export const env = parseEnv();
