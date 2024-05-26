import { createSharedComposable } from '@vueuse/core';
import { createPinataIpfsResolver, resolveIpfs, resolveIpfsString } from '~/utils';

export const useIpfsGateway = createSharedComposable(() => {
  const { public: { ipfs } } = useRuntimeConfig();

  const resolver = ipfs.gatewayUri && ipfs.accessToken
    // If dedicated gateway is set up, use it
    ? createPinataIpfsResolver({
      uri: ipfs.gatewayUri,
      accessToken: ipfs.accessToken,
    })
    // Otherwise, just a public one
    : createPinataIpfsResolver({
      uri: ipfs.gatewayUri,
    });

  const resolveString = (link: string) => {
    return resolveIpfsString(link, resolver);
  };

  const resolve = <T>(obj: T) => {
    return resolveIpfs(obj, resolver);
  };

  return {
    resolver,
    resolveString,
    resolve,
  };
});
