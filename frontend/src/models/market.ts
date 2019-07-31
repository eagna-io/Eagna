import {Organizer} from 'models/organizer';

/*
 * ====================
 *      Market
 * ====================
 */

abstract class AbstractMarket {
  readonly tokenPrices: Map<string, number>;

  constructor(
    readonly id: MarketId,
    readonly attrs: MarketAttributes,
    private tokenDistribution: Map<string, number>,
  ) {
    this.tokenPrices = computeLMSRPrices(tokenDistribution);
  }

  abstract getStatus(): MarketStatus;
}

export class MarketId {
  constructor(private id: string) {}
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
  constructor(id: MarketId, attrs: MarketAttributes) {
    const tokenDistribution = attrs.tokens.map(token => [token, 0]);
    super(id, attrs, tokenDistribution);
  }

  getStatus(): MarketStatus {
    return MarketStatus.Upcoming;
  }
}

export class OpenMarket extends AbstractMarket {
  constructor(
    id: MarketId,
    attrs: MarketAttributes,
    tokenDistribution: Map<string, number>,
  ) {
    super(id, attrs, tokenDistribution);
  }

  getStatus(): MarketStatus {
    return MarketStatus.Open;
  }

  // 指定のオーダーで、増える/減る coin の量を計算する
  // Buy オーダーの時、 amountToken は正の値をとる（トークンの量は増えるため）
  // Sell オーダーの時、 amountToken は負の値をとる（トークンの量は減るため）
  computeAmountCoinOfOrder(tokenName: string, amountToken: number): number {
    const currrentCost = computeLMSRCost(this.lmsrB, this.tokenDistribution);

    const nextTokenDistribution = this.tokenDistribution.map(
      ([token, amount]) =>
        token.name === tokenName ? amount + amountToken : amount,
    );
    const nextCost = computeLMSRCost(this.lmsrB, nextTokenDistribution);

    // cost が増えた時、 coin は減る. vice versa.
    return -(nextCost - currrentCost);
  }
}

export class ClosedMarket extends AbstractMarket {
  constructor(
    id: MarketId,
    attrs: MarketAttributes,
    tokenDistribution: Map<string, number>,
  ) {
    super(id, attrs, tokenDistribution);
  }

  getStatus(): MarketStatus {
    return MarketStatus.Closed;
  }
}

export class ResolvedMarket extends AbstractMarket {
  constructor(
    id: MarketId,
    attrs: MarketAttributes,
    tokenDistribution: Map<string, number>,
    readonly resolvedTokenName: string,
  ) {
    super(id, attrs, tokenDistribution);
  }

  getStatus(): MarketStatus {
    return MarketStatus.Resolved;
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
