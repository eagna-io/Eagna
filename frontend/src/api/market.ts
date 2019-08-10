import moment, {Moment} from 'moment';
import * as D from '@mojotech/json-type-validation';

import {request, Method, Failure, FailureCode} from 'api/core';
import {
  Market,
  UpcomingMarket,
  OpenMarket,
  ClosedMarket,
  ResolvedMarket,
  MarketAttributes,
  MarketStatus,
  Token,
  Prize,
  PrizeId,
} from 'models/market';
import {Order, NormalOrder, CoinSupplyOrder, RewardOrder} from 'models/order';

/*
 * ========================
 * Get Market
 * ========================
 */

export function getMarket(id: string): Promise<Market | null> {
  return request({
    method: Method.GET,
    path: `/markets/${id}/`,
    decoder: marketDecoder,
  }).then(res => {
    if (res instanceof Failure) {
      if (res.code === FailureCode.ResourceNotFound) {
        return null;
      } else {
        throw new Error(`Unexpected failure : ${res.message}`);
      }
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
    if (res instanceof Failure) {
      throw new Error(`Unexpected failure : ${res.message}`);
    } else {
      return res;
    }
  });
}

export function getMyMarkets(
  accessToken: string,
): Promise<Market[] | 'Unauthorized'> {
  return request({
    method: Method.GET,
    path: '/markets/',
    params: {
      participated: true,
    },
    accessToken: accessToken,
    decoder: D.array(marketDecoder),
  }).then(res => {
    if (res instanceof Failure) {
      if (res.code === FailureCode.Unauthorized) {
        return 'Unauthorized';
      } else {
        throw new Error(`Unexpected failure : ${res.message}`);
      }
    } else {
      return res;
    }
  });
}

export const marketDecoder: D.Decoder<Market> = D.object({
  id: D.string(),
  title: D.string(),
  organizerId: D.string(),
  description: D.string(),
  open: D.string().map(s => moment(s)),
  close: D.string().map(s => moment(s)),
  lmsrB: D.number(),
  status: D.string().map(str2status),
  resolvedTokenName: D.optional(D.string()),
  tokenDistribution: D.dict(D.number()).map(dic => Object.values(dic)),
  tokens: D.array(
    D.object({
      name: D.string(),
      description: D.string(),
      sumbnailUrl: D.string(),
    }).map(t => new Token(t.name, t.description, t.sumbnailUrl)),
  ),
  prizes: D.array(
    D.object({
      id: D.number().map(id => new PrizeId(id)),
      name: D.string(),
      sumbnailUrl: D.string(),
      target: D.string(),
    }).map(p => new Prize(p.id, p.name, p.target, p.sumbnailUrl)),
  ),
}).map(m => {
  const attrs = new MarketAttributes(
    m.title,
    m.organizerId,
    m.description,
    m.open,
    m.close,
    m.lmsrB,
    m.tokens,
    m.prizes,
  );
  if (m.status === 'Upcoming') {
    return new UpcomingMarket(m.id, attrs);
  } else if (m.status === 'Open') {
    return new OpenMarket(m.id, attrs, m.tokenDistribution);
  } else if (m.status === 'Closed') {
    return new ClosedMarket(m.id, attrs, m.tokenDistribution);
  } else {
    const resolvedTokenName = m.resolvedTokenName;
    if (resolvedTokenName === undefined) {
      throw new Error(
        'Market status is "resolved" but resolved_token_name is missing',
      );
    }
    return new ResolvedMarket(
      m.id,
      attrs,
      m.tokenDistribution,
      resolvedTokenName,
    );
  }
});

function str2status(s: string): MarketStatus {
  switch (s) {
    case 'Upcoming':
    case 'Open':
    case 'Closed':
    case 'Resolved':
      return s;
    default:
      throw new Error(`Invalid market status : ${s}`);
  }
}

/*
 * ==================
 * Post Market
 * =================
 */

interface PostMarketArgs {
  title: string;
  organizerId: string;
  description: string;
  lmsrB: number;
  open: Moment;
  close: Moment;
  tokens: {
    name: string;
    description: string;
    sumbnailUrl: string;
  }[];
  prizes: {
    id: number;
    name: string;
    sumbnailUrl: string;
    target: string;
  }[];
}

export function postMarket(
  market: PostMarketArgs,
  accessToken: string,
): Promise<string | 'Unauthorized'> {
  return request({
    method: Method.POST,
    path: '/markets/',
    accessToken: accessToken,
    body: market,
    decoder: D.string(),
  }).then(res => {
    if (res instanceof Failure) {
      if (res.code === FailureCode.Unauthorized) {
        return 'Unauthorized';
      } else {
        throw new Error(`Unexpected failure : ${res.message}`);
      }
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

export function resolveMarket(
  marketId: string,
  resolvedTokenName: string,
  accessToken: string,
): Promise<string | 'Unauthorized'> {
  return request({
    method: Method.PUT,
    path: `/markets/${marketId}/`,
    accessToken: accessToken,
    body: {
      status: 'Resolved',
      resolvedTokenName: resolvedTokenName,
    },
    decoder: D.string(),
  }).then(res => {
    if (res instanceof Failure) {
      if (res.code === FailureCode.Unauthorized) {
        return 'Unauthorized';
      } else {
        throw new Error(`Unexpected failure : ${res.message}`);
      }
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

export function getOrders(marketId: string): Promise<NormalOrder[]> {
  return request({
    method: Method.GET,
    path: `/markets/${marketId}/orders/`,
    decoder: D.object({
      orders: D.array(normalOrderDecoder),
    }),
  }).then(res => {
    if (res instanceof Failure) {
      throw new Error(`Unexpected failure : ${res.message}`);
    } else {
      return res.orders;
    }
  });
}

export function getMyOrders(
  marketId: string,
  accessToken: string,
): Promise<Order[] | 'Unauthorized'> {
  return request({
    method: Method.GET,
    path: `/markets/${marketId}/orders/`,
    params: {
      mine: true,
    },
    accessToken: accessToken,
    decoder: D.object({
      orders: D.array(
        D.union(normalOrderDecoder, coinSupplyOrderDecoder, settleOrderDecoder),
      ),
    }),
  }).then(res => {
    if (res instanceof Failure) {
      if (res.code === FailureCode.Unauthorized) {
        return 'Unauthorized';
      } else {
        throw new Error(`Unexpected failure : ${res.message}`);
      }
    } else {
      return res.orders;
    }
  });
}

const normalOrderDecoder: D.Decoder<NormalOrder> = D.object({
  tokenName: D.string(),
  amountToken: D.number(),
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
  type: D.constant('Normal'),
}).map(
  obj =>
    new NormalOrder(obj.tokenName, obj.amountToken, obj.amountCoin, obj.time),
);

const coinSupplyOrderDecoder: D.Decoder<CoinSupplyOrder> = D.object({
  amountToken: D.number(),
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
  type: D.constant('CoinSupply'),
}).map(obj => new CoinSupplyOrder(obj.amountCoin, obj.time));

const settleOrderDecoder: D.Decoder<RewardOrder> = D.object({
  tokenName: D.string(),
  amountCoin: D.number(),
  time: D.string().map(s => moment(s)),
  type: D.constant('Reward'),
}).map(obj => new RewardOrder(obj.tokenName, obj.amountCoin, obj.time));

/*
 * ===================
 * Create Initial Supply Order
 * ===================
 */

export function createInitialSupplyOrder(
  marketId: string,
  accessToken: string,
): Promise<CoinSupplyOrder | 'Unauthorized'> {
  return request({
    method: Method.POST,
    path: `/markets/${marketId}/orders/`,
    accessToken: accessToken,
    body: {
      amountToken: 0, // Dont care
      amountCoin: 10000, // Dont care
      time: moment(), // Dont care
      type: 'CoinSupply',
    },
    decoder: coinSupplyOrderDecoder,
  }).then(res => {
    if (res instanceof Failure) {
      if (res.code === FailureCode.Unauthorized) {
        return 'Unauthorized';
      } else {
        throw new Error(`Unexpected failure : ${res.message}`);
      }
    } else {
      return res;
    }
  });
}

/*
 * ===================
 * Create Normal Order
 * ===================
 */

export function createNormalOrder(
  marketId: string,
  accessToken: string,
  order: NormalOrder,
): Promise<NormalOrder | 'PriceSlip' | 'Unauthorized'> {
  return request({
    method: Method.POST,
    path: `/markets/${marketId}/orders/`,
    accessToken: accessToken,
    body: {
      tokenName: order.tokenName,
      amountToken: order.amountToken,
      amountCoin: order.amountCoin,
      time: order.time,
      type: 'Normal',
    },
    decoder: normalOrderDecoder,
  }).then(res => {
    if (res instanceof Failure) {
      if (res.code === FailureCode.InvalidPayload) {
        // 他の要素は適切（なはず）なので、ここでのエラーは価格スリップエラー
        return 'PriceSlip';
      } else if (res.code === FailureCode.Unauthorized) {
        return 'Unauthorized';
      } else {
        throw new Error(`Unexpected failure : ${res.message}`);
      }
    } else {
      return res;
    }
  });
}
