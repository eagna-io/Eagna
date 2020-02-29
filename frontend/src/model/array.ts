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

export const empty = <T>(): Array<T> => {
  return {
    inner: [],
    size: 0
  };
};

export const newArray = <T>(inner: T[]): Array<T> => {
  return {
    inner,
    size: inner.length
  };
};

export const normalizeSubtract = <T>(
  minuend: Array<T>,
  subtrahend: Array<T>
): T[] => {
  const newArr = [];
  for (let i = subtrahend.size; i < minuend.size; i++) {
    newArr.push(minuend.inner[i]);
  }
  return newArr;
};
