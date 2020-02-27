import { Map, List, Seq } from "immutable";
import { Moment } from "moment";

export type Record = {
  outcome: string;
  price: number;
  time: Moment;
  prevDistribution: Distribution;
  nextDistribution: Distribution;
};

/*
 * ============
 * Distribution
 * ============
 */
export class Distribution {
  readonly lmsrCost: number;
  readonly lmsrPriceDenom: number;

  constructor(private readonly inner: Map<string, number>) {
    this.lmsrCost = computeLmsrCost(inner);
    this.lmsrPriceDenom = computeLmsrPriceDenom(inner);
  }

  static initialize(outcomes: List<string>): Distribution {
    const distribution = Map(outcomes.map(o => [o, 0] as [string, number]));
    return new Distribution(distribution);
  }

  increment(outcome: string): Distribution {
    return new Distribution(this.inner.update(outcome, prev => prev + 1));
  }

  lmsrPrice(outcome: string): number {
    const numer = Math.exp((this.inner.get(outcome) || 0) / 30);
    return (numer / this.lmsrPriceDenom) * 1000;
  }

  outcomes(): Seq.Indexed<string> {
    return this.inner.keySeq();
  }
}

const computeLmsrCost = (distribution: Map<string, number>): number => {
  const sum = distribution
    .map(n => Math.exp(n / 30))
    .reduce((acc, n) => acc + n, 0);
  return Math.log(sum) * 30 * 1000;
};

const computeLmsrPriceDenom = (distribution: Map<string, number>): number => {
  return distribution.map(n => Math.exp(n / 30)).reduce((acc, n) => acc + n, 0);
};
