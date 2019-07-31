class PublicOrderHistory {
  constructor(readonly orders: NormalOrder[]) {}
}

class MyAssets {
  readonly myTokens: Map<string, number>;
  readonly myCoins: number;

  constructor(tokens: Token[], orders: Order[]) {
    this.myCoins = orders.reduce((acc, order) => acc + order.amountCoin, 0);

    const amountTokens = new Map(tokens.map(token => [token.name, 0]));
    for order in order {
      if (order instanceof NormalOrder) {
        const currentVal = amountTokens.get(order.tokenName);
        amountTokens.set(order.tokenName, currentVal + order.amountToken);
      }
    }
    this.myTokens = amountTokens;
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
