import { getIpfsCid } from './helpers';
import type { IpfsResolver } from '~/types';

export * from './helpers';
export * from './resolvers';

/**
 * Resolve link with provided ipfs resolver if it's an ipfs path;
 * returns without changes otherwise
 * @param link Source `ipfs://` link to resolve
 * @returns Resolved https link to the resource, or original link if it was not an ipfs path
 */
export const resolveIpfsString = (link: string, resolver: IpfsResolver) => {
  if (link && /^ipfs:\/\//.test(link)) {
    const cid = getIpfsCid(link);
    if (cid) {
      // An IPFS Content ID, resolve it with a gateway
      return resolver(cid);
    }
  }
  // A regular URI / no CID provided
  return link;
};

/**
 * Recursively resolve all the `ipfs://` entries in the object / array / string
 * @param obj Source object to resolve
 * @param resolver Resolver function to transform CIDs into gateway links
 * @returns Object with resolved ipfs paths if applicable; original value otherwise
 */
export const resolveIpfs = <T>(obj: T, resolver: IpfsResolver): T => {
  if (obj && typeof obj !== 'number') {
    // Just a string
    if (typeof obj === 'string') {
      return resolveIpfsString(obj, resolver) as T;
    }

    // Array or object
    if (Array.isArray(obj)) {
      return obj.map((item) => resolveIpfs(item, resolver)) as T;
    } else {
      return Object.entries(obj).reduce(
        (res, [key, value]) => {
          res[key] = resolveIpfs(value, resolver);
          return res;
        },
        {} as Record<string, any>,
      ) as T;
    }
  }
  return obj;
};
