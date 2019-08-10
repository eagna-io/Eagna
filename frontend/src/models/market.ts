import {computeLMSRCost, computeLMSRPrices} from 'models/lmsr';
import {Moment} from 'moment';

/*
 * ====================
 *      Market
 * ====================
 */

abstract class AbstractMarket {
  readonly tokenPrices: TokenPrices;
  readonly tokenDistribution: TokenDistribution;

  constructor(
    readonly id: string,
    readonly attrs: MarketAttributes,
    rawTokenDistribution: number[],
  ) {
    this.tokenDistribution = new TokenDistribution(
      attrs.tokens.map(t => t.name),
      rawTokenDistribution,
    );
    this.tokenPrices = this.tokenDistribution.computeLMSRPrices(attrs.lmsrB);
  }

  abstract getStatus(): MarketStatus;
}

export class MarketAttributes {
  constructor(
    readonly title: string,
    readonly organizerId: string,
    readonly description: string,
    readonly open: Moment,
    readonly close: Moment,
    readonly lmsrB: number,
    readonly tokens: Token[],
    readonly prizes: Prize[],
  ) {}
}

export type MarketStatus = 'Upcoming' | 'Open' | 'Closed' | 'Resolved';

export type Market =
  | UpcomingMarket
  | OpenMarket
  | ClosedMarket
  | ResolvedMarket;

export class UpcomingMarket extends AbstractMarket {
  constructor(id: string, attrs: MarketAttributes) {
    const rawTokenDistribution = attrs.tokens.map(_t => 0);
    super(id, attrs, rawTokenDistribution);
  }

  getStatus(): MarketStatus {
    return 'Upcoming';
  }
}

export class OpenMarket extends AbstractMarket {
  getStatus(): MarketStatus {
    return 'Open';
  }

  // 指定のオーダーで、増える/減る coin の量を計算する
  // Buy オーダーの時、
  //  - amountToken は正の値をとる（トークンの量は増えるため）
  //  - coinCost は負の値をとる
  // Sell オーダーの時、
  //  - amountToken は負の値をとる（トークンの量は減るため）
  //  - coinCost は正の値をとる
  computeAmountCoinOfOrder(tokenName: string, amountToken: number): number {
    const curCost = this.tokenDistribution.computeLMSRCost(this.attrs.lmsrB);

    const nextCost = this.tokenDistribution
      .add(tokenName, amountToken)
      .computeLMSRCost(this.attrs.lmsrB);

    // cost が増えた時、 coin は減る. vice versa.
    return -(nextCost - curCost);
  }
}

export class ClosedMarket extends AbstractMarket {
  getStatus(): MarketStatus {
    return 'Closed';
  }
}

export class ResolvedMarket extends AbstractMarket {
  constructor(
    id: string,
    attrs: MarketAttributes,
    rawTokenDistribution: number[],
    readonly resolvedTokenName: string,
  ) {
    super(id, attrs, rawTokenDistribution);
  }

  getStatus(): MarketStatus {
    return 'Resolved';
  }
}

export class Token {
  constructor(
    readonly name: string,
    readonly desc: string,
    readonly sumbnailUrl: string,
  ) {}
}

export class Prize {
  constructor(
    readonly id: PrizeId,
    readonly name: string,
    readonly target: string,
    readonly sumbnailUrl: string,
  ) {}
}

export class PrizeId {
  constructor(private id: number) {}
}

/*
 * ===================
 * Token Distribution
 * ===================
 */
export class TokenDistribution {
  constructor(readonly tokens: string[], readonly rawDistribution: number[]) {}

  getUncheck(tokenName: string): number {
    const idx = this.tokens.indexOf(tokenName);
    if (idx === -1) {
      throw new Error(`${tokenName} does not exist`);
    }
    return this.rawDistribution[idx];
  }

  computeLMSRCost(lmsrB: number): number {
    return computeLMSRCost(lmsrB, this.rawDistribution);
  }

  computeLMSRPrices(lmsrB: number): TokenPrices {
    return new TokenPrices(lmsrB, this);
  }

  add(tokenName: string, quantity: number): TokenDistribution {
    const cloned = this.clone();
    cloned.addAssign(tokenName, quantity);
    return cloned;
  }

  addAssign(tokenName: string, quantity: number) {
    const idx = this.tokens.indexOf(tokenName);
    if (idx === -1) {
      throw new Error(`${tokenName} does not exist`);
    }
    this.rawDistribution[idx] += quantity;
  }

  clone(): TokenDistribution {
    return new TokenDistribution(this.tokens, Array.from(this.rawDistribution));
  }
}

export class TokenPrices {
  readonly rawPrices: number[];
  readonly tokens: string[];

  constructor(lmsrB: number, distribution: TokenDistribution) {
    this.tokens = distribution.tokens;
    this.rawPrices = computeLMSRPrices(lmsrB, distribution.rawDistribution);
  }

  getUncheck(tokenName: string): number {
    const idx = this.tokens.indexOf(tokenName);
    if (idx === -1) {
      throw new Error(`${tokenName} does not exist`);
    }
    return this.rawPrices[idx];
  }
}
