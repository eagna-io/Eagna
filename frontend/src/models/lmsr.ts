const RewardCoin = 1000;

export function computeLMSRCost(lmsrB: number, distribution: number[]): number {
  const raw = lmsrB * Math.log(sum(distribution.map(q => Math.exp(q / lmsrB))));
  return normalize(raw);
}

export function computeLMSRPrices(
  lmsrB: number,
  distribution: number[],
): number[] {
  const denom = sum(distribution.map(q => Math.exp(q / lmsrB)));
  return distribution.map(q => normalize(Math.exp(q / lmsrB) / denom));
}

function sum(array: number[]): number {
  return array.reduce((acc, n) => acc + n, 0);
}

function normalize(n: number): number {
  return Math.floor(n * RewardCoin);
}
