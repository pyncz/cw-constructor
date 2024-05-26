import type { IpfsResolver } from '~/types';

interface GatewayConfig {
  uri: string
  accessToken?: string
}

/**
 * Crate ipfs paths' resolver using provided Piñata gateway
 * @param gateway Source `ipfs://` link
 * @returns Resolved https link to the resource
 */
export const createPinataIpfsResolver = (gateway?: GatewayConfig): IpfsResolver => {
  const uri = gateway?.uri || 'https://ipfs.io';
  return (cid: string) => {
    return `${uri}/ipfs/${cid}${gateway?.accessToken ? `?pinataGatewayToken=${gateway.accessToken}` : ''}`;
  };
};
