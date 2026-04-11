export const kiesWillekeurig = <T>(array: T[]): T | undefined =>
  array.length === 0
    ? undefined
    : array[Math.floor(Math.random() * array.length)];
