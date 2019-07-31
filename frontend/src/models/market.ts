import {computeLMSRCost, computeLMSRPrices} from 'models/lmsr';
import {Moment} from 'moment';

/*
 * ====================
 *      Market
 * ====================
 */

abstract class AbstractMarket {
  readonly tokenPrices: Map<string, number>;

  constructor(
    readonly id: string,
    readonly attrs: MarketAttributes,
    protected tokenDistribution: Map<string, number>,
  ) {
    this.tokenPrices = computeLMSRPrices(attrs.lmsrB, tokenDistribution);
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
    const tokenDistribution = new Map(
      attrs.tokens.map(token => [token.name, 0] as [string, number]),
    );
    super(id, attrs, tokenDistribution);
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
  // Buy オーダーの時、 amountToken は正の値をとる（トークンの量は増えるため）
  // Sell オーダーの時、 amountToken は負の値をとる（トークンの量は減るため）
  computeAmountCoinOfOrder(tokenName: string, amountToken: number): number {
    const currrentCost = computeLMSRCost(
      this.attrs.lmsrB,
      this.tokenDistribution,
    );

    const currentAmountToken = this.tokenDistribution.get(tokenName);
    if (currentAmountToken === undefined) {
      throw new Error(`Token ${tokenName} does not exist`);
    }

    const nextTokenDistribution = new Map(this.tokenDistribution);
    nextTokenDistribution.set(tokenName, currentAmountToken + amountToken);

    const nextCost = computeLMSRCost(this.attrs.lmsrB, nextTokenDistribution);

    // cost が増えた時、 coin は減る. vice versa.
    return -(nextCost - currrentCost);
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
    tokenDistribution: Map<string, number>,
    readonly resolvedTokenName: string,
  ) {
    super(id, attrs, tokenDistribution);
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
