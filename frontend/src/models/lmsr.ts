const RewardCoin = 1000;

export class LMSR {
  // CostとPriceを計算するときに必要な計算をキャッシュしておく。
  // 具体的には sum(exp(q / B)) (qは各トークンの発行量)
  private cachePriceDenom: number;
  constructor(
    readonly distribution: { name: string; amount: number }[],
    readonly B: number
  ) {
    this.cachePriceDenom = sum(
      distribution.map(({ amount }) => Math.exp(amount / B))
    );
  }

  computeCost(): number {
    return this.normalize(this.B * Math.log(this.cachePriceDenom));
  }

  computePrice(tokenName: string): number {
    const token = this.distribution.find(
      ({ name }) => name === tokenName
    );
    if (token === undefined) {
      throw new Error(`${tokenName} is not found`);
    }
    return this.normalize(
      Math.exp(token.amount / this.B) / this.cachePriceDenom
    );
  }

  private normalize(n: number): number {
    return Math.floor(n * RewardCoin);
  }
}

function sum(array: number[]): number {
  return array.reduce((acc, n) => acc + n, 0);
}
