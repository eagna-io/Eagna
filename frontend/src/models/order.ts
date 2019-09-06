import moment, { Moment } from "moment";
import {
  EagnaOrderApi,
  Order as InfraOrder,
  OrderType as InfraOrderType
} from "infra/eagna/order";
import { Market, TokenDistribution } from "models/market";
import { LMSR } from "models/lmsr";
import { User } from "models/user";

export enum OrderType {
  CoinSupply = "CoinSupply",
  Normal = "Normal",
  Reward = "Reward"
}

export class Order {
  readonly uniqueString: string;

  constructor(
    readonly amountToken: number,
    readonly amountCoin: number,
    readonly time: Moment,
    readonly type: OrderType,
    readonly tokenName?: string
  ) {
    const randomNum = Math.floor(Math.random() * 100000);
    this.uniqueString = `${time.unix()}-${randomNum}`;
  }

  static coinSupply(): Order {
    return new Order(0, 10000, moment(), OrderType.CoinSupply);
  }

  static normal(order: {
    tokenName: string;
    amountToken: number;
    amountCoin: number;
  }): Order {
    return new Order(
      order.amountToken,
      order.amountCoin,
      moment(),
      OrderType.Normal,
      order.tokenName
    );
  }

  static fromInfra(order: InfraOrder): Order {
    return new Order(
      order.amountToken,
      order.amountCoin,
      order.time,
      fromInfraOrderType(order.type),
      order.tokenName
    );
  }
}

function fromInfraOrderType(type: InfraOrderType): OrderType {
  switch (type) {
    case InfraOrderType.CoinSupply:
      return OrderType.CoinSupply;
    case InfraOrderType.Normal:
      return OrderType.Normal;
    case InfraOrderType.Reward:
      return OrderType.Reward;
  }
}

function toInfraOrderType(type: OrderType): InfraOrderType {
  switch (type) {
    case OrderType.CoinSupply:
      return InfraOrderType.CoinSupply;
    case OrderType.Normal:
      return InfraOrderType.Normal;
    case OrderType.Reward:
      return InfraOrderType.Reward;
  }
}

export class DistributionHistory {
  constructor(
    readonly rawHistory: {
      date: Moment;
      distribution: TokenDistribution;
    }[]
  ) {}

  static fromPublicOrders(
    market: Market,
    publicOrders: Order[]
  ): DistributionHistory {
    // 最初のトークン配布量で初期化
    const initialDistribution = new TokenDistribution(
      market.attrs.tokens.map(token => ({ name: token.name, amount: 0 }))
    );
    const history = [
      {
        date: market.attrs.open,
        distribution: initialDistribution
      }
    ];

    publicOrders.sort((a, b) => a.time.valueOf() - b.time.valueOf());

    // 各オーダーが出された時の価格を算出
    const distribution = initialDistribution.clone();
    publicOrders.forEach(order => {
      // publicOrdersは必ずtokenNameを持っているはずだが
      // 一応キャストではなくif文を使う
      if (order.tokenName) {
        distribution.addAssign(order.tokenName, order.amountToken);
        history.push({
          date: order.time,
          distribution: distribution.clone()
        });
      }
    });

    // 最終価格を追加
    // 日時は、現在時間とclose時間のうち古い方
    const lastDistribution = history[history.length - 1].distribution;
    history.push({
      date: moment.min(moment(), market.attrs.close),
      distribution: lastDistribution
    });

    console.log(history);

    return new DistributionHistory(history);
  }
}

export class PriceHistory {
  constructor(readonly rawHistory: { date: Moment; lmsr: LMSR }[]) {}

  static fromDistributionHistory(
    distributionHistory: DistributionHistory,
    lmsrB: number
  ): PriceHistory {
    const history = distributionHistory.rawHistory.map(
      ({ date, distribution }) => ({
        date,
        lmsr: new LMSR(distribution.rawDistribution, lmsrB)
      })
    );
    return new PriceHistory(history);
  }

  get(tokenName: string): { date: Moment; price: number }[] {
    return this.rawHistory.map(({ date, lmsr }) => ({
      date,
      price: lmsr.computePrice(tokenName)
    }));
  }
}

export class MyAssets {
  constructor(
    readonly myTokens: {
      name: string;
      amount: number;
    }[],
    readonly myCoins: number
  ) {}

  static fromMyOrders(myOrders: Order[]): MyAssets {
    const myCoins = myOrders.reduce((acc, order) => acc + order.amountCoin, 0);

    const myTokens: { name: string; amount: number }[] = [];
    myOrders.forEach(order => {
      if (order.type === OrderType.Normal) {
        const tokenName = order.tokenName as string;
        const myToken = myTokens.find(({ name }) => name === tokenName);
        if (myToken) {
          myToken.amount += order.amountToken;
        } else {
          myTokens.push({
            name: tokenName,
            amount: order.amountToken
          });
        }
      }
    });

    return new MyAssets(myTokens, myCoins);
  }

  getToken(tokenName: string): number {
    const token = this.myTokens.find(({ name }) => name === tokenName);
    if (token) {
      return token.amount;
    } else {
      return 0;
    }
  }

  getCoin(): number {
    return this.myCoins;
  }
}

export class OrderRepository {
  static async queryList(market: Market): Promise<Order[]> {
    const infraOrders = await EagnaOrderApi.queryList(market.id);
    return infraOrders.map(Order.fromInfra);
  }

  static async queryListOfMine(market: Market, user: User): Promise<Order[]> {
    const accessToken = await user.getAccessToken();
    const infraOrders = await EagnaOrderApi.queryListOfMine(
      market.id,
      accessToken
    );
    return infraOrders.map(Order.fromInfra);
  }

  // 引数のOrderと返り値のOrderは異なる可能性がある。
  // 価格スリップが発生する可能性があるため。
  static async create(
    market: Market,
    user: User,
    order: Order
  ): Promise<Order> {
    const accessToken = await user.getAccessToken();
    const reqOrder = {
      tokenName: order.tokenName,
      amountToken: order.amountToken,
      amountCoin: order.amountCoin,
      time: order.time,
      type: toInfraOrderType(order.type)
    };
    const newOrder = await EagnaOrderApi.create(
      market.id,
      accessToken,
      reqOrder
    );
    return Order.fromInfra(newOrder);
  }
}
