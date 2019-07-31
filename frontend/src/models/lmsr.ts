const RewardCoin = 1000;

export function computeLMSRCost(
  lmsrB: number,
  distribution: Map<string, number>,
): number {
  const raw =
    lmsrB * Math.log(sum(values(distribution).map(n => Math.exp(n / lmsrB))));
  return normalize(raw);
}

export function computeLMSRPrices(
  lmsrB: number,
  distribution: Map<string, number>,
): Map<string, number> {
  const denom = sum(values(distribution).map(n => Math.exp(n / lmsrB)));
  const prices = entries(distribution).map(
    ([name, n]) =>
      [name, normalize(Math.exp(n / lmsrB) / denom)] as [string, number],
  );

  return new Map(prices);
}

function values(map: Map<string, number>): number[] {
  return Array.from(map.values());
}

function entries(map: Map<string, number>): [string, number][] {
  return Array.from(map.entries());
}

function sum(array: number[]): number {
  return array.reduce((acc, n) => acc + n, 0);
}

function normalize(n: number): number {
  return Math.floor(n * RewardCoin);
}
