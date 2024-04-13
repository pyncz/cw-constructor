import consola from 'consola';

export const requireEnv = (key: string, env?: Record<string, string>) => {
  const value = env?.[key];
  if (!value) {
    consola.error(new Error(`"${key}" env var is not provided!`));
    process.exit();
  }
  return value;
};
