import { Map, values, keys, update } from "model/map";
import { DateTime } from "model/time";

export type Record = {
  outcome: string;
  price: number;
  time: DateTime;
  user: string;
};

/*
 * ============
 * Distribution
 * ============
 */
export type Distribution = {
  inner: Map<number>;

  // Computation Cache
  lmsrCost: number;
  lmsrPriceDenom: number;
};

export const create = (inner: Map<number>): Distribution => {
  return {
    inner,
    lmsrCost: computeLmsrCost(inner),
    lmsrPriceDenom: computeLmsrPriceDenom(inner)
  };
};

export const increment = (
  distribution: Distribution,
  outcome: string
): Distribution => {
  return create(update(distribution.inner, outcome, n => n + 1));
};

export const lmsrPrice = (
  distribution: Distribution,
  outcome: string
): number => {
  const numer = Math.exp((distribution.inner[outcome] || 0) / 30);
  return (numer / distribution.lmsrPriceDenom) * 1000;
};

export const outcomes = (distribution: Distribution): string[] => {
  return keys(distribution.inner);
};

const computeLmsrCost = (distribution: Map<number>): number => {
  const sum = values(distribution)
    .map(n => Math.exp(n / 30))
    .reduce((acc, n) => acc + n, 0);
  return Math.log(sum) * 30 * 1000;
};

const computeLmsrPriceDenom = (distribution: Map<number>): number => {
  return values(distribution)
    .map(n => Math.exp(n / 30))
    .reduce((acc, n) => acc + n, 0);
};
