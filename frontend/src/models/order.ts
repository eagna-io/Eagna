import {Moment} from 'moment';
import {Market, Token, TokenDistribution} from 'models/market';

export class PriceHistory {
  readonly rawHistory: {date: Date; prices: number[]}[];
  readonly tokens: string[];

  constructor(market: Market, orders: NormalOrder[]) {
    this.tokens = market.attrs.tokens.map(t => t.name);
    const distribution = new TokenDistribution(
      this.tokens,
      this.tokens.map(_n => 0),
    );

    const lmsrB = market.attrs.lmsrB;

    this.rawHistory = [
      {
        date: market.attrs.open.toDate(),
        prices: distribution.computeLMSRPrices(lmsrB).rawPrices,
      },
    ];

    orders.forEach(order => {
      distribution.addAssign(order.tokenName, order.amountToken);
      this.rawHistory.push({
        date: order.time.toDate(),
        prices: distribution.computeLMSRPrices(lmsrB).rawPrices,
      });
    });
  }

  getHistoryOf(tokenName: string): [Date, number][] {
    const idx = this.tokens.indexOf(tokenName);
    if (idx === -1) {
      throw new Error(`${tokenName} does not exist`);
    }
    return this.rawHistory.map(
      ({date, prices}) => [date, prices[idx]] as [Date, number],
    );
  }
}

export class MyAssets {
  readonly myTokens: number[];
  readonly myCoins: number;
  readonly tokens: string[];

  constructor(tokens: Token[], orders: Order[]) {
    this.tokens = tokens.map(t => t.name);
    this.myCoins = orders.reduce((acc, order) => acc + order.amountCoin, 0);

    this.myTokens = this.tokens.map(_n => 0);
    orders.forEach(order => {
      if (order instanceof NormalOrder) {
        const idx = this.tokens.indexOf(order.tokenName);
        if (idx === -1) {
          throw new Error(`Token ${order.tokenName} does not exist`);
        }
        this.myTokens[idx] += order.amountToken;
      }
    });
  }

  getTokenUncheck(tokenName: string): number {
    const idx = this.tokens.indexOf(tokenName);
    if (idx === -1) {
      throw new Error(`Token ${tokenName} does not exist`);
    }
    return this.myTokens[idx];
  }

  getCoin(): number {
    return this.myCoins;
  }
}

abstract class AbstractOrder {
  readonly uniqueString: string;

  constructor(
    readonly amountToken: number,
    readonly amountCoin: number,
    readonly time: Moment,
  ) {
    const randomNum = Math.floor(Math.random() * 100000);
    this.uniqueString = `${time.unix()}-${randomNum}`;
  }

  abstract getType(): OrderType;
}

export enum OrderType {
  CoinSupply,
  Normal,
  Reward,
}

export type Order = CoinSupplyOrder | NormalOrder | RewardOrder;

export class CoinSupplyOrder extends AbstractOrder {
  constructor(amountCoin: number, time: Moment) {
    super(0, amountCoin, time);
  }

  getType(): OrderType {
    return OrderType.CoinSupply;
  }
}

export class NormalOrder extends AbstractOrder {
  constructor(
    readonly tokenName: string,
    amountToken: number,
    amountCoin: number,
    time: Moment,
  ) {
    super(amountToken, amountCoin, time);
  }

  getType(): OrderType {
    return OrderType.Normal;
  }
}

export class RewardOrder extends AbstractOrder {
  constructor(readonly tokenName: string, amountCoin: number, time: Moment) {
    super(0, amountCoin, time);
  }

  getType(): OrderType {
    return OrderType.Reward;
  }
}
