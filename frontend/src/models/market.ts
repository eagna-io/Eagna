import moment, {Moment} from 'moment';

export interface Market {
  id: MarketId;
  title: string;
  organizer: string;
  shortDesc: string;
  description: string;
  openTime: Moment;
  closeTime: Moment;
  lmsrB: number;
  tokens: Token[];
  status: MarketStatus;
  settleTokenId?: number;
}

export type MarketId = number;

export interface Token {
  id: number;
  name: string;
  description: string;
}

export enum MarketStatus {
  Preparing = 'Preparing',
  Open = 'Open',
  Closed = 'Closed',
  Settled = 'Settled',
}

export type TokenId = number;

export interface MyMarket {
  orders: Order[];
}

export type Order = NormalOrder | InitialSupplyOrder | SettleOrder;

export interface NormalOrder {
  tokenId: TokenId;
  amountToken: number;
  amountCoin: number;
  time: Moment;
  type: 'Normal';
}

export interface InitialSupplyOrder {
  amountToken: number;
  amountCoin: number;
  time: Moment;
  type: 'InitialSupply';
}

export interface SettleOrder {
  tokenId: TokenId;
  amountToken: number;
  amountCoin: number;
  time: Moment;
  type: 'Settle';
}

export type PublicOrderHistory = NormalOrder[];

export type MyOrderHistory = Order[];

export type TokenDistribution = Map<TokenId, number>;

export type TokenPrices = Map<TokenId, number>;

export const KILO: number = 1000;

export function isNormalOrder(order: Order): order is NormalOrder {
  return order.type === 'Normal';
}

export function isInitialSupplyOrder(
  order: Order,
): order is InitialSupplyOrder {
  return order.type === 'InitialSupply';
}

export function isSettleOrder(order: Order): order is SettleOrder {
  return order.type === 'Settle';
}

export function getTokenDistribution(
  market: Market,
  orders: NormalOrder[],
): TokenDistribution {
  let distribution = new Map(market.tokens.map(t => [t.id, 0]));

  orders.forEach(order => {
    const currentAmount = distribution.get(order.tokenId) || 0;
    distribution.set(order.tokenId, currentAmount + order.amountToken);
  });

  return distribution;
}

export function getTokenPrices(
  lmsrB: number,
  distribution: TokenDistribution,
): TokenPrices {
  const distributionArray = Array.from(distribution.entries());
  const denom = distributionArray.reduce(
    (acc, [_id, n]) => acc + Math.exp(n / lmsrB),
    0,
  );

  return new Map(
    distributionArray.map(([id, n]) => [
      id,
      normalize(Math.exp(n / lmsrB) / denom),
    ]),
  );
}

export function distributionCost(
  lmsrB: number,
  distribution: TokenDistribution,
): number {
  const amountTokenArray = Array.from(distribution.values());
  const res =
    lmsrB *
    Math.log(amountTokenArray.reduce((acc, n) => acc + Math.exp(n / lmsrB), 0));
  return normalize(res);
}

function normalize(n: number): number {
  return Math.floor(n * KILO);
}

export function createDemoPreparingMarket() {
  return {
    id: 1,
    title: '明日の天気は？',
    organizer: 'eagna.com',
    shortDesc: '明日の天気を予測する',
    description: '明日の天気が雨かどうかを予測する',
    openTime: moment().subtract(1000 * 60 * 10), // 10分前
    closeTime: moment().add(1000 * 60 * 10), // 10分後
    lmsrB: 100,
    tokens: [
      {
        id: 1,
        name: '雨が降る',
        description: '雨が降る',
      },
      {
        id: 2,
        name: '雨が降らない',
        description: '雨が降らない',
      },
    ],
    status: MarketStatus.Preparing,
  };
}
