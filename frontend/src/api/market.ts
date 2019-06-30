import moment, {Moment} from 'moment';
import * as D from '@mojotech/json-type-validation';

import {request, Method, isFailure} from 'api/core';
import {
  Market,
  MarketStatus,
  MarketId,
  TokenId,
  Order,
  NormalOrder,
  InitialSupplyOrder,
  SettleOrder,
} from 'models/market';

/*
 * ========================
 * Get Market
 * ========================
 */

export function getMarket(id: MarketId): Promise<Market> {
  return request({
    method: Method.GET,
    path: `/markets/${id}/`,
    decoder: marketDecoder,
  }).then(res => {
    if (isFailure(res)) {
      throw `Unexpected failure : ${res.error.message}`;
    } else {
      return res;
    }
  });
}

export function getMarkets(statusList?: MarketStatus[]): Promise<Market[]> {
  return request({
    method: Method.GET,
    path: '/markets/',
    params:
      statusList === undefined
        ? undefined
        : {
            status: statusList.map(s => s.toLowerCase()),
          },
    decoder: D.array(marketDecoder),
  }).then(res => {
    if (isFailure(res)) {
      throw `Unexpected failure : ${res.error.message}`;
    } else {
      return res;
    }
  });
}

export const marketDecoder: D.Decoder<Market> = D.object({
  id: D.number(),
  title: D.string(),
  organizer: D.string(),
  shortDesc: D.string(),
  description: D.string(),
  openTime: D.string().map(s => moment(s)),
  closeTime: D.string().map(s => moment(s)),
  lmsrB: D.number(),
  tokens: D.array(
    D.object({
      id: D.number(),
      name: D.string(),
      description: D.string(),
    }),
  ),
  status: D.string().map(str2status),
  settleTokenId: D.optional(D.number()),
});

function str2status(s: string): MarketStatus {
  switch (s) {
    case 'Preparing':
      return MarketStatus.Upcoming;
    case 'Open':
      return MarketStatus.Open;
    case 'Closed':
      return MarketStatus.Closed;
    case 'Settled':
      return MarketStatus.Resolved;
    default:
      throw `Invalid market status : ${s}`;
  }
}

/*
 * ==================
 * Post Market
 * =================
 */

interface PostMarketArgs {
  market: {
    title: string;
    organizer: string;
    shortDesc: string;
    description: string;
    lmsrB: number;
    openTime: Moment;
    closeTime: Moment;
    tokens: {
      name: string;
      description: string;
    }[];
  };
  accessToken: string;
}

export function postMarket({
  market,
  accessToken,
}: PostMarketArgs): Promise<MarketId> {
  return request({
    method: Method.POST,
    path: '/markets/',
    accessToken: accessToken,
    body: market,
    decoder: D.number(),
  }).then(res => {
    if (isFailure(res)) {
      throw `Unexpected error : ${res.error.message}`;
    } else {
      return res;
    }
  });
}

/*
 * =======================
 * Resolve Market
 * =======================
 */

interface ResolveMarketArgs {
  marketId: number;
  resolveTokenId: number;
  accessToken: string;
}

export function resolveMarket({
  marketId,
  resolveTokenId,
  accessToken,
}: ResolveMarketArgs): Promise<number> {
  return request({
    method: Method.PUT,
    path: `/markets/${marketId}/`,
    accessToken: accessToken,
    body: {
      status: 'Settled',
      settleTokenId: resolveTokenId,
    },
    decoder: D.number(),
  }).then(res => {
    if (isFailure(res)) {
      throw `Unexpected error : ${res.error.message}`;
    } else {
      return res;
    }
  });
}

/*
 * ========================
 * Get Orders
 * ========================
 */

interface GetOrdersResp {
  orders: NormalOrder[];
  myOrders?: Order[];
}

export function getOrders(
  marketId: MarketId,
  accessToken?: string,
): Promise<GetOrdersResp> {
  return request({
    method: Method.GET,
    path: `/markets/${marketId}/orders/`,
    params: accessToken
      ? {
          contains: 'mine',
        }
      : undefined,
    accessToken: accessToken,
    decoder: ordersDecoder,
  }).then(res => {
    if (isFailure(res)) {
      // AccessTokenの有効期限切れ
      if (res.error.code === 2) {
        return getOrders(marketId);
      } else {
        throw `Unexpected failure : ${res.error.message}`;
      }
    } else {
      return res;
    }
  });
}

const normalOrderDecoder: D.Decoder<NormalOrder> = D.object({
  tokenId: D.number(),
  amountToken: D.number(),
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
  type: D.constant('Normal'),
});

const initialSupplyOrderDecoder: D.Decoder<InitialSupplyOrder> = D.object({
  amountToken: D.number(),
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
  type: D.constant('InitialSupply'),
});

const settleOrderDecoder: D.Decoder<SettleOrder> = D.object({
  tokenId: D.number(),
  amountToken: D.number(),
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
  type: D.constant('Settle'),
});

const ordersDecoder: D.Decoder<GetOrdersResp> = D.object({
  orders: D.array(
    D.object({
      tokenId: D.number(),
      amountToken: D.number(),
      amountCoin: D.number(),
      time: D.string().map(s => moment(s)),
      type: D.succeed<'Normal'>('Normal'),
    }),
  ),
  mine: D.optional(
    D.object({
      orders: D.array(
        D.union(
          normalOrderDecoder,
          initialSupplyOrderDecoder,
          settleOrderDecoder,
        ),
      ),
    }),
  ),
}).map(obj => ({
  orders: obj.orders,
  myOrders: obj.mine ? obj.mine.orders : undefined,
}));

/*
 * ===================
 * Create Initial Supply Order
 * ===================
 */

interface CreateInitialSupplyOrderArgs {
  marketId: MarketId;
  accessToken: string;
}

interface CreatedInitialSupplyOrder {
  amountCoin: number;
  time: Moment;
  type: 'initialSupply';
}

export function createInitialSupplyOrder({
  marketId,
  accessToken,
}: CreateInitialSupplyOrderArgs): Promise<CreatedInitialSupplyOrder> {
  return request({
    method: Method.POST,
    path: `/markets/${marketId}/orders/`,
    accessToken: accessToken,
    body: {
      type: 'initialSupply',
    },
    decoder: createdInitialSupplyOrderDecoder,
  }).then(res => {
    if (isFailure(res)) {
      throw `Unexpected error : ${res.error.message}`;
    } else {
      return res;
    }
  });
}

const createdInitialSupplyOrderDecoder: D.Decoder<
  CreatedInitialSupplyOrder
> = D.object({
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
  type: D.constant('initialSupply'),
});

/*
 * ===================
 * Create Normal Order
 * ===================
 */

interface CreateNormalOrderArgs {
  marketId: MarketId;
  order: {
    tokenId: TokenId;
    amountToken: number;
    amountCoin: number;
  };
  accessToken: string;
}

interface CreatedNormalOrder {
  tokenId: TokenId;
  amountToken: number;
  amountCoin: number;
  time: Moment;
}

export function createNormalOrder({
  marketId,
  order,
  accessToken,
}: CreateNormalOrderArgs): Promise<CreatedNormalOrder | 'PriceSlip'> {
  return request({
    method: Method.POST,
    path: `/markets/${marketId}/orders/`,
    accessToken: accessToken,
    body: {
      type: 'normal',
      ...order,
    },
    decoder: createdNormalOrderDecoder,
  }).then(res => {
    if (isFailure(res)) {
      if (res.error.code === 1) {
        // code 1 => Invalid payload error
        // 他の要素は適切（なはず）なので、ここでのエラーは価格スリップエラー
        return 'PriceSlip';
      } else {
        throw `Unexpected error : ${res.error.message}`;
      }
    } else {
      return res;
    }
  });
}

const createdNormalOrderDecoder: D.Decoder<CreatedNormalOrder> = D.object({
  tokenId: D.number(),
  amountToken: D.number(),
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
  type: D.constant('normal'),
});
