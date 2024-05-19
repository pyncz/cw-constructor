import fs from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

export type ConfigResolvedValue = string | number | null | true | false;
export type ConfigValue = ConfigResolvedValue | { $ref: string } | { $var: string } | { $json: string };

type Resolver = (_input: Config) => Config | undefined;

export type Config<V extends ConfigValue = ConfigValue> = V | Config<V>[] | { [key: string]: Config<V> };

export const parse = (input: Config, resolver: Resolver): Config => {
  // Try to resolve with provided resolver
  const resolved = resolver(input);
  if (resolved !== undefined) {
    return parse(resolved, resolver);
  }

  // Otherwise, if a primitive, return as it is
  if (!input || typeof input === 'boolean' || typeof input === 'string' || typeof input === 'number') {
    return input;
  }

  // Array or object
  if (Array.isArray(input)) {
    return input.map((item) => parse(item, resolver));
  } else {
    return Object.entries(input).reduce(
      (res, [key, value]) => {
        (res as any)[key] = parse(value, resolver);
        return res;
      },
      {} as Config,
    );
  }
};

export const createVarsResolver = (vars: Record<string, ConfigResolvedValue>) => {
  return (input: Config) => {
    const { $var: variable } = (input ?? {}) as { $var?: string };
    if (variable) {
      return vars[variable] ?? input;
    }

    return undefined;
  };
};

export const createRefsResolver = (config: Config) => {
  return (input: Config) => {
    const { $ref: localReference } = (input ?? {}) as { $ref?: string };
    if (localReference) {
      const referencePath = localReference.split('.');

      // Resolve the path key-by-key,
      // until all the keys are used, target doesn't exist or the current target is null / string / number
      let target = config;
      while (target && typeof target !== 'boolean' && 'number' && typeof target !== 'string' && referencePath.length) {
        const cd = referencePath.shift();
        if (cd) {
          if (Array.isArray(target)) {
            const index = Number.parseInt(cd);
            target = target[index] ?? null;
          } else {
            target = target[cd as keyof typeof target] ?? null;
          }
        }
      }

      // Null if couldn't resolve to a value
      return target ?? null;
    }

    return undefined;
  };
};

export const jsonResolver = (input: Config) => {
  const __dirname = dirname(fileURLToPath(import.meta.url));
  const { $json: filePath } = (input ?? {}) as { $json?: string };
  if (filePath) {
    try {
      // Load json from path
      const rootPath = resolve(__dirname, '../../..');
      const resolvedFilepath = resolve(rootPath, filePath);
      const content = fs.readFileSync(resolvedFilepath, 'utf-8');
      return JSON.parse(content) as Config;
    } catch (e) {
      // Failed to resolve
      return null;
    }
  }

  return undefined;
};
