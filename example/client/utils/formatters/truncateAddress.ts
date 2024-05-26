export const truncateAddress = (address: string) => {
  return address.replace(/^(.{11})(.{4,})(.{5})$/, '$1...$3');
};
