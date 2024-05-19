export const truncateString = (value: string, max: number) => {
  const trailingString = '...';
  return value.length > max
    ? `${value.slice(0, max - trailingString.length)}${trailingString}`
    : value;
};
