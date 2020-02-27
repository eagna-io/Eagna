// Extensible immutable Array
export type Array<T> = {
  inner: T[];
  size: number;
};

// Returns a new created Array
export const push = <T>(array: Array<T>, item: T): Array<T> => {
  array.inner.push(item);
  return {
    inner: array.inner,
    size: array.size + 1
  };
};
