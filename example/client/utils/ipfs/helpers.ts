export const getIpfsCid = (link: string) => {
  return link.replace(/^ipfs:\/\//, '') || undefined;
};
