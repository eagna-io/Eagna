import {Moment} from 'moment';

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

export type MyAssets = Map<'Coin' | TokenId, number>;

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

export function newTokenDistribution(
  tokens: Token[],
  maybeOrders?: NormalOrder[],
): TokenDistribution {
  let distribution = new Map(tokens.map(t => [t.id, 0]));

  const orders = maybeOrders === undefined ? [] : maybeOrders;
  orders.forEach(order => addOrderToTokenDistribution(distribution, order));

  return distribution;
}

export function addOrderToTokenDistribution(
  distribution: TokenDistribution,
  order: NormalOrder,
): void {
  const curAmount = distribution.get(order.tokenId) || 0;
  distribution.set(order.tokenId, curAmount + order.amountToken);
}

export function newTokenPrices(
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

export function getTokenPrice(
  prices: TokenPrices,
  tokenId: TokenId,
): number {
  return prices.get(tokenId) || 0;
}

export function cloneTokenDistribution(
  distribution: TokenDistribution,
): TokenDistribution {
  return new Map(distribution);
}

export function cloneTokenPrices(prices: TokenPrices): TokenPrices {
  return new Map(prices);
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

export function getMyAssets(
  tokens: Token[],
  myOrders: MyOrderHistory,
): MyAssets {
  let assets = new Map<'Coin' | TokenId, number>(tokens.map(t => [t.id, 0]));
  assets.set('Coin', currentAmountOfCoin(myOrders));
  tokens.forEach(token =>
    assets.set(token.id, currentAmountOfToken(myOrders, token.id)),
  );
  return assets;
}

function currentAmountOfCoin(myOrders: MyOrderHistory): number {
  return myOrders.reduce((acc, order) => acc + order.amountCoin, 0);
}

function currentAmountOfToken(
  myOrders: MyOrderHistory,
  tokenId: TokenId,
): number {
  return myOrders
    .filter(order => !isInitialSupplyOrder(order) && order.tokenId === tokenId)
    .reduce((acc, order) => acc + order.amountToken, 0);
}
