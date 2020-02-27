export type Map<V> = {
  [key: string]: V;
};

export const values = <V>(map: Map<V>): V[] => {
  return Array.from(Object.values(map));
};

export const keys = <V>(map: Map<V>): string[] => {
  return Array.from(Object.keys(map));
};

export const update = <V>(map: Map<V>, key: string, f: (v: V) => V): Map<V> => {
  return {
    ...map,
    [key]: f(map[key])
  };
};
