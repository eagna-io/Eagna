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
      return MarketStatus.Preparing;
    case 'Open':
      return MarketStatus.Open;
    case 'Closed':
      return MarketStatus.Closed;
    case 'Settled':
      return MarketStatus.Settled;
    default:
      throw `Invalid market status : ${s}`;
  }
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
      throw `Unexpected failure : ${res.error.message}`;
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
 * ===================
 * Post Order
 * ===================
 */

interface PostOrderArgs {
  marketId: MarketId;
  order:
    | {
        tokenId: TokenId;
        amountToken: number;
        amountCoin: number;
        type: 'normal';
      }
    | {
        type: 'initialSupply';
      };
  accessToken: string;
}

interface CreatedNormalOrder {
  tokenId: TokenId;
  amountToken: number;
  amountCoin: number;
  time: Moment;
  type: 'normal';
}

interface CreatedInitialSupplyOrder {
  amountCoin: number;
  time: Moment;
  type: 'initialSupply';
}

export function postOrder({
  marketId,
  order,
  accessToken,
}: PostOrderArgs): Promise<CreatedNormalOrder | CreatedInitialSupplyOrder> {
  return request({
    method: Method.POST,
    path: `/markets/${marketId}/orders/`,
    accessToken: accessToken,
    body: order,
    decoder: createdOrderDecoder,
  }).then(res => {
    if (isFailure(res)) {
      // TODO;
      throw `Unexpected error : ${res.error.message}`;
    } else {
      return res;
    }
  });
}

const createdOrderDecoder: D.Decoder<
  CreatedNormalOrder | CreatedInitialSupplyOrder
> = D.union(
  D.object({
    tokenId: D.number(),
    amountToken: D.number(),
    amountCoin: D.number(),
    time: D.string().map(s => moment(s)),
    type: D.constant('normal'),
  }),
  D.object({
    amountCoin: D.number(),
    time: D.string().map(s => moment(s)),
    type: D.constant('initialSupply'),
  }),
);
