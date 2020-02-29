export type Map<V> = {
  [key: string]: V;
};

export const values = <V>(map: Map<V>): V[] => {
  return Object.values(map);
};

export const keys = <V>(map: Map<V>): string[] => {
  return Object.keys(map);
};

export const entries = <V>(map: Map<V>): [string, V][] => {
  return Object.entries(map);
};

export const update = <V>(map: Map<V>, key: string, f: (v: V) => V): Map<V> => {
  return {
    ...map,
    [key]: f(map[key])
  };
};

export const forEach = <V>(map: Map<V>, f: (k: string, v: V) => void): void => {
  Object.entries(map).forEach(([k, v]) => f(k, v));
};
